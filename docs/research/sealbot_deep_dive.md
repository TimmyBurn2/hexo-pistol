# Sealbot deep dive — claims register (ramora0/sealbot, full-history clone)

Status: LEAD SHEET, not a design source. Binding rule (ADR'd the day this
landed): nothing in this document may drive a pistol code change until its
claim is verified by the entry's proposed method; verification results amend
the register entry by entry. The operator's calibration stands (D-197,
sealbot_notes.md): sealbot is strong, strong humans exploit it; every fact
below is a lead.

Provenance: clone of https://github.com/ramora0/sealbot taken 2026-08-19,
full history, all branches, no --depth. Branches: `master` (tip c94749c,
16 commits), `origin/nnue-eval` (tip 6892e5e, 55 commits past master; the
research log commit 4098d22 is its parent), `origin/mixnet-repro` (tip
ef40a22, forks from nnue-eval's tip, 93 commits past master). No tags. The
clone lives in the session workbench, never in this repo; every citation is
checkable by `git show <sha>:<path>` (line numbers against that blob) in any
fresh clone. Prior read of master/current: docs/research/sealbot_notes.md.

Key context identifications used throughout: **strix** = hexo-strix, a GNN
AlphaZero engine (checkpoint_00237000, Gumbel MCTS on a V100), sealbot's
external benchmark opponent (`4098d22:experiments/strix/LOG.md:3-12`);
**"e1" gate** = 100 games vs strix sims=64 on paired dev openings 0-49 at
tl=0.44 s/turn, any positive delta vs control adopts
(`ef40a22:autoresearch/gate.py:12-18,58-65`); **"N/150"** = the honest
paired human-openings bench (75 openings x both colors,
`4098d22:experiments/strix/EVAL_SCHEME.md:48-51`). sealbot's Elo numbers
below are all RELATIVE TO STRIX, a machine opponent — per D-197 none of
this licenses any human-strength claim.

Entry format: {claim, evidence, why it matters for pistol, status, proposed
verification}. Status is UNVERIFIED for every entry at landing; a verified
entry is amended in place to VERIFIED/REFUTED with the measurement's
citation. A claim without a checkable citation was not recorded.

## A. Search features and the experiment ledger (branches, history)

Scale note: `git diff --stat master..origin/nnue-eval` = 825 files /
~12.3k insertions (NNUE/trunk eval pipeline, VCF solver, SMP rewrite, strix
bench harness); `master..origin/mixnet-repro` = 915 files / ~30.1k
insertions (adds an `autoresearch/` gate harness and the Mixnet builds).
The branches are a pre-run traversal of pistol Stages 1-4 on the identical
game; the DISCARD rows are as load-bearing as the ADOPT rows.

### SB-01 — hitting-set VCF solver proves deep kills in ~40 us median

- CLAIM: sealbot's threat-space solver `forced_win()` — iterative deepening
  over only four/five-threat-creating attacker turns, defender replies = all
  2-stone covers of the threat-window hitting set; min hitting set >= 3
  proves a win, == 2 recurses over every cover, <= 1 returns unknown
  (conservative) — proved wins on 79/500 real positions at max_turns=8 with
  median 38 us, mean 0.6 ms, max 10.1 ms per call; all 79 claims survived
  adversarial playout (0 failures).
- EVIDENCE: nnue-eval commit 01891ee (message carries the numbers); code at
  `01891ee:current/engine/vcf.h`.
- WHY IT MATTERS: a working, measured design for pistol's threat solver
  (WP-1.5 threat gen -> WP-1.8/Stage-3 TSS): the hitting-set-over-windows
  formulation with conservative unknowns is portable, and ~40 us median is
  cheap enough to call inside search.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: implement hitting-set threat proof in
  pistol-solver; validate by the same protocol (win-in-N suite +
  adversarial playout of every claimed win); measure per-call latency.

### SB-02 — the VCF stack alone was worth ~150 Elo before any eval work

- CLAIM: the honest 150-game ladder vs strix went champion 10/150 (-451) ->
  +VCF root attack k=8: 16/150 (-365) -> +post-search deep veto: 22/150
  (-303) -> trunk3(K=64)+full VCF: 43/150 (-157).
- EVIDENCE: `4098d22:experiments/strix/LOG.md:364-370` ("Beat-strix
  campaign ladder"); commit d6fc41e.
- WHY IT MATTERS: quantifies Stage-3 payoff — bolting a proof solver onto
  alpha-beta at the root (play proven wins, veto proven losses) moved
  ~90-150 Elo before eval improved at all.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: SPRT pistol+root-threat-probe vs baseline over
  paired balanced openings at fixed nodes, once WP-1.5/1.8 land.

### SB-03 — post-search deep veto: never move into a provable forced loss

- CLAIM: analysis found 127/127 recorded losses ended in proven strix
  forcing wins and 114 entered provably-lost territory >= 4 turns early;
  the fix reserves a slice of the clock to probe the chosen move deeper
  (initially k-2, later a tier probing at k+3 with doubled budget) and walk
  down the root ordering (<= 5 probes) if the choice proves lost.
- EVIDENCE: nnue-eval commit messages eb0bcc0 and a78b9a8; the 0.82/0.16
  main/veto time split at `6892e5e:current/engine/search.h:143-145,404-406`
  and `ef40a22:autoresearch/ideas.md:44-49`.
- WHY IT MATTERS: a concrete Stage-3 architecture — proof search as a
  root-level veto layer with a pre-measured clock split — and a direct
  answer to the loss mode pistol will share (tactical horizon).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: classify pistol's arena losses by "was the played
  move already provably lost N turns earlier" using pistol's own solver;
  if the fraction is high, SPRT a veto layer at ~15% of movetime.

### SB-04 — VCF knobs closed at K=11, budget 25000, interior probes OFF

- CLAIM: under the mixnet champion the VCF levers closed at SEAL_VCF_K=11
  (mx64_vcfk11 43/100, +1, ADOPT), budget 25000 (40k: +0; 15k: -1 — flat
  both directions), and interior probes off (vcf11_nointerior 36/100 +1
  ADOPT; mx64_vcf15 +0 DISCARD); deeper proof (K=13/60k) lost 11 points
  (23/100, z -1.72, "starves main search").
- EVIDENCE: `ef40a22:autoresearch/results.tsv` rows vcf_k13_b60k,
  vcf_k9_b25k, vcf11_nointerior, mx64_vcfb40k, mx64_vcfb15k, mx64_vcfk11,
  mx64_vcf15.
- WHY IT MATTERS: Stage-3 tuning prior — proof depth/budget has a sharp
  optimum well below "as deep as affordable", and in-tree probes bought
  nothing in two independent sweeps; suspect until re-derived (their gate
  is 100 games, noisy).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: sweep pistol's threat-search depth/node budget at
  fixed movetime by SPRT; pre-register the expectation of an interior
  optimum and of interior-probe futility.

### SB-05 — losses are proof-budget-constrained, not knowledge-constrained

- CLAIM: at the last avoidable decision in 40 recorded losses, 33/40 were
  CONSTRAINT (a generous engine at tl 2.0, k=16, 200k nodes finds a save),
  only 6/40 eval-preference and 1/40 dead-early; the saving cells ranked
  median #2 in the engine's own ordering.
- EVIDENCE: `4098d22:experiments/strix/LOG.md:456-468` ("Blunder
  autopsy"); commit b95c1a2.
- WHY IT MATTERS: prioritization signal for Stages 1-3 — once ordering is
  decent, marginal Elo is in proof depth/time at critical moments, not in
  candidate generation; motivates WP-1.5/1.8 over further ordering work.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: reproduce the autopsy on pistol's SPRT losses:
  classify each loss by whether a 4x-budget pistol avoids it.

### SB-06 — root candidate cap: 20 measured optimal, 26 a blowout

- CLAIM: SEAL_ROOT_CAP=26 scored 22/100 (-13, z -2.04, DISCARD, "width not
  free at T=16") and cap 16 scored 33/100 (-2, ns), closing the root-width
  lever at 20.
- EVIDENCE: `ef40a22:autoresearch/results.tsv` rows rootcap26, rootcap16;
  commits 467dd55, 7ab1ea8.
- WHY IT MATTERS: numeric prior for pistol's candidate-policy config
  (Stage 1): root width has a measured optimum near 20 in this exact game —
  suspect (single 100-game gate, their eval, their ordering).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: SPRT pistol root-candidate-cap {16, 20, 26} at
  fixed movetime once threat-first generation (WP-1.5) is the source.

### SB-07 — TT sized to L3: auto 2^22 optimal, 2^24 a cache blowout

- CLAIM: SEAL_TT_BITS=24 (400 MB) scored 25/100 (-10, "blows V-cache,
  RAM-latency bound") and 2^21 scored 30/100 (-5, "capacity loss offsets
  latency win"), closing the TT lever at auto 2^22 on their 96 MB-L3 box.
- EVIDENCE: `ef40a22:autoresearch/results.tsv` rows tt24, tt21; commits
  928b0c1, c14db2e.
- WHY IT MATTERS: Stage-4 tuning lead — fit the TT to L3 rather than
  maximizing it; the optimum is hardware-specific.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol bench of time-to-depth across TT sizes
  bracketing the deployment workstation's L3; SPRT the best two.

### SB-08 — lazy SMP bought zero Elo; root-split YBW +90 at T=20

- CLAIM: at T=20 vs strix s64, legacy lazy SMP scored 25/100, ABDADA
  31/100, root-split YBW 35/100 (baseline 20/100 single-thread, z ~2.7);
  stated law: on ~13k-node trees, linear NPS and non-redundant parallel
  work are mutually exclusive — lazy = linear thread-nodes / zero Elo,
  root-split = 2.2x work-nps / +0.4 ply / +90 Elo, serial PV child capping
  root-split at ~2.5x (Amdahl).
- EVIDENCE: nnue-eval commit 8e8c438 (message);
  `4098d22:experiments/strix/LOG.md:497-523`.
- WHY IT MATTERS: challenges pistol's Stage-4 "Lazy SMP" plan head-on for
  this game's small forced trees; the roadmap's ABDADA fallback also lost
  on both their machines (SB-09).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: at Stage 4, A/B root-split vs lazy SMP at fixed
  wall-clock over 100+ paired games, reporting work-nps and depth (never
  thread-nodes).

### SB-09 — ABDADA lost on both machines tried

- CLAIM: SEAL_SMP_MODE=3 (ABDADA) scored 30/100 (-5) on the 8-core box,
  confirming the T=20 cluster result (31 vs root-split's 35/100).
- EVIDENCE: `ef40a22:autoresearch/results.tsv` row smp3; commit 85a1b62.
- WHY IT MATTERS: demotes ABDADA on pistol's Stage-4 shortlist, with the
  caveat their writeup notes it scales past ~20 root moves.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: re-test only if pistol's root move count after
  capping regularly exceeds ~20.

### SB-10 — time-scaling: ~+41 Elo per clock doubling for alpha-beta here

- CLAIM: on a 9-cell tl x sims grid with paired openings, sealbot scales
  -82/0/+82 Elo at tl 0.11/0.44/1.76 s (log-linear, ~+41 Elo/doubling
  across 16x), while strix is flat sims 4->16 and jumps +240 from 16->64;
  matching strix s64 extrapolates to ~35x time odds.
- EVIDENCE: `4098d22:experiments/strix/LOG.md:471-495` ("Time-scaling
  map"); commit eb893f9.
- WHY IT MATTERS: calibrates pistol's 0.5 s design point against an
  AlphaZero opponent on the same game and gives the expected Elo-per-
  doubling slope for budget and SPRT design (Stage 5, mantis comparison).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: measure pistol's own Elo-vs-clock slope over a
  3-point movetime grid with paired openings; compare against ~40
  Elo/doubling.

### SB-11 — self-play h2h is an invalid gate: +160 h2h mapped to 1/149

- CLAIM: sealbot's measurement law #1 bans seal-vs-seal head-to-head as a
  gate (a +160 h2h build scored 1/149 vs strix) and law #2 bans offline
  metrics; play vs the external opponent is the only gate, and old
  pre-NNUE seal builds are not a valid opponent (same family + saturation).
- EVIDENCE: `4098d22:experiments/strix/EVAL_SCHEME.md:54-56`.
- WHY IT MATTERS: stresses pistol's Hard Rule 6 harness design (Stage 5):
  self-play SPRT can adopt opponent-specific changes; an external frozen
  reference opponent matters.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: when pistol has two generations, cross-check a
  self-play-SPRT-adopted change against a frozen third engine (sealbot
  itself, post threat-core per the deferred-match note).

### SB-12 — non-transitivity observed directly among engine generations

- CLAIM: distill beat champion 91-8 (+413) and champion beat original 87%
  (+338), but distill beat original only 79.5% — an explicitly recorded
  non-transitive triangle.
- EVIDENCE: `4098d22:experiments/strix/LOG.md:96-99` ("Non-transitive
  triangle"); commit 1183b0f.
- WHY IT MATTERS: same-game evidence for pooled multi-opponent gating in
  pistol's Stage-5 arena rather than a single Elo ladder.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: round-robin three pistol generations when they
  exist; test transitivity of paired-opening results.

### SB-13 — hybrid net+linear blend: strong, but a cliff-edged optimum

- CLAIM: the deep-label NNUE at blend 0.15 gated +338/200g (87.5%) vs the
  original engine, but the blend optimum is razor-thin — 0.10 -> ~0,
  0.20 -> +21, 0.25 -> -576 — because the legacy table's ~43% wrong signs
  on decided positions outvote the net above ~0.2.
- EVIDENCE: `4098d22:experiments/nnue/LOG.md:57-68`;
  `4098d22:memory/nnue-eval-branch-status.md` ("Blend is a SHARP optimum").
- WHY IT MATTERS: Stage-2 lead — if pistol blends learned and handcrafted
  eval, the mixing weight is a cliff-edged hyperparameter to sweep finely,
  never guess.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: when pistol's codebook eval lands, sweep
  blend-with-handcrafted in <= 0.05 steps under SPRT before adopting.

### SB-14 — label space beats architecture: WDL saturates, score-Huber wins

- CLAIM: WDL-sigmoid-target nets lost -382/-449 Elo ("flat/saturated
  logits", "net learns sign only") while Huber regression on deep (tl 0.12)
  search scores produced the +338 champion.
- EVIDENCE: `4098d22:experiments/nnue/LOG.md:38-48,57-62`; restated in
  `4098d22:memory/nnue-eval-branch-status.md`.
- WHY IT MATTERS: Stage-2 training-recipe prior — start with score-space
  regression on deep labels, not WDL, for this game's label distribution.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: train two identical pistol eval nets differing
  only in target space on the same deep-labeled corpus; gate both.

### SB-15 — oracle policy: gold at the root (+402), poison in the tree (-512)

- CLAIM: strix-distilled policy tables used for root ordering only gated
  +402 (91.0%, n=300) over delta ordering, while the same tables at
  opponent interior nodes gated -512.
- EVIDENCE: `4098d22:experiments/strix/LOG.md:114-137,144-160`; commits
  661a68a, 5f71a6f.
- WHY IT MATTERS: ordering architecture for Stage 1/2 — an external policy
  belongs at root choice; interior alpha-beta ordering must stay consistent
  with the engine's own eval derivative.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: if pistol adds a policy ordering source, ablate
  root-only vs everywhere vs opponent-nodes-only at fixed depth.

### SB-16 — the -512 mechanism: pair-index capping wedges out forced defenses

- CLAIM: the corrected root cause of the -512 is that interior pair
  generation only emits index pairs with i+j <= PAIR_SUM_CAP (14) — 56 of
  105 pairs — so a mid-ranked double-block defense (e.g. indices 6+9, sum
  15) is unrepresentable; threat-first partition recovered -512 -> -104,
  and opening the wedge (SUM_CAP 28) made policy-interior ~neutral (-53)
  but cost ~200 Elo of breadth in the default config.
- EVIDENCE: `4098d22:experiments/strix/LOG.md:170-196` ("-512 mechanism
  CORRECTED"); commit bc37088; wedge code at master
  `current/engine/tables.h:60-62` (i+j <= PAIR_SUM_CAP, 56 pairs) used at
  `current/engine/search.h:584-589`; `ef40a22:autoresearch/ideas.md` knob
  inventory calls it "wedge — measured load-bearing, don't touch".
- WHY IT MATTERS: critical for pistol's pair movegen and WP-1.5: any pair
  cap must guarantee forced defensive pairs stay representable — cap by
  threat class, never by index-sum alone. This is the measured cost of
  sealbot's own cap design, and the mechanism behind its known
  double-threat blind spots.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol test asserting that in positions with a
  must-block double threat, the blocking pair survives every candidate/
  pair cap; measure the Elo of threat-first partition in ordering.

### SB-17 — contrastive sibling loss cut decision regret 5x vs pointwise

- CLAIM: trunk v1.1's |dOracle|-weighted pairwise logistic sibling loss
  (plus a policy head taking (move_count, moves_left) and 580k human
  positions) took decision regret on the REAL set from .175 to .052 vs
  linear .277, because "pointwise Huber never trains sibling contrasts".
- EVIDENCE: `4098d22:experiments/strix/LOG.md:319-334,246-253`; commits
  e1076bc, a9b8ea8.
- WHY IT MATTERS: Stage-2 loss design — gate eval nets on sibling-
  discrimination/decision-regret, and include a contrastive term; absolute
  value fidelity does not order moves.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: build a small child-value oracle set for pistol;
  compare regret of pointwise vs contrastive training before any play gate.

### SB-18 — offline wins did not convert: trunk v1.2 lost 1-149

- CLAIM: the offline-superior trunk build lost 1-149 vs strix (gap -799);
  the POV hypothesis was rejected by measurement, and the loss decomposed
  into a never-parity-tested in-engine policy head (~230 Elo, recovered by
  the fix, 1 -> 5/150) plus a residual attributed to ordering x eval
  self-consistency (mixed value/policy pairs measurably disagreed: tv+tp
  +45 vs tv+lp -258 h2h).
- EVIDENCE: commits 73a9901, 6d43b51, 86d3c42;
  `4098d22:experiments/strix/LOG.md:336-363`.
- WHY IT MATTERS: two Stage-2 process leads — every ported net component
  needs an engine-vs-trainer parity test before any strength claim, and
  value+ordering may need to be co-trained.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: make per-head parity tests a landing gate for
  pistol's Stage-2 eval port; ablate co-trained vs mixed pairs at fixed
  depth.

### SB-19 — label shaping was the campaign's single largest jump

- CLAIM: stamping VCF-proven-win training targets with a HARD 8.0
  saturation regressed (34 vs 45/150; v1.4 reverted to v1.3), while a SOFT
  floor t=max(t, 6.5) on the same ~18% of positions produced v1.5 at
  63/150 — logged as "the largest single jump of the campaign... it came
  from LABELS, not architecture".
- EVIDENCE: `4098d22:experiments/strix/LOG.md:379-397`; commits 42ae3e7,
  53f3849, 6c21908 ("Restore v1.3 weights"), 111a74b, dd74d9e.
- WHY IT MATTERS: prime Stage-2 x Stage-3 synergy — use pistol's threat
  solver to stamp soft proven-win floors on eval training labels; the hard
  version is a measured regression.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: A/B three label schemes (raw, hard-saturate,
  soft-floor) with identical data under SPRT once pistol trains an eval.

### SB-20 — ordering was already at the Knuth-Moore floor; horizon must
### come from proof search

- CLAIM: a fixed-depth probe showed trunk-policy ordering reaches d=4 in
  87k nodes vs legacy 100k (-14%) with EBF 10.7 ~ sqrt(branching) — at the
  perfect-ordering floor, worth ~0.06 ply — so remaining horizon must come
  from VCF and leaf foresight, not better ordering.
- EVIDENCE: `4098d22:experiments/strix/LOG.md:426-431` ("Depth probe").
- WHY IT MATTERS: sets an expectation ceiling for pistol's WP-1.7 ordering
  work and justifies the report's Stage-3 emphasis.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: measure pistol's EBF vs sqrt(pair-branching) at
  fixed depth after Stage 1; stop ordering investment within ~10% of floor.

### SB-21 — Mixnet (Rapfi repro) ladder: adopted at C64, size closed by L3

- CLAIM: the Rapfi Mixnet reproduction gated +2 on first attempt (mixnet1m
  38/100), +4 at C64 (mixnet2_c64 42/100, transfer-verified +4 on held-out
  openings), and closed size at C64 because C128's ~90 MB table busts the
  96 MB L3 (mixnet3_c128 37/100, -5); final champion mx64_vcfk11 43/100
  (Elo -49 vs strix).
- EVIDENCE: `ef40a22:autoresearch/results.tsv` rows mixnet1m, mixnet2_c64,
  mixnet2_c64_transfer, mixnet3_c128, mx64_vcfk11; architecture at
  `ef40a22:experiments/strix/MIXNET_DESIGN.md:10-41`; scaling battery at
  `ef40a22:experiments/strix/SCALING.md:9-41`.
- WHY IT MATTERS: this IS pistol's Stage-2 incremental codebook net,
  already built for hex Connect(6,2,1) — with cache residency as the
  binding deployment constraint and "mapping width free, codebook width
  bound" as the scaling law.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: size pistol's codebook to fit the target CPU's L3
  from day one; replicate the M-free/C-bound scaling battery offline
  before any engine port.

### SB-22 — int16 quantization closed: +3% NPS, ~7 points of quant noise

- CLAIM: quantization gave only +3% NPS (float table already L3-resident;
  int16 does not beat float on a 7-tap depthwise kernel) while |dv|~120
  value noise through the unnormalized star head cost ~7 points
  (mixnet4_c128q 30/100 vs float C128's 37, z -1.91); the postmortem
  prescribes per-channel codebook scales (~5x less error) or a normalized
  value input if revisited.
- EVIDENCE: `ef40a22:autoresearch/ideas.md:88-104` (graveyard);
  `ef40a22:autoresearch/results.tsv` row mixnet4_c128q; commits 66321b4,
  9fcb157.
- WHY IT MATTERS: saves a Stage-2 dead end — the Stockfish int16-NNUE
  speedup is priced for memory-bound tables and does not transfer when the
  table is already cache-resident.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: before quantizing pistol's eval, bench whether the
  table is L3-resident; if quantizing anyway, per-channel scales + a
  parity-tolerance harness.

### SB-23 — shallow self-play labels actively hurt (-12)

- CLAIM: retraining on own-search self-play labels generated at tl
  0.08-0.16 (depth-1 quiet labels) gated 31/100 (-12, z -1.76, DISCARD),
  reproducing an earlier gen-1 flatline; the surviving hypothesis is
  deeper labels (tl .25-.45).
- EVIDENCE: `ef40a22:autoresearch/results.tsv` row mixnet5_gen2a; commit
  ef40a22 (message); pipeline spec `ef40a22:experiments/nnue/GEN2_CLUSTER.md:1-38`.
- WHY IT MATTERS: Stage-2 data prior — self-improvement loops need teacher
  depth above a threshold; shallow self-play labels are worse than nothing
  on this game.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: if pistol runs a self-labeling loop, A/B label
  depth (shallow vs ~3x deeper) with everything else fixed first.

### SB-24 — even a strong AlphaZero teacher cannot "see" danger early

- CLAIM: strix's single forward value has sign-agreement only 0.63 vs deep
  search labels (0.535 early-game) yet beats the engine hybrid at
  predicting outcomes (0.673 vs 0.601), and reads provably-lost entries at
  only +0.78 (own POV) / 5-turns-early positions at +0.25 — "perfect value
  mimicry alone does not close the gap".
- EVIDENCE: `4098d22:experiments/strix/LOG.md:47-63,433-455` ("Danger
  probe"); commit 65d30d7.
- WHY IT MATTERS: bounds what pistol's Stage-2 net can deliver even with a
  perfect teacher — tactical horizon must come from Stage-1/3 search,
  which is the minimax report's founding premise, here measured.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: when mantis nets are available as teachers, run
  the same sign-vs-deep-label and danger-probe splits before committing to
  a distillation-heavy Stage-2 plan.

### SB-25 — the graveyard: the full DISCARD ledger

- CLAIM: the abandoned approaches across branches are: pure-net eval
  (-382/-449), WDL targets, gen-1 self-play retrain (flat), policy-
  everywhere / policy-interior ordering (-81/-512), trunk v1.2 play build
  (1-149), v1.4 hard-saturated labels, legacy lazy SMP (zero Elo), ABDADA,
  TT 2^21/2^24, root caps 16/26, defense-filter strength both directions,
  VCF K=13/60k, veto slice 16%->10% (2660fb8: "veto earns its clock") and
  veto tier k+6 (03945f7), policy mode 75 (b432f4f: "interior stays
  delta-ordered"), C128 and int16 mixnets, and shallow gen-2 labels.
- EVIDENCE: the DISCARD rows of `ef40a22:autoresearch/results.tsv`;
  graveyard section `ef40a22:autoresearch/ideas.md:86-110`; commit
  messages 6c21908, 73a9901, 8e8c438, d0fe63b.
- WHY IT MATTERS: a pre-paid map of Stage-1..4 dead ends on pistol's exact
  game; each row is a prior with a known expected sign, to be re-opened
  only with a stated reason the context differs.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: import the DISCARD table into pistol's experiment
  backlog as pre-registered priors at each stage gate.

### SB-26 — the champions' exact configurations (match-pinning data)

- CLAIM: nnue-eval's final champion is trunk v1.5 with env
  `SEAL_EVAL=trunk SEAL_TRUNK_POLICY=1 SEAL_TRUNK_BLEND=0
  SEAL_POLICY_MODE=74 SEAL_VCF=15 SEAL_VCF_K=11 SEAL_VCF_BUDGET=40000`,
  SMP mode 2, T=20 (63/150 empty-board / 42/150 paired-openings vs strix
  s64); mixnet-repro's is cand_mixnet64 with SEAL_VCF=11, K=11,
  BUDGET=25000 (43/100, Elo -49).
- EVIDENCE: `4098d22:experiments/strix/EVAL_SCHEME.md:42-47`;
  `ef40a22:autoresearch/gate.py:66-75` (CHAMPION_ENV);
  `ef40a22:autoresearch/ideas.md:8-15`.
- WHY IT MATTERS: when the deferred pistol-vs-sealbot match happens
  (post threat core), the opponent must be pinned to one of these exact
  envs in the match manifest to be reproducible.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: build the pinned env from the clone and record
  its self-identification (weights sha, env) in the match manifest.

## B. Time management, platform behavior, opening handling

(Section letter B here is document order only; investigation area 4.)

### SB-27 — time is checked every 1024 nodes, aborted by exception

- CLAIM: `_check_time()` increments `_nodes` and only when
  `(_nodes & 1023) == 0` compares `steady_clock::now()` to `_deadline`,
  throwing a `TimeUp` exception to abort mid-search; the deadline is
  exactly `now + time_limit` with zero safety margin, set once per
  `get_move` (constructor default slice 0.05 s), and nothing reads a
  remaining game clock.
- EVIDENCE: master c94749c, `current/engine/bot.h:208-212` (check),
  `current/engine/search.h:41-42` (deadline), `current/engine/bot.h:32`
  (default 0.05).
- WHY IT MATTERS: pistol's `movetime_ms` play mode (WP-1.4 territory)
  needs a check-interval constant with a known worst-case overshoot of
  interval x ns/node; 1024 is the incumbent's calibration point, and
  fixed-slice-only management is evidently sufficient at sealbot's level.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: instrument pistol's play-mode search with a
  1024-node check interval; measure the overshoot distribution at movetime
  50/100/500 ms over 200 midgame positions.

### SB-28 — real overshoot bound: <= one check interval of search, but an
### UNTIMED pre-loop setup

- CLAIM: iterative deepening starts every next depth unconditionally
  (`for depth = 1..max_depth`, default 200) with no will-it-fit
  prediction — the only stops are a proven win or `TimeUp` — so mid-search
  overshoot is bounded by ~1024 nodes of work; but the pre-loop setup
  (full window/candidate init scans plus the SavedArrays snapshot) runs
  before any deadline check and is untimed.
- EVIDENCE: master c94749c, `current/engine/search.h:157-200` (loop; win
  break at 178, TimeUp catch at 179), `current/engine/search.h:17-127`
  (untimed setup), `current/engine/bot.h:29` (max_depth 200).
- WHY IT MATTERS: exception-abort gives tight overshoot without per-
  iteration prediction — a candidate design for pistol's WP-1.4 play mode,
  provided setup cost is bounded; the untimed setup is also the reason
  sealbot's real overshoot grows with position size.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: drive sealbot's `current/` build at
  time_limit=0.1 over 500 positions of increasing stone count; record the
  `elapsed - 0.1` distribution and correlate the tail with stone count.

### SB-29 — abort rollback is a full-array memcpy snapshot (~400 KB/move)

- CLAIM: `TimeUp` rollback restores a `SavedArrays` snapshot taken once
  before the ID loop — board (140x140 int8), per-axis window counts
  (3x140x140 pair<int8,int8>), eval window patterns (3x140x140 int),
  candidate refcounts, hot/candidate sets, plus scalars — while TT,
  history, and killers are deliberately not rolled back.
- EVIDENCE: master c94749c, `current/engine/search.h:139-155` (save),
  `:179-199` (restore), `current/engine/bot.h:192-205` (SavedArrays).
- WHY IT MATTERS: pistol's incremental apply/undo contract avoids this
  ~2x400 KB per-move structural cost entirely — a known floor pistol can
  beat, and a reason not to copy sealbot's abort design wholesale.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: microbench one SavedArrays save+restore cycle vs
  pistol's undo-stack unwind for a depth-6 abort.

### SB-30 — sealbot's published strength was measured under a 3x time grace

- CLAIM: the evaluation harness tolerates overshoot: `GRACE_FACTOR = 3.0`
  and `MAX_VIOLATIONS_PER_GAME = 10` — a turn is a violation only past 3x
  the limit and a bot forfeits only after 10 violations in one game.
- EVIDENCE: master c94749c, `evaluate.py:29-30,174-178`.
- WHY IT MATTERS: any pistol-vs-sealbot comparison (Stage-5 arena /
  hexo-bridge match) must state the grace policy; pistol's per-side
  compute accounting (Hard Rule 6) should replicate then tighten it.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: rerun sealbot self-play with GRACE_FACTOR=1.05
  and count violations at time_limit 0.05/0.1/0.5.

### SB-31 — they shipped (and fixed) the exact per-stone-vs-per-turn
### budget bug pistol's TURNS pin exists to prevent

- CLAIM: before commit 2a99edd ("Fix timing issue", 2026-03-28) the
  allowance was `deadline = now + time_limit * moves_left_in_turn` and the
  violation check scaled the same way — a 2-stone turn got a 2x budget;
  the fix made the budget strictly per TURN and relabeled the report
  "avg move time" -> "avg turn time".
- EVIDENCE: `git show 2a99edd` (diff hunks on evaluate.py:79-82,165-180).
- WHY IT MATTERS: exactly the turn-vs-stone unit ambiguity pistol pins
  ("sudden death is scored in TURNS"); pistol's movetime semantics and the
  future bridge must state per-turn explicitly or the 2x bug recurs.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol-arena test asserting a 2-stone turn
  consumes one movetime slice, not two.

### SB-32 — the check interval was re-derived when node cost rose

- CLAIM: on the branches the clock-check mask tightens from 1023 to 255
  when the NNUE trunk eval is active (`int mask = _use_trunk ? 255 :
  1023;`) with the comment "Trunk nodes are ~1.5x slower; check the clock
  more often so the overshoot past the deadline stays comparable".
- EVIDENCE: origin/nnue-eval 6892e5e, `current/engine/bot.h:552-563`.
- WHY IT MATTERS: overshoot = interval x ns/node is a maintained
  invariant, not a one-off constant — when pistol swaps eval backends
  (Stage 2), its play-mode check interval must be re-derived from measured
  ns/node.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol bench recording ns/node per eval backend;
  assert check-interval x ns/node < 1% of the movetime budget.

### SB-33 — partial root-iteration harvest: an aborted iteration is not wasted

- CLAIM: on the branches the `TimeUp` handler harvests
  `_root_partial_best` — the best fully-searched root move of the aborted
  iteration supersedes the previous iteration's choice, sound because the
  previous best is always searched first (monotone information).
- EVIDENCE: origin/nnue-eval 6892e5e, `current/engine/search.h:334-342`;
  commit 0009de2.
- WHY IT MATTERS: directly applicable to pistol's WP-1.4 movetime fix —
  cheap strength from the otherwise-wasted final iteration, sound only
  with PV-first root ordering (which pistol has via the TT move).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: SPRT partial-root-harvest on/off in pistol play
  mode at movetime 100 ms, paired openings.

### SB-34 — there is NO HeXO client in the repo; the platform adapter
### lives elsewhere

- CLAIM: play.py is a local pygame GUI and the only external interface is
  the pybind11 module `minimax_cpp.MinimaxBot(time_limit)` /
  `get_move(game)`; grep for "hexo" across all files and commit messages
  of all branches returns nothing.
- EVIDENCE: master c94749c, `play.py:1-24,313,551-567`;
  `current/minimax_bot.cpp:15-60,78-103`.
- WHY IT MATTERS: sealbot's HeXO adapter (hexo-bridge's
  seal_perf_engine.py, per sealbot_notes.md) is external to this repo —
  pistol's bridge behavior must be derived from the platform spec, not
  copied from this codebase.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: reconnaissance of the hexo-bridge repo (already
  the API target per project memory); confirm where sealbot's platform
  time/format handling actually lives.

### SB-35 — silent (0,0) fallback on empty candidate/turn sets

- CLAIM: the pybind result convention returns `{0,0,0,0,1}` when the
  candidate set or turn list is empty — a pathological position makes the
  engine emit a stone at (0,0) regardless of whether (0,0) is legal or
  occupied.
- EVIDENCE: master c94749c, `current/minimax_bot.cpp:47-58`;
  `current/engine/search.h:14-15,129-135,202-203`.
- WHY IT MATTERS: the exact silent-fallback class pistol's Hard Rule 3
  forbids; also a candidate red-team fixture theme (full-neighborhood
  positions) for the eventual head-to-head.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: construct a position whose distance-2
  neighborhood is fully occupied; check sealbot returns an occupied (0,0).

### SB-36 — no opening book anywhere; turn 1 is hardcoded origin

- CLAIM: on an empty board sealbot returns a single stone at (0,0)
  (`if (gs.cells.empty()) return {0,0,0,0,1};`) and the repo contains no
  opening book, stored openings, or other early-move special case.
- EVIDENCE: master c94749c, `current/engine/search.h:13-15`;
  `current/minimax_bot.cpp:47-51`.
- WHY IT MATTERS: Stage-5 opening book is uncontested ground vs this
  opponent; also matches pistol's turn-1-origin-WLOG convention. NOTE the
  ~107 Elo empty-board memorization inflation their own protocol measured
  (SB-26/EVAL_SCHEME law #5) — opening strength claims need paired
  non-empty openings.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: grep is exhaustive already; behavioral check =
  drive `get_move` on an empty game and assert (0,0).

### SB-37 — the colony move: ONE hash-directed root-only escape candidate

- CLAIM: colony generation appends exactly one extra root candidate at
  `centroid + COLONY_D[_hash % 6] * (max_r + 3)` (centroid = integer mean
  of all stones, max_r = max hex distance of any stone from it, direction
  = one of the six axial units picked by the position hash), appended
  AFTER the ROOT_CANDIDATE_CAP=20 cut and only in root `_generate_turns()`
  — interior nodes generate no colony moves.
- EVIDENCE: master c94749c, `current/engine/movegen.h:143-161` (incl.
  `int cd = max_r + 3;`), `current/engine/tables.h:13-14`,
  `current/engine/constants.h:45`; interior gen without colony at
  `current/engine/search.h:544-592`.
- WHY IT MATTERS: the incumbent's only escape from its distance-2
  candidate shell is a single predictable (hash-directed) cell — pistol
  could trial an "escape candidate" as a config-level CandidatePolicy
  variant (Stage 1), and the predictability is itself exploitable.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: log how often sealbot's chosen turn contains the
  colony cell over a match corpus; test whether a radius-8-aware pistol
  punishes a spawned colony that sealbot's distance-2 candidates cannot
  then defend.

### SB-38 — branch-side root defense filter carries BOTH a node cap and a
### wall-clock fraction

- CLAIM: the "honest time control" fix (98bac5a) caps the root VCF
  defense-filter probes at `min(3000, max(800, budget/6))` nodes AND cuts
  the whole filter off at 40% of the move budget (`vcf_cutoff = now +
  time_limit * 400000.0 us`), after which unfiltered turns pass through to
  the search; one commit later (797732b) the cap was raised 3000->8000
  because a two-knob run was confounded ("deeper root + halved defense =
  23/150"); final form `fb = min(8000, max(800, vcf_node_budget/6))`.
- EVIDENCE: `git show 98bac5a` and `git show 797732b` (diffs on
  current/engine/search.h); final form at origin/nnue-eval 6892e5e,
  `current/engine/search.h:212-243`, `current/engine/bot.h:63-66,207`.
- WHY IT MATTERS: precedent for pistol's Stage-3 root threat filters —
  budget them in BOTH nodes and wall-clock fraction, degrade by
  pass-through rather than truncation; the confounded run is also a worked
  example of why pistol pre-registers one-knob experiments.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: when pistol adds a root defense filter,
  pre-register a 40%-of-budget cutoff arm vs unlimited; measure Elo and
  overshoot.

### SB-39 — sealbot's own referee has NO radius-8 rule and an unbounded
### sparse board

- CLAIM: game.py's `is_valid_move` is exactly "not game_over and (q,r) not
  in board" — any empty cell of the infinite grid is legal; wins are
  detected only around the just-placed stone by scanning +/-(win_length-1)
  along the 3 axes with `len(cells) >= win_length` (overlines win).
- EVIDENCE: master c94749c, `game.py:48-51` (is_valid_move), `game.py:28`
  (sparse dict), `game.py:96-118` (_check_win), `game.py:21` (3 axes).
- WHY IT MATTERS: confirms sealbot_notes.md item 1 at the code level: the
  radius-8 legality pistol pins as a game rule (D-101, platform-confirmed)
  is absent from sealbot's own referee — any match adapter must state
  whose legality governs and what happens on a rule-5-illegal move.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: already platform-resolved for HeXO by D-101; for
  a local match, red-team the adapter with a distance-9 move and assert
  refusal (pistol fail-loud) on record.

### SB-40 — engine-internal 140x140 bound is unchecked on input: legal
### positions beyond ring 70 are UB

- CLAIM: the engine's arrays cover coords [-70,69] ("padding for windows
  (+/-5) and neighbor candidates (+/-2)"); the colony generator bounds-
  checks itself (`abs(col_q) < OFF && abs(col_r) < OFF`) but board
  population from GameState does not (`_board[cell.q+OFF][cell.r+OFF] =
  cell.player`), so a legal long game reaching |coord| > 70 is
  out-of-bounds UB.
- EVIDENCE: master c94749c, `current/engine/constants.h:55-58`;
  `current/engine/movegen.h:158` (checked); `current/engine/search.h:30`
  (unchecked).
- WHY IT MATTERS: validates pistol-core's unbounded-lattice + lazy-zobrist
  choice, and is a concrete adversarial input class for any engine with
  fixed arrays (red-team theme; also an exploit avenue against sealbot in
  very wide games).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: feed sealbot a legal HexGame with stones near
  q=75 under ASan; observe crash/corruption.

## C. The CMA pipeline and the eval-generation ladder (Stage-2 method prior)

How master's 729-pattern weights were actually produced, what judged them,
and what the branches later measured about their ceiling.

### SB-41 — CMA-ES objective: raw self-play win rate, nothing else

- CLAIM: the fitness CMA-ES minimizes is `-(wins + 0.5*draws)/num_games`
  per candidate, measured by playing full games against an opponent pool;
  default 20 games per candidate per generation; no regularization term.
- EVIDENCE: master `experiments/cma/optimize.py:330-334` (fitness),
  `:472-473` (--games default 20), `:139-141` (opponent drawn from the
  pool — the docstring at `:3-5` still says "against the baseline" and is
  stale; the pool code governs, see SB-44).
- WHY IT MATTERS: Stage-2 method prior — sealbot's baseline table was
  tuned by direct game outcome with no gradient/label pipeline at all.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol Stage-2 experiment: tune a small weight
  subset by game-outcome fitness vs by regression labels at matched
  compute; SPRT both against the baseline.

### SB-42 — the CMA hyperparameters: popsize 50, sigma0 50.0, seed 42

- CLAIM: defaults are `--popsize 50`, `--sigma0 50.0`, `--max-gen 500`,
  `--seed 42`, with `tolfun=1e-6` the only convergence option set.
- EVIDENCE: master `experiments/cma/optimize.py:229-237` (CMAOptions),
  `:478-489` (argparse defaults).
- WHY IT MATTERS: a working hyperparameter starting point for tuning a
  ~364-dim codebook (sigma0 ~ 1/5 of the median weight magnitude).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: reproduce a few generations in a sandbox with
  seed 42 and confirm the log trajectory format (`optimize.py:288-289`).

### SB-43 — fitness games: empty board, 0.02 s/move, diversity only from
### clock jitter, exceptions scored as draws

- CLAIM: every fitness game starts from an empty `HexGame(win_length=6)`
  capped at 200 moves at a per-move time limit defaulting to 0.02 s (drawn
  from a degenerate uniform unless --time-limit-max is set); with
  deterministic engines all game diversity comes from wall-clock jitter,
  and any exception in `_play_game` is silently scored as a draw
  (`winner = Player.NONE`).
- EVIDENCE: master `experiments/cma/optimize.py:92-114` (empty board,
  max_moves 200), `:131` (time draw), `:474-477` (default 0.02),
  `:148-151` (exception -> draw).
- WHY IT MATTERS: three anti-patterns pistol's hard rules already forbid
  (no openings, wall-clock nondeterminism, swallowed errors) — pistol's
  Stage-2 tuner should use paired openings + fixed-node budgets and
  fail loud; sealbot's weights are nonetheless the product of this judge.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: play two fixed-budget games between deterministic
  pistol builds from the empty board and confirm they are identical —
  demonstrating the degenerate-diversity failure the openings fix.

### SB-44 — a growing opponent pool guards against opponent-specific tuning

- CLAIM: each fitness game's opponent is drawn uniformly from a pool that
  starts as the current baseline and gains every generation's best
  (`gen_%04d.h` via `_save_to_pool`); on promotion the pool resets to the
  new baseline alone.
- EVIDENCE: master `experiments/cma/optimize.py:139-141` (random choice),
  `:346-349` (save per gen), `:180-201` (_load_pool), `:438-440` (reset).
- WHY IT MATTERS: a cheap league mechanism against overfitting the tuning
  opponent — reusable in Stage-2/4 tuning where SPRT vs one frozen
  baseline can be gamed (see SB-11's h2h law).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: tune vs one frozen opponent and vs a 5-member
  pool; measure transfer of each to a held-out third engine.

### SB-45 — promotion gate: 65% over 400 games, then CMA restarts fresh

- CLAIM: a new best is re-evaluated over `--promote-games 400`; at
  wr >= 0.65 it overwrites `current/pattern_data.h`, a 100-game check vs
  `best/` runs, and CMA-ES restarts from the promoted point with sigma0
  reset.
- EVIDENCE: master `experiments/cma/optimize.py:397-440` (promotion block,
  restart 432-436), `:492-495` (defaults).
- WHY IT MATTERS: a concrete accept-threshold + restart schedule for
  iterated Stage-2 tuning rounds; pistol replaces the fixed-n gate with
  SPRT.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: simulate error rates of a 65%-of-400 gate vs
  SPRT(0,+20) at equal expected game counts.

### SB-46 — symmetry folds 729 -> 364 free parameters by color antisymmetry

- CLAIM: `mirror(i)` swaps ternary digits 1<->2; free indices are
  {i : i < mirror(i)} — exactly 364 = (729-1)/2 — expanded as
  pv[i]=param, pv[mirror(i)]=-param, pv[0]=0; both baked tables satisfy
  the antisymmetry with 0 violations (checked numerically in this
  session); spatial reversal is NOT folded in (only the optional 35-param
  mode uses it).
- EVIDENCE: master `experiments/cma/symmetry.py:6-8,25-48,95-119`;
  numeric check on `best/pattern_data.h` and `current/pattern_data.h`.
- WHY IT MATTERS: halves Stage-2 codebook dimensionality and guarantees
  color-symmetric eval by construction; the un-folded reversal symmetry
  is a further free halving pistol could take (to ~189) and measure.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pin a pistol test asserting the codebook
  satisfies pv[i] = -pv[swap(i)]; measure whether adding reversal
  symmetry changes tuned strength.

### SB-47 — an optional 35-param single-color mode exists (its help text
### is stale)

- CLAIM: `single_color_free_indices()` returns 35 indices (own-and-empty
  patterns deduped by reversal, mixed patterns zeroed) while the
  `--single-color` help string claims "63 params instead of 364".
- EVIDENCE: master `experiments/cma/symmetry.py:69-92` vs
  `experiments/cma/optimize.py:490-491`.
- WHY IT MATTERS: a coarse-to-fine tuning curriculum idea for Stage 2 —
  and a live example of pistol rule 9's "counts are derived, never
  asserted".
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: run `len(single_color_free_indices())` in a
  sandbox copy; expect 35.

### SB-48 — weight magnitudes: five-in-window ~48-50k, open-ended-4 ~2.3k,
### completed-6 untrained

- CLAIM: `best/pattern_data.h` values are doubles spanning +/-49587.6;
  the six five-own-one-empty windows score 48109.2-49587.6, pattern
  011110 scores 2279.79, 111100 scores 2901.39, and the all-own
  (completed-6) entry is 0.0 in best / -56.4 in master's current —
  untrained noise, because search never evaluates won boards; the branch
  log confirms "PATTERN_VALUES for completed-6 windows ~untrained (-56)
  ... linear ranks the winning move ~last" in offline child scoring.
- EVIDENCE: master `best/pattern_data.h:5-78` (49413.77 at :18, 49587.65
  at :39, 2901.39 at :10); `4098d22:experiments/strix/LOG.md:302-304`.
- WHY IT MATTERS: magnitude anchors for pistol's handcrafted/codebook
  scales AND a pitfall — terminal-pattern codebook entries must be pinned
  constants or excluded, never left to a tuner that cannot observe them.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: recompute the cited indices from the blob; add a
  pistol test that terminal-pattern entries are pinned constants.

### SB-49 — the whole table is machine-produced; nothing is hand-set

- CLAIM: commit 6402fb8 replaced `current/pattern_data.h` with
  full-precision CMA output (1 zero entry vs best's 29), preserving the
  prior weights as `current/pattern_data_v0.h` (numerically identical to
  `best/pattern_data.h`, unchanged since the initial commit ecd2328 — its
  CMA run predates the repo).
- EVIDENCE: `git show 6402fb8 --stat`; `6402fb8:current/pattern_data_v0.h`
  lines 6-12 vs `best/pattern_data.h:6-12`; `git log --follow --
  best/pattern_data.h` (44779d9, ecd2328 only).
- WHY IT MATTERS: the method prior is "tune everything, pin nothing by
  hand" — so pistol pinning mate-adjacent windows (per SB-48) is a
  deliberate deviation, not imitation.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: diff the ecd2328 blob against master's best
  table across the 44779d9 path move.

### SB-50 — the tuned weights are coupled to one fixed search build

- CLAIM: `cma_wrapper.cpp` compiles against `include_dirs=["../../best"]`
  and exposes `load_patterns(list of 729 doubles)` on a dedicated
  `cma_minimax_cpp.MinimaxBot` — CMA tunes weights inside the exact
  search implementation that ships.
- EVIDENCE: master `experiments/cma/setup.py:17`;
  `experiments/cma/cma_wrapper.cpp:2-5,62-68,90-95`.
- WHY IT MATTERS: Stage-2 tuning must run through pistol's own `Engine`
  trait/search build, not a side harness — weights do not transfer across
  search configs for free.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: retune a weight subset under two pistol search
  configs (e.g., quiescence on/off); measure the cross-config strength
  drop.

### SB-51 — the judge that accepted the weights: 20 games, no openings,
### Wilson CI + naive Elo, 3x grace

- CLAIM: `evaluate.py` defaults to 20 games at 0.1 s/turn with seat swap
  by game parity, a 200-move draw cap, `GRACE_FACTOR=3.0` /
  `MAX_VIOLATIONS_PER_GAME=10` forfeits, Wilson 95% CI, normal-approx
  p-value, and `-400*log10(1/score - 1)` Elo — and no openings, so with
  deterministic engines all "independent" games differ only by timing.
- EVIDENCE: master `evaluate.py:29-31,104-135,330,545-548`.
- WHY IT MATTERS: pistol-arena already exceeds this judge (paired
  openings, GSPRT, dedupe) — but any strength number quoted from sealbot's
  own README/history was measured by THIS judge and inherits its noise.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: run pistol-arena's distinct-game dedupe on a
  no-openings fixed-budget match to quantify how few independent games
  such a harness produces.

### SB-52 — the harness once compared a bot against ITSELF (C-extension
### module cache); fixed by per-bot subprocesses

- CLAIM: before cd6c5b0, both bots imported the same cached C extension —
  "CPython's C extension cache return[s] the same module for both (del
  sys.modules doesn't work for C exts)" — so current-vs-best matches
  could be self-play; the fix isolates each bot in its own subprocess.
- EVIDENCE: `git show cd6c5b0 -- evaluate.py`; surviving comment at
  master `evaluate.py:246-249`.
- WHY IT MATTERS: red-team case for pistol-arena — assert the two sides
  are provably distinct builds (per-game engine-reported identity), which
  pistol's weights-sha/id-line machinery (D-198/D-205) already trends
  toward.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: arena self-test that a deliberate same-binary
  pairing is detected and errors loudly.

### SB-53 — the linear 729 table's measured ceiling: regret .277, 43%
### wrong signs, moves-left-blind

- CLAIM: with strix values of all depth-2 children as oracle, the CMA
  linear table's decision regret is .277 vs trunk v1.1's .052; it carries
  43% wrong signs on decided positions; split by intra-turn phase it is
  ml-blind (regret .450 on second-stone positions vs trunk .109, "picks
  oracle -1.0 losers") — while its +/-49k window weights hard-code
  must-block priority, which is why it stays competitive at move choice.
- EVIDENCE: `4098d22:experiments/strix/LOG.md:315-318,280-294`;
  `4098d22:experiments/nnue/LOG.md:67-68`.
- WHY IT MATTERS: quantifies the Stage-2 headroom target (~5x decision
  regret) over the linear codebook form, and two design inputs: the eval
  must see intra-turn phase (pistol already keys zobrist on it), and
  must-block behavior can live in weights OR in the WP-1.5 filter — but
  something must own it.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: mini decision-regret battery in pistol: deep
  pistol search as oracle over all D2 children of n positions; compare
  eval-argmax regret for handcrafted v0, split by intra-turn phase.

### SB-54 — offline-metric wins on the linear table did not transfer to play

- CLAIM: a ridge refit on deep labels beat the CMA table on every offline
  metric (spearman 0.518 vs 0.425, sign agreement 0.851 vs 0.576) yet
  gated only +28 Elo at blend 0 and <= 0 at other blends; the shipped
  champion kept the original CMA table.
- EVIDENCE: `4098d22:experiments/nnue/LOG.md:70-85,92-93`.
- WHY IT MATTERS: pistol Hard Rule 6 applied to Stage 2, measured in the
  wild — regression fit is not an acceptance criterion; only match
  results are.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: when pistol fits codebook weights from search
  labels, pre-register the SPRT gate and record offline-vs-play
  divergence explicitly.

### SB-55 — the Mixnet bake: 3^11 per-axis codebook, zero-anchor, 75/25
### distill CE

- CLAIM: MIXNET_DESIGN bakes a learned mapping CNN into a per-direction
  codebook of 3^11 = 177,147 length-11 line patterns with a zero anchor
  (mapping(all-empty) == 0, subtracted at bake) so sparse pooling equals
  infinite-board pooling exactly, trained with 0.75*CE(soft teacher) +
  0.25*CE(true label), Adam 1e-3.
- EVIDENCE: `ef40a22:experiments/strix/MIXNET_DESIGN.md:10-12,33,35,63-66,
  87-89`.
- WHY IT MATTERS: the zero-anchor trick is what makes an incremental
  codebook eval EXACT on pistol's unbounded board (empty regions
  contribute literally zero) — a Stage-2 design element pistol should
  adopt from day one; length-11 windows also match pistol's Stage-2 spec.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: prototype the zero-anchor property in
  pistol-eval: assert the empty-region contribution is exactly 0 and that
  incremental == from-scratch on random unbounded positions.

### SB-56 — their parity-test recipe: exact table parity, toleranced eval
### parity, zero accumulator drift through search

- CLAIM: `test_parity.py` checks (1) engine feature counts == numpy
  extraction exactly on 60 random positions, (2) engine leaf eval ==
  trainer forward within 3e-4 relative, (3) accumulator drift after a
  real search with make/undo + rollback < 3e-4; the mixnet port repeats
  it ("policy parity 0.0 rel, acc_drift exactly 0 through search") and
  `mixnet_bake.py` verifies table-forward == module-forward as "the
  parity oracle for the C++ port".
- EVIDENCE: `6892e5e:experiments/nnue/test_parity.py:1-7`;
  `4098d22:experiments/nnue/LOG.md:23-25`; commit 8acabcd;
  `ef40a22:experiments/strix/mixnet_bake.py:1-25`;
  `ef40a22:experiments/strix/MIXNET_DESIGN.md:88-89,118-121`.
- WHY IT MATTERS: a ready Stage-2 CI recipe mapping directly onto
  pistol-eval's apply/undo contract and D-68's from-scratch oracle: SB-18
  shows what skipping it cost (1-149).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: three behavior-named pistol tests: codebook
  lookup parity vs the training-side table, eval parity within a pinned
  tolerance, apply/undo drift == 0 after a fixed-node search.

### SB-57 — the deep-label corpus that produced the +338 champion: 693k
### positions labeled at tl 0.12

- CLAIM: the NNUE champion was trained on 693k positions labeled by the
  engine's own deep search at tl 0.12 ("deep_score, blend 0.15: +363;
  confirmed 200g: 87.5% / +338, p=2.8e-26"), a ~6x-deeper teacher than
  the 0.02 s CMA fitness games.
- EVIDENCE: `4098d22:experiments/nnue/LOG.md:30-33,58-62,100`.
- WHY IT MATTERS: a corpus-size and teacher-depth anchor for pistol's
  Stage-2 data budget (together with SB-23's shallow-label DISCARD it
  brackets the useful teacher-depth range).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol Stage-2 ablation: identical nets fit on
  depth-2-turn vs depth-4-turn labels of the same positions; SPRT both.

## D. Weakness inventory and the threat/threshold census (WP-1.5 feed)

Scope note: on master, `best/` and `current/` differ ONLY in
`pattern_data.h` weights (verified by `diff -r` on the clone); every
engine threshold below is identical in both. `vcf.h` exists only on the
branches — master is the pure alpha-beta bot.

### SB-58 — the engine's own todo ranks "fix inability to block colonies" #1

- CLAIM: todo.md's first open item is "Fix inability to block colonies",
  and null-move pruning, aspiration windows, and PVS are also absent (root
  searches the full window alpha=-INF, beta=+INF; grep for
  aspiration/null over `current/engine/*.h` finds nothing).
- EVIDENCE: master `todo.md:3-7`; `current/engine/search.h:411`.
- WHY IT MATTERS: colony play is the flagship exploit class for pistol's
  Stage-1 fixtures, admitted by the author; and the strength baseline is
  plain alpha-beta, so pistol's Stage-1 target is not a fancier search.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: scripted-colony arena game (see SB-65).

### SB-59 — the must-block filter SILENTLY UNFILTERS when no pair covers
### everything

- CLAIM: must-block returns `out.empty() ? turns : out` — when no
  generated pair covers every opponent four-threat window, the filter
  falls back to ALL turns, i.e. exactly when the opponent's threat is
  hardest to meet, it stops filtering instead of proving loss or
  maximizing resistance.
- EVIDENCE: master `current/engine/movegen.h:108`.
- WHY IT MATTERS: a named anti-pattern for WP-1.5: pistol's must-block
  must fail loud (Hard Rule 3) into "no cover exists" knowledge — play
  the longest-resistance line or return a proven-loss score, never
  quietly widen.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: fixture with two opponent 4-windows whose only
  2-stone cover uses cells outside the pair wedge (SB-16); predict
  sealbot plays a non-blocking move.

### SB-60 — quiescence stands pat unless a count-4 window already exists

- CLAIM: qsearch returns static eval whenever qdepth hits 0 OR neither
  side has a hot (>= 4 own, 0 opp) window, so forcing sequences that must
  first CREATE fours from threes are invisible to it; threat cells are
  generated only from >= 4-count clean windows.
- EVIDENCE: master `current/engine/search.h:349-357`;
  `current/engine/movegen.h:49-69`.
- WHY IT MATTERS: pistol's WP-1.6 quiescence needs threat GENERATION
  (three -> four extensions in turns), not only threat resolution, or it
  inherits the same one-turn horizon.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: fixture whose winning line starts from two clean
  3-windows and no existing four — sealbot's qsearch scores it quiet.

### SB-61 — quiescence never generates block-and-counter defenses

- CLAIM: in quiescence movegen, when the opponent has threat cells the
  only turns generated are pairs drawn from opponent-threat empties (or
  one threat cell + best-delta companion when only one exists) — a
  defense blocking with one stone and counter-attacking with the other is
  never in the set.
- EVIDENCE: master `current/engine/movegen.h:183-208`.
- WHY IT MATTERS: block-and-counter is the canonical Connect6 defensive
  resource; WP-1.5/1.6 must emit block+free-stone pairs with the free
  stone chosen by attack value.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: fixture where the sole non-losing defense is
  (block, own-four-creating stone); sealbot's qsearch value is provably
  wrong there.

### SB-62 — the history table is dead code

- CLAIM: `_history` is written on beta cutoffs (+= depth*depth) and
  cleared but never read anywhere since the "delta-only scoring" change
  (f53f926); the branch roadmap lists "dead `_history` table" for
  deletion.
- EVIDENCE: grep over master `current/`: writes at
  `current/engine/search.h:639-640,659-660`, declaration
  `current/engine/bot.h:175`, no reads; `ef40a22:memory/
  sealbot-improvement-roadmap.md`.
- WHY IT MATTERS: sealbot provides NO evidence for or against history
  ordering (WP-1.7) — its measured strength rests on TT-move + killers +
  delta only; do not cite it as a history-heuristic datapoint.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: the grep is the verification; recompile with the
  writes removed and confirm identical node counts at fixed depth.

### SB-63 — the linear eval cannot price "two independent threats = win";
### 602/729 patterns are noise

- CLAIM: the branch analysis of master's eval records that 602/729
  patterns are dead (mixed-color windows) yet carry median |w| ~ 345 of
  noise, real signal lives only in clean 4-windows (~+2.3k) and 5-windows
  (~+49k), and a linear window-sum "cannot represent 'two independent
  threats = win'" — stated there as the root cause of the colony todo.
- EVIDENCE: `ef40a22:memory/sealbot-improvement-roadmap.md` (analysis of
  c94749c); magnitudes in master `current/pattern_data.h`; the -56
  completed-6 quirk also at `4098d22:experiments/strix/LOG.md:302-304`.
- WHY IT MATTERS: Stage-2 structural requirement — pistol's eval (or its
  threat layer) needs a non-linear threat-count term, because double
  threats are exactly what colonies and every won position monetize.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: ablation on pistol's v0 eval: two disjoint
  clean-4 formations vs one; confirm the difference is priced >= the cost
  of one blocked four.

### SB-64 — colony blindness, the complete mechanism

- CLAIM: sealbot cannot block an opponent colony because every defensive
  organ triggers only at count-4 windows: colony-adjacent candidates
  exist (distance-2 of any stone) but must survive the delta-ranked cap
  of 15 (interior) / 20 (root) against main-battle cells; must-block and
  threat cells activate only at `my_count >= WIN_LENGTH-2 && opp == 0`;
  and the linear eval cannot express that two remote clean threes are
  jointly lethal — so the colony builder gets ~2 free tempi per turn
  until a four exists, by which time a 2-stones-per-turn double four is
  unstoppable.
- EVIDENCE: trigger counts at master `current/engine/movegen.h:22,82`;
  caps at `current/engine/constants.h:44-45` applied at
  `current/engine/search.h:573-575` and `current/engine/movegen.h:138-139`;
  root-cause statement in `ef40a22:memory/sealbot-improvement-roadmap.md`;
  `todo.md:3`.
- WHY IT MATTERS: THE canonical failure mode of capped candidate policies
  (sealbot_notes.md already flags it) now with its full causal chain —
  WP-1.5's must-block should trigger on threat trajectories (clean 3s
  with room), and the candidate policy must reserve slots for
  opponent-heat regions regardless of own-delta rank.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: scripted colony opponent (2 stones/turn in a
  fixed remote pattern reaching a double four in 3 turns); count how many
  sealbot turns land >= distance 5 from the colony — predicted: all,
  until a four appears, then lost.

### SB-65 — colony BLOCKING was never attempted; colony PLAYING emits
### platform-illegal moves ~8/150 games

- CLAIM: `git log --all -S colony` shows no blocking fix was ever tried;
  meanwhile the colony candidate generates HeXO-illegal far moves that
  their bench bridge substitutes with the nearest legal cell (~8/150
  games for the NNUE champion) — and that log note describes HeXO
  legality as "radius 6 of a stone", a discrepancy against pistol's
  platform-confirmed radius 8 (D-101) worth re-checking at the bridge.
- EVIDENCE: the -S sweep;
  `4098d22:experiments/strix/LOG.md:13-17`.
- WHY IT MATTERS: the weakness is open and un-attempted — a clean WP-1.5
  differentiator; the bridge note is also the second independent claim
  about the platform's legal radius, and it disagrees with D-101's
  confirmed 8, so the match adapter must pin the value from the htttx
  spec, never from either engine's folklore.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: re-read the htttx spec / platform source for the
  legal radius (operator act, D-101 said 8); count sealbot's out-of-region
  proposals per game under enforcement.

### SB-66 — decision quality is measurably WORST on human positions

- CLAIM: decision regret on a 600-position KrakenBot human set is 0.254
  vs 0.175 (REAL set) and 0.115 (PERT set), and close-call resolution is
  5-7x worse than the strix policy (~0.17-0.25 vs 0.026-0.057),
  compounding over ~25 decisions/game.
- EVIDENCE: `4098d22:experiments/strix/LOG.md:239-251`.
- WHY IT MATTERS: pistol's fixture and opening sourcing (WP-1.2a already
  uses human openings) — human-game distributions are where this engine
  family's ordering/eval is weakest, so wins there are cheapest and most
  human-relevant.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol arena runs on human-opening vs random
  books against a fixed opponent; compare relative gains.

### SB-67 — cross-move state: killers cleared only on side change, TT never

- CLAIM: killers/history are cleared only when the side to move changes
  (`if (_cur_player != _player)`), and the TT has no clear call at all —
  so search state persists across `get_move` calls and the same position
  can search differently depending on the game path that reached it.
- EVIDENCE: master `current/engine/search.h:45-48`;
  `current/engine/bot.h:149-172` (no clear).
- WHY IT MATTERS: pistol's determinism law — instrument mode must define
  TT/killer lifetime explicitly; this is the "stale cross-move state"
  trap the determinism self-test exists to catch.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: reach one position via two different move orders
  in sealbot and compare node counts (expect a difference); assert
  pistol's self-test does not exhibit it.

### SB-68 — the master threshold census, in one place

- CLAIM: master constants are CANDIDATE_CAP=15, ROOT_CANDIDATE_CAP=20,
  PAIR_SUM_CAP=14 (56 interior pairs, i<j<15 & i+j<=14), NEIGHBOR_DIST=2,
  DELTA_WEIGHT=15, MAX_QDEPTH=16, WIN_LENGTH=6, WIN_SCORE=1e8,
  WIN_THRESHOLD=WIN_SCORE-1000, ARR=140/OFF=70; root ID re-sorts root
  turns by previous-iteration exact scores each depth.
- EVIDENCE: master `current/engine/constants.h:44-58`;
  `current/engine/tables.h:60-62`; `current/engine/search.h:171-177`.
- WHY IT MATTERS: the complete numeric prior set for pistol's candidate
  policy and search config — all suspect until re-derived on 3 axes, but
  each one is a pre-registered starting bracket.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: node-matched sweeps per knob as WP-1.3 did for
  radius, reproducing at least the SIGNS of SB-06's width result first.

### SB-69 — DELTA_WEIGHT=15 is inert: only its sign is used

- CLAIM: DELTA_WEIGHT's sole use is `dsign = maximizing ? DELTA_WEIGHT :
  -DELTA_WEIGHT` multiplying every candidate's delta before a sort, so
  only its sign can affect anything; the "// 1.5" comment marks it as a
  vestige of a removed history blend.
- EVIDENCE: master `current/engine/search.h:557` (sole use, grep);
  `current/engine/constants.h:48`.
- WHY IT MATTERS: do not import DELTA_WEIGHT as a tuning prior — a live
  example of a config value that LOOKS load-bearing and is not.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: recompile sealbot with DELTA_WEIGHT=1; assert
  identical bestmoves and node counts at fixed depth.

### SB-70 — TT: 2^20 direct-mapped on master, upper-32-bit verify,
### intra-turn phase in the key

- CLAIM: the master TT is a fixed 2^20-entry direct-mapped array verified
  by the hash's upper 32 bits, replacement always-replace on key mismatch
  and depth-preferred on match; the key mixes side-to-move AND intra-turn
  stones-left (`_hash ^ cur_player*C1 ^ moves_left*C2`); killers are 2
  slots/ply to ply 64.
- EVIDENCE: master `current/engine/bot.h:33,152-172,214-217,178-186`.
- WHY IT MATTERS: independent convergence on pistol's zobrist design
  (side-to-move + intra-turn phase in the key was pinned in pistol-core
  from day one); the 32-bit-verify-vs-index split is the weak point
  sealbot_notes.md already flags vs pistol's D-8 layout.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol already tests phase-in-key; optionally
  measure sealbot TT collision rate at depth 5 on 100 positions.

### SB-71 — mate scores count PLIES (stones), not turns

- CLAIM: mate scoring is +/-(1e8 - ply) with mate-distance TT adjustment
  gated by WIN_THRESHOLD = 1e8 - 1000 (capping representable mate
  distance at 1000 ply), where ply increments per STONE — so sealbot
  prefers fewer stones, not fewer turns.
- EVIDENCE: master `current/engine/bot.h:137-146`;
  `current/engine/constants.h:51-52`; usage
  `current/engine/search.h:333-334,423`.
- WHY IT MATTERS: pistol pins sudden death in TURNS (rule 4); the unit
  mismatch is a golden-fixture theme — same mate reachable in 2 turns via
  3 vs 4 placed stones must be distinguished correctly by pistol.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: golden fixture as above; assert pistol prefers
  fewer turns and document sealbot's stone-count preference.

### SB-72 — the branch VCF integration budget, complete

- CLAIM: the branch VCF wiring is: bit 1 root attack probe at k=vcf_k;
  bit 2 root defense filter with fk = max(4, vcf_k/2), fb =
  clamp(vcf_node_budget/6, 800, 8000), a hard clock cutoff at 40% of the
  move budget, and all-losing => keep original turns; bit 4 interior
  probe at depth >= 2 with k=3 and budget max(400, budget/12); bit 8
  post-search veto reserving 0.82/0.16 of the clock, <= 5 probes, chosen
  move probed at k+3 with doubled budget, alternatives at max(6, k-2);
  defaults vcf_k=8, budget 5000 (champion 25000).
- EVIDENCE: origin/mixnet-repro ef40a22, `current/engine/search.h:143,
  209-259,404-458,1015-1028`; `current/engine/bot.h:207,216`; history
  98bac5a, 797732b.
- WHY IT MATTERS: a complete, gate-tested budget architecture for Stage-3
  forcing search under a sub-second clock, born directly from the loss
  post-mortems (SB-03/SB-05).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol Stage-3 ablation of veto slice on/off at
  equal total clock, pre-registered to reproduce SB-25's "veto earns its
  clock" sign.

### SB-73 — threat substrate: per-window (countA,countB) with dirty
### insert / clean re-check

- CLAIM: threats are defined exclusively on contiguous 6-cell line
  windows via per-window (countA,countB) pairs; hot sets are inserted at
  count >= 4 REGARDLESS of contamination and cleanliness (opp == 0) is
  re-checked at every use site (insert-dirty / erase-below-4 asymmetry).
- EVIDENCE: master `current/engine/board.h:41-53`;
  use-site checks `current/engine/movegen.h:22,57,82`.
- WHY IT MATTERS: the natural data structure for WP-1.5's tables (as
  sealbot_notes.md predicted) plus a concrete mutation-testing target:
  pistol should pin hot-set == from-scratch recount after make/undo.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol pinning test comparing incremental
  hot-set contents to a from-scratch recount across random make/undo
  sequences.

### SB-74 — the 2-stones-per-turn arithmetic is exactly right at the
### four/five level

- CLAIM: instant win = clean window with >= 4 own (WIN_LENGTH-2: two
  stones complete it); defense = hit every clean opponent-4 window with
  >= 1 of the pair's stones; unblockable = no single pair covers all such
  windows (exhaustive pair-cover test) returning a mate score one turn
  out; the branch VCF states the same rule as "min hitting set >= 3 =>
  attacker wins next turn".
- EVIDENCE: master `current/engine/movegen.h:22`;
  `current/engine/search.h:496-537` (cover test; score -/+(WIN_SCORE -
  _ply - 1) at :532-533); `ef40a22:current/engine/vcf.h:13-17,204`.
- WHY IT MATTERS: a correct, reusable kernel for WP-1.5's must-block and
  unblockable-double-threat check — the pair-cover formulation is the
  right arithmetic for 2-stone turns and is worth porting as-is.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: golden fixtures: (a) two disjoint 4-windows —
  blockable; (b) three disjoint — declared lost exactly one turn early;
  (c) two windows sharing one empty — 1-stone block found.

### SB-75 — threat level 3 is invisible to EVERY defensive path — and the
### helper that sees it is never called

- CLAIM: must-block, threat cells, quiescence, and the unblockable test
  all require count >= 4, while `has_near_threats()` — which detects "2+
  clean windows with 3+ stones" — exists in the API but has zero call
  sites in the search.
- EVIDENCE: thresholds master `current/engine/movegen.h:22,57,82`;
  `current/engine/bot.h:84-94` (helper, grep: no callers in
  search.h/movegen.h).
- WHY IT MATTERS: WP-1.5 must define triggers in TURNS-TO-WIN (a clean
  double-3 is a 2-turn threat under 2-stone turns), not raw counts; this
  is the precise blind spot the colony exploit (SB-64) and a strong
  human's slow buildup both walk through.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: fixture with two clean open 3-windows for the
  opponent; depth-limited sealbot ignores them, pistol's must-block flags.

### SB-76 — anything spanning more than one 6-cell window is invisible

- CLAIM: windows are 3 dirs x 6 contiguous offsets, so split fours like
  "X X . X X" count iff they fit one 6-cell window, and any formation
  whose completing structure spans > 6 cells — and ALL openness beyond
  the window (overline room, edge-of-cluster space) — is invisible to
  both threat detection and the 729-pattern eval (PATTERN_EVAL_LENGTH=6).
- EVIDENCE: master `current/engine/tables.h:45-47`;
  `current/engine/board.h:39-53`; `current/pattern_data.h:3`; gap listed
  in `ef40a22:memory/sealbot-improvement-roadmap.md`.
- WHY IT MATTERS: Stage-1 fixture theme (7+-cell split formations,
  window-boundary traps) and a Stage-2 argument for pistol's planned
  length-11 windows.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: paired fixtures with identical window-count
  multisets but different real threat values; sealbot evals them
  identically, pistol must not.

### SB-77 — the VCF solver's exact exclusion list and caps

- CLAIM: the branch VCF returns unknown (never a false win) when: the
  defender's hitting set has size <= 1 (free stone), or the defender has
  any immediate counter-win; attacker forcing turns are limited to
  fours/fives built from clean-3 completions and restricted clean-2
  pairs; caps are VCF_MAX_THREAT_WINDOWS=32, VCF_MAX_U=24,
  VCF_MAX_CANDS=192, node budget default 5000 — "+1 only on proven wins".
- EVIDENCE: `ef40a22:current/engine/vcf.h:1-31,39-41,179-186,254,336-427`;
  validation in commit 01891ee's message.
- WHY IT MATTERS: defines exactly where pistol's WP-1.8 df-pn must go
  BEYOND the cheap solver (VCT, counter-threat handling, free-stone
  defense) — the exclusions are the roadmap of remaining value.
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: port the 21 synthetic tests referenced in
  01891ee as sha-pinned pistol-solver fixtures; add one case per excluded
  class expecting unknown-from-VCF / proven-from-df-pn.

### SB-78 — nondeterministic free stone in the instant-win pair

- CLAIM: `_find_instant_win` on a 5-stone window returns a winning pair
  whose second stone is the first cell of `_cand_set` iteration order —
  harmless for the result, but bestmove text depends on hash-set
  insertion order.
- EVIDENCE: master `current/engine/movegen.h:33-39`.
- WHY IT MATTERS: pistol's determinism law — winning-turn construction
  must pin the free stone by a total order or the determinism self-test
  flakes exactly on won positions (a subtle CI trap worth a named test).
- STATUS: UNVERIFIED.
- PROPOSED VERIFICATION: pistol CI case: position with a 5-window and
  multiple candidates; assert identical bestmove across runs and across
  state reload.

## The five claims most worth verifying first

Ranked by (imminence of the WP they feed) x (quantified stakes) x
(cheapness of the verification):

1. **SB-16 + SB-59 — the pair-cap wedge and the silent must-block
   fallback.** WP-1.5 is the next work package and these are its two
   named anti-patterns, with ~500 Elo of measured stakes (-512 -> -104
   recovery) attached to the wedge alone. Verify first: the pistol test
   that a must-block pair survives every cap is an afternoon, and it
   shapes WP-1.5's design before code exists.
2. **SB-75 + SB-64 — the count>=4 trigger blindness and the colony causal
   chain.** This is the mechanism strong humans exploit, admitted in
   their own todo and never fixed. Verify by building the scripted-colony
   and clean-double-3 fixtures — they become pistol's Stage-1 tactical
   fixture themes whether or not sealbot behaves exactly as predicted.
3. **SB-01 + SB-77 — the hitting-set VCF solver (~40 us median, proven
   sound on 79/79) and its exact exclusion list.** The strongest single
   design prior for WP-1.5's unblockable check and WP-1.8's solver, and
   its adversarial-playout validation protocol is worth copying outright.
4. **SB-33 — partial root-iteration harvest.** Feeds WP-1.4, which is in
   flight RIGHT NOW: the aborted final iteration can supersede the
   previous best soundly when the previous best is searched first. Cheap
   to verify (SPRT on/off in play mode) and immediately relevant.
5. **SB-05 + SB-02 — losses are proof-budget-constrained, and the VCF
   stack alone moved ~150 Elo.** Together these order the whole Stage-1/3
   investment: after ordering reaches the sqrt(b) floor (SB-20), the
   marginal Elo is in threat proof depth, not width or ordering. Verify
   by replicating the blunder-autopsy methodology on pistol's own arena
   losses once WP-1.5 lands.

## Binding rule

Per D-206 (landed with this document): nothing in this register may drive
a pistol code change until the claim is verified by its proposed method;
the register is a lead sheet, not a design source. Verification results
amend the register entry by entry (STATUS: UNVERIFIED -> VERIFIED or
REFUTED, with the measurement's citation).

## Appendix — citation spot-check review (fresh context, appended verbatim
## minus per-entry bodies duplicated above)

Reviewer: fresh-context subagent, dispatched against the draft at SHA256
5fee21612325b630aa795456e311564f13638a45bafcc9c715c02da1c5728747, clone
tips verified master = c94749c21c16c3b072fff6da49762dd5f92f3986,
origin/nnue-eval = 6892e5e0d673f67e9dbf16f8334062ad86ef62a2,
origin/mixnet-repro = ef40a22ae346d63fbaa9fadc96fa7921f865c32d (all match
this document's provenance block). Random selection: seed 20260819,
`random.sample(range(1,79),12)` -> [4, 10, 15, 19, 21, 23, 41, 49, 53,
56, 63, 69]; extras (non-random): SB-27, SB-48.

Per-entry verdicts: SB-04 PASS; SB-10 PASS; SB-15 PASS; SB-19 PASS
(quoted strings verbatim, all five commits match); SB-21 PASS; SB-23
PASS; SB-41 PASS-with-minor — the claim's "opponent pool" wording was
backed only by lines cited under SB-44, and the docstring the entry cited
says "against the baseline" (stale); SB-49 PASS (including a full
729-value diff showing pattern_data_v0.h identical to best's table);
SB-53 PASS; SB-56 PASS; SB-63 PASS; SB-69 PASS (reasoning re-derived from
code, not just quoted); extra SB-27 PASS; extra SB-48 PASS.

Overall verdict: "YES — the register's citation discipline is sound
enough to land. 12/12 randomly selected entries plus 2 extras verified:
every cited file and commit resolves at the named revision, every quoted
number/string was found at or within the cited lines (no line drift
beyond +/-2 anywhere), and no claim sentence misstated its source."
Required correction: amend SB-41's evidence line (the only finding).

Author's action: SB-41's evidence line was amended exactly as required
AFTER the review (so the landed file's hash differs from the reviewed
draft's hash by that correction and this appendix); no other entry was
touched. The reviewer's note stands: the sample is 14/78 — a spot-check,
not exhaustive verification — and every entry remains STATUS: UNVERIFIED
under the register's binding rule.
