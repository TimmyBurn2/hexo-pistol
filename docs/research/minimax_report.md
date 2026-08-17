# Maximum-Strength Classical Minimax Bot for Hex-Lattice Connect(6,2,1): Decision Report

## TL;DR
- **BUILD a threat-guided PVS (principal variation search) alpha-beta engine with a Rapfi-style incrementally-updated pattern-codebook evaluation ("NNUE-lite"), one turn = two same-side plies, a threat-first staged candidate generator, and a df-pn-based forcing-line solver reusing your existing AND-OR solver.** This is the highest-strength architecture at the 0.5 s/move budget and it is directly validated by the closest published analog: Jin, Duan & Hang's Rapfi (this exact architecture on square-board gomoku) reached "strength surpassing Katagomo," the strongest AlphaZero-style gomoku engine, "under limited computational resources where accelerators like GPUs are absent" — the paper puts α-β+Mixnet at approximately 400 Elo above that SOTA agent in a CPU-only environment.
- **On the stated hardware, keep leaf evaluation on the CPU (incremental codebook), not the GPU.** Batched GPU leaf evaluation loses to CPU-incremental NNUE under sub-second sequential alpha-beta because per-leaf latency and batch-stall dominate and depth-first search exposes too few leaves at once; the GPU's defensible role is offline training and optional offline opening solving.
- **Expect roughly 5–8 full turns of uniform selective depth plus 15–30-turn forcing lines within 0.5 s on 8–32 cores — enough to refute the tactical class your NN misjudges.** Measure everything with deterministic fixed-depth/fixed-node modes and SPRT with paired balanced opening books; never make wall-clock-only claims.

---

## Key Findings

1. **The architecture question is essentially settled by Rapfi.** On square-board gomoku, Jin, Duan & Hang's alpha-beta engine using a distilled pattern-codebook network ("Mixnet") with incremental ("efficiently updatable") evaluation "reached strength surpassing Katagomo, the strongest open-source Gomoku AI based on AlphaZero's algorithm, under limited computational resources where accelerators like GPUs are absent" (arXiv:2503.13178). Concretely the paper reports that "combining MixNet with α-β search ... achiev[es] a strength of approximately 400 ELO above the SOTA agent in a CPU-only environment," and separately that "all MixNet configurations consistently outperform ResNet baselines by a significant margin of 300-400 ELOs, translating to a winning rate of 85%-92%." Rapfi "ranks #1 among 520 Gomoku agents on Botzone, and won the championship, defeating 54 competitors" at GomoCup 2024. This directly answers your central "learned-but-cheap eval under alpha-beta" question: **YES, and it beats the NN+MCTS approach at equal (CPU) compute.**

2. **The two-stone turn is the central structural challenge and it is well-studied for Connect6.** NCTU6, RZOP, CTSS/ITSS/CRZS, dependency-based search (DBS) for Connect6, and job-level PN all wrestle with "two stones per move." The pair-move branching factor is ~m²/2, and the defender also replies with two stones, which changes threat arithmetic fundamentally versus one-stone gomoku: a single "four" is no longer forcing because the defender has two stones to both block and develop. Connect6's solvers therefore center on VCDT (victory by continuous double-threat) and VCST (victory by continuous single-threat-or-more), not simple VCF.

3. **Threat/relevance-zone search is the proven route to deep forcing lines, and it transfers structurally (but not numerically) to 3 axes.** RZOP (Wu & Lin, IEEE TCIAIG 2(3):191–207, 2010, built on Thomsen's lambda search) plus proof-number search solved multiple Connect6 openings including the "Mickey Mouse" opening: "Using the JL-PN search together with our RZOP search, we successfully solved up to 65 positions ... [including] the Mickey Mouse opening, which used to be one of the popular openings before we solved it." NCTU6's VCST solver "is able to find a [λ]-strategy up to depth 25 where the size of the longest path with [λ]-moves is 13." These are square-board (4-axis) results; the *algorithms* transfer directly, but every threat count, relevance-zone radius, and win-density number must be recomputed for 3 axes.

4. **Your prior experiments are consistent with the literature.** Brute candidate-widening collapsing depth at ~100× cost is exactly why the Connect6 field uses dependency-based and conservative threat-space search rather than full enumeration. Per Wu et al.'s "Dependency-Based Search for Connect6" (Advances in Computer Games, LNCS, Springer), "the experimental results show that DBS yields a speedup factor of 4.12 on average, and up to 50 for some hard positions," precisely by pruning independent threat regions.

5. **Complexity numbers must be quoted carefully.** For square-board Connect(19,19,6,2,1), the state-space complexity is ~10^172 (same as Go/Gomoku, since legal positions coincide) and game-tree complexity is quoted as ~10^140 — but that figure is disputed. The 10^140 estimate derives from "(300×300/2)^30 ∼ 10^140"; a Wikipedia editor (Talk:Game complexity, 13 Jan 2008) notes "The number 140 ... is uncited and based on a questionable assumption. If one makes a different (and perhaps more reasonable) assumption one gets 70 (in other words, the same as that for Gomoku)." Your game is on an *unbounded* board, so both complexities are formally infinite; the practical complexity is set by your radius-8 legal region and your search-candidate policy, not by these Go-board numbers. Report the conflict; do not average.

6. **There is directly relevant hex-lattice theory.** Sieben, "Hexagonal polyomino weak (1,2)-achievement games" (Acta Cybernetica 16(4):579–585, 2004), studies *exactly* your turn structure: "in which the first player marks one cell and the second player marks two cells at each move. All polyominos but one on an infinite 2-dimensional hexagonal board are characterized to be weak winners or losers" (the lone exception is animal P7, conjectured a loser). This establishes that the (1,2) handicap on the hex lattice is strong enough to make most target shapes losers for the maker. It is an achievement game (form a fixed shape), a close cousin — not identical — to your maker-maker connect-6 goal; treat it as directional evidence that hex + 2-stone defense is defense-favorable, not as an exact verdict for your game.

---

## Details

### A. Search under compound 2-stone turns

**Decomposition: two same-side plies vs native pair-moves.** Treat one turn as a native pair-move for correctness but generate it as two *sequential same-side plies internally*, with an intra-turn marker in the transposition-table key. Decisive reasons:
- **Mate-distance/turn semantics (LAW): measure depth in TURNS.** A win completes the instant a placed stone makes ≥6, and the turn's second stone is not played. If you split a turn into two plies you must encode "this is the mover's second stone of the same turn" in the state (and Zobrist key), or transposition and mate-distance accounting break. Connect6 win detection is explicitly about the completing stone — so score sudden-death in turns and carry an intra-turn phase bit.
- **Branching control.** With m candidate cells, a native pair-move has ~m²/2 children. Every Connect6 engine attacks this with (a) threat-first generation (pairs that make/block threats before quiet pairs), (b) dominance pruning between pair members (if stone A is dominated everywhere by A′, prune pairs containing A), and (c) staged/lazy generation (TT/killer pair first, then threat pairs, then a widening quiet set only if needed). The two-plies-internally trick converts m²/2 pair-generation into two ordinary m-wide generators with an alpha-beta cutoff between them, which is where most pruning leverage comes from — the first "stone" fails high and you never enumerate its partners.

**Standard alpha-beta enhancements — evidence in the gomoku/Connect6 family:**
- **PVS/NegaScout, iterative deepening, TT, aspiration windows, killer/history:** universal in strong gomoku engines (Yixin, Embryo — a Stockfish derivative — and Rapfi all use alpha-beta with these). BUILD all; table stakes.
- **Late move reductions (LMR), futility/razoring:** Rapfi explicitly draws on "futility pruning ... razoring ... null move pruning ... late move reduction ... and history reduction." Square-board-validated in gomoku; PROTOTYPE with per-feature SPRT because the two-stone move and sudden-death threat density change safe reduction margins.
- **Null-move pruning:** the game is zugzwang-free (an extra stone never hurts the mover), so null-move is theoretically sound — but in sudden-death threat races the danger is that passing hands the opponent a double-threat you can't then verify. Gomoku engines use null-move; the Connect6 solvers use "null-move" and "seminull-move" concepts inside VCDT/VCST proof search (RZOP solves seminull positions via VCST). PROTOTYPE null-move in the quiet regime with a threat-verification guard (never null-move when the opponent has an open double-threat), and prefer verified null-move (search reduced, then confirm).
- **Multi-cut / ProbCut:** no specific Connect6 evidence found; SKIP initially, PROTOTYPE only once the eval is well-calibrated enough for reliable margins.

### B. Threat calculus and forcing-sequence search

**Threat taxonomy on 3 axes (transfer flag).** Connect6's taxonomy (single/double/triple threat; VCDT, VCST) is built on 4 axes. On the hex lattice there are only **3 line axes**, so:
- Fewer axes through each cell means fewer simultaneous lines a single stone participates in → **fewer natural fork/double-threat geometries per stone**, biasing the game toward the defender relative to square Connect6. This is qualitatively consistent with Sieben's hex (1,2)-achievement result that most shapes are losers for the maker.
- Blocking geometry changes: a block interdicts one of 3 axial directions, and the 6-neighbour structure changes which single defensive stone covers which threats. All published threat-count thresholds (how many stones make a "live" vs "dead" threat; how many simultaneous threats force a win against a two-stone defense) must be **re-derived**; do not import Connect6's numeric thresholds.

**The core threat-search family and what each buys:**
- **Threat-Space Search (TSS) / Dependency-Based Search (DBS):** Allis' line, solved Go-Moku (1993–94). DBS applied to Connect6 (first done for NCTU6) is hard *because* of two stones per move, but yielded 4.12× average / up to 50× speedup by decomposing independent threat regions. BUILD an adapted DBS/TSS: the primary weapon against the candidate explosion you already hit.
- **Conservative TSS (CTSS), Iterative TSS (ITSS), Conservative Relevance-Zone Search (CRZS):** CTSS performs conservative defense on single-threat positions to prove/disprove them; CRZS extends this and "is significantly more efficient than Relevance Zone Search (RZS) on positions for which TSS solutions are available in Connect6, and it can solve 100% of the tested puzzles." These make threat proofs *sound* against a two-stone defender. BUILD CTSS-style conservative defense; PROTOTYPE CRZS.
- **RZOP (relevance-zone-oriented proof search):** the general lambda-search-based method that "construct[s] and promot[es] relevance zones" and, with PN search, solved several Connect6 openings including Mickey Mouse. This is the highest-leverage *solver* technique and the state of the art for proving hex/Connect forcing wins; the relevance-zone machinery is what keeps a proof finite on an unbounded board. BUILD.

**Proof-number search family — the upgrade path for your existing AND-OR solver:**
- **PN → PN² → df-pn → Deep df-pn (+ 1+ε trick):** df-pn is the depth-first, TT-based PN variant; it suffers the "seesaw effect," which **Deep df-pn** mitigates by making unsolved-node (dis)proof numbers a function of depth. A relevance-zone-oriented Deep df-pn "worked quite efficiently" on Connect6.
- **Graph-history-interaction (GHI):** under pair moves and an unbounded board, transpositions with different histories can corrupt PN results; handle GHI explicitly (history-aware TT entries or the standard GHI-safe df-pn fixes).
- **Job-level PN (JL-PN) and parallel df-pn (SPDFPN):** JL-PN solved Connect6 openings on desktop grids with roughly linear speedups; SPDFPN (based on the 1+ε method) scales on shared memory. These matter for *offline* opening analysis, not the 0.5 s/move online budget.
- **Highest-leverage upgrade to your AND-OR solver:** move from plain PN/AND-OR to **relevance-zone-oriented Deep df-pn with the 1+ε trick and GHI handling**, fed by a threat-move generator. Biggest single solver upgrade; reuses your existing infrastructure.

**Quiescence via threat extensions at the horizon — avoiding the prior-experiment-2 blowup.** Your naive one-primitive threat-extension probe failed because it broadened the candidate set globally. The field's fix is to make horizon extensions *forcing-only and relevance-zone-bounded*: at the horizon, extend **only** moves that make/answer a threat, generated by the TSS/DBS generator, never the full candidate set. This is exactly the CTSS "conservative defense" pattern. BUILD threat-only, zone-bounded quiescence — the design that reconciles depth with the branching you already measured.

### C. Evaluation functions

**Handcrafted line-window pattern tables (3 axes).** Decompose the board into line windows along the 3 axes; index a codebook of window patterns; sum feature contributions; update incrementally on each placed stone. This is the classical gomoku eval and Rapfi's starting point. Because your board is unbounded, features must be **relative/windowed (no absolute cell identity)** — Rapfi uses length-11 line windows ("Each line segment has a length of 11, enabling it to capture the connection features of the surrounding five stones"), which works identically on a hex axis. BUILD this as the day-one eval and the fallback deterministic eval.

**Learned-but-cheap NNUE-style eval (the key recommendation).** Rapfi's "Mixnet": decompose the board into directional line patterns, train a small mapping network, **bake it into a pattern-indexed codebook** for O(1) lookup, and incrementally update only the affected windows per stone. Measured facts to transfer (all from arXiv:2503.13178):
- Rapfi's incremental codebook eval runs at **hundreds of thousands of alpha-beta nodes/sec** (Mixnet small/medium/large ≈ 428K / 257K / 104K nodes/sec, Table 1) versus **thousands or fewer** for a ResNet under the same alpha-beta traversal (ResNet 4b64f ≈ 1,758, 6b96f ≈ 484 nodes/sec) — i.e., the incremental pattern eval is **2–3 orders of magnitude faster per node** than a CNN. ("MixNet's inference throughput is orders of magnitude higher than that of ResNet.")
- Under MCTS with equal move time, "all MixNet configurations consistently outperform ResNet baselines by a significant margin of 300-400 ELOs, translating to a winning rate of 85%-92%" — the small cheap net is not just faster, it is *stronger per unit time*.
- Under the same net, "α-β search exhibits significantly higher playing strength [than MCTS] due to its depth-first traversal ... effectively leveraging the speed of our incremental update mechanism." (The exact α-β-vs-MCTS Elo gap on the same net is shown only graphically in the paper's Fig. 6, not stated numerically.)
- Adapting to 3 axes: use **3 directional mapping functions instead of 4** (one per hex axis) and length-11 windows along axial coordinates. Incremental update under 2-stone moves = apply the single-stone delta twice (at most ~3 axes × 11 window positions × 2 stones per turn).
- Training data: Rapfi distilled from "approximately 30.8 million positions" of Katagomo self-play, mixing 75% distillation labels with 25% true labels. You already have self-play and human corpora — feasible.
- Deterministic mode: the integer-quantized codebook (Rapfi clamps to [−16,16], 16-bit) is fully deterministic and SIMD-accelerated (AVX2 ≈ 4× speedup). This satisfies your deterministic-operation requirement natively — no GPU nondeterminism.

**GPU-vs-CPU leaf eval under alpha-beta (re-examined with GPU allowed, sub-second budget).** The batched-NN-under-DFS literature is clear that batching is awkward for alpha-beta: depth-first search exposes only a few leaves at a time and "progress in the search tree is blocked until heuristic evaluations are complete" (Sturtevant et al., arXiv:2507.11916). "Child batching" (evaluating a node's children in parallel on GPU) exists (Cohen-Solal; "Study and improvement of search algorithms," arXiv:2505.09639) and CPU-parallel-tree/GPU-parallel-heuristic frameworks help *some* search styles, but:
- Rapfi's headline (CPU-incremental beats GPU-CNN) was obtained under CPU-only constraints. With a GPU allowed, the calculus still favors CPU-incremental for *this* workload at 0.5 s/move, because the incremental codebook eval is sub-microsecond per node — a GPU round-trip (tens of µs to ms of latency plus batch-formation stall) cannot compete per-leaf, and alpha-beta's serial dependency prevents forming large batches without speculative over-evaluation that wastes the very pruning alpha-beta exists to provide.
- Verdict: **CPU-incremental NNUE for online play; GPU for offline training and (optionally) root/analysis parallelism.** PROTOTYPE a GPU child-batching leaf evaluator only as a research spike; keep it off the strength-critical path.

**Where the eval-vs-search tradeoff sits for this game class.** The Rapfi evidence says **deep+cheap wins** for the k-in-a-row family: a small incremental eval that keeps nodes/sec high, feeding a deep alpha-beta + forcing search, beats a smart+slow (CNN/MCTS) eval at equal compute. This is the opposite of Go. Build for throughput, then add just enough eval sophistication (via the distilled net) to fix tactical blind spots.

### D. Engineering for strength

**Zobrist hashing on unbounded coordinates.** You cannot pre-allocate a key table for an infinite board. Use **lazy per-cell key generation**: hash (q, r, color) via a splittable PRNG evaluated on demand and cached, XOR-combined incrementally as stones are placed. Include the **intra-turn phase bit** and side-to-move in the key. Zobrist keys are incrementally XOR-updated and engines routinely store only the hash, tolerating negligible collision risk with 64-bit keys; go **128-bit** here for proof-search soundness.

**12-fold symmetry canonicalization in the TT.** The hex point group has 12 elements (6 rotations × 2 reflections); with the origin-fixed first stone, translation is removed for opening theory. Canonicalizing to a representative before TT lookup gives up to ~12× effective TT density near the opening, but costs a canonicalization per probe and is *lossy* once the stone cloud is large and asymmetric. Verdict: PROTOTYPE canonicalization for the opening/near-empty region and the opening book; SKIP full-board canonicalization on the hot search path — the per-probe cost rarely pays off mid-game (mirroring why most chess/Go engines don't canonicalize TTs).

**Move-ordering stack (reported gains in comparable engines):** TT move → threat-making/blocking pairs → killers → history/countermove → quiet pairs by static pattern score. Well-ordered alpha-beta approaches the square-root node count of minimax; this ordering is where that comes from. Adapt killers/history to *pair* moves by keying on the completing stone and on the pair. BUILD.

**Parallel search (8–32 cores).** Empirical chess data (Kai Laskos / TalkChess; Chessprogramming wiki): Lazy SMP scales best on NPS and is far simpler; YBWC/DTS get better time-to-depth but are hard (Stockfish's own devs repeatedly failed at Lazy SMP before succeeding; it "widens" rather than deepening). For your branching profile and single-workstation target: **BUILD Lazy SMP first** (shared TT, threads at staggered depths) — most of the parallel Elo for the least engineering. PROTOTYPE ABDADA (recursive, easy, good speedups) if Lazy SMP under-scales past 16 cores. Keep YBWC as a later option. For the *solver*, use SPDFPN/parallel df-pn.

**Opening handling.** With origin-fixed first stone and 12-fold symmetry, the principled book is: canonicalize under the 12-element group, store solved/analyzed lines from your df-pn+RZOP solver (exactly how Connect6 openings like Mickey Mouse were catalogued and solved offline via JL-PN + RZOP). Generate **balanced opening books** for testing by enumerating symmetry-distinct k-turn openings and filtering to near-equal eval. BUILD a small book offline; it also supplies the paired opening set for SPRT.

### E. Complexity and realistic depth

**Complexity.** Square-board Connect(19,19,6,2,1): state-space ~10^172, game-tree complexity quoted ~10^140 (disputed; ~10^70 under an alternative assumption). Your unbounded board makes these formally infinite; the operative complexity is bounded by the radius-8 legal region and (much more tightly) by your search-candidate policy. The Connect(m,n,k,p,q) family and its fairness/complexity were formalized by Wu & Huang, "A New Family of k-in-a-row Games" (ACG 2005 / LNCS 4250, 2006); the p=2 stones-per-move term inflates game-tree complexity by the ~n(n−1)/2 pair factor per turn.

**Depth projection for THIS game (reasoned, with uncertainty).** No published engine runs this exact game, so this is a projection from gomoku/Connect6 engine behavior and Rapfi's measured throughput, adjusted for the pair-move branching. Assume a strong ordering stack (effective per-turn branching b_eff after pruning). Key uncertainty: b_eff per turn on 3 axes with threat-first ordering is unknown; I bracket it at 8–20 (a raw m²/2 factor collapses dramatically under cutoffs and dominance pruning, but stays above one-stone gomoku's).

| Node budget | Uniform depth (turns), b_eff≈8–20 | Notes / uncertainty |
|---|---|---|
| 1e6 | ~3–5 turns | Dominated by threat-gen overhead; wide error |
| 1e7 | ~4–6 turns | Deterministic fixed-node bar feasible here |
| 1e8 | ~5–8 turns | Matches your "6–8 turns refutes NN blind spots" prior |
| 1e9 | ~7–10 turns | Plus forcing lines much deeper |

Forcing (threat) lines reach far deeper than uniform depth: NCTU6's VCST solver reached **depth 25 (λ-path 13)** on square Connect6. Expect hex forcing lines of **15–30 turns** in sharp positions, at negligible node cost relative to uniform widening, *because* the threat generator keeps branching near 1–3.

**0.5 s/move row (deployment budget).** At Rapfi-class throughput (~2–4×10^5 nodes/sec/core for a small incremental eval) × 8–32 cores × ~0.5 s, with Lazy SMP efficiency ~0.4–0.7:
- **Without GPU (recommended):** e.g. ~4×10^5 × 16 cores × 0.5 s × 0.5 eff ≈ **~1.6×10^6–~5×10^6 effective nodes/move** → **~5–8 uniform turns + 15–30-turn forcing lines**. This is the design point.
- **With GPU-assisted eval:** *lower* effective online strength expected, because GPU leaf latency cuts nodes/sec despite better per-leaf eval; GPU helps only if you move to a much heavier net than the budget warrants. Treat as neutral-to-negative online.

**Fraction of strength from selective forcing depth vs uniform depth.** For sudden-death connection games the majority of decisive strength comes from **forcing-line depth** (winning/refuting threat sequences), with uniform depth providing positional judgment to steer toward those lines. Budget accordingly: the threat/df-pn solver is not an add-on, it is the primary strength engine; the eval+PVS is the steering.

### F. Strength measurement and comparison methodology

**Instruments (reproducibility first).** Provide three modes: fixed-depth (in TURNS), fixed-nodes, fixed-time. **All strength claims at fixed-depth or fixed-nodes**; wall-clock only for the 0.5 s deployment calibration, always paired with a fixed-node equivalent. Deterministic tie-breaking (stable move sort, fixed thread-reduction order) required; note Lazy SMP is nondeterministic across runs, so the deterministic bar must be **single-threaded or a deterministic-scheduling mode**, CPU-eval-only.

**Elo methodology.** Use **SPRT (GSPRT, as in Fishtest)** for accept/reject of each change, expressed in normalized Elo; games scale as ~640000/(normalized-Elo)². Report Elo with BayesElo/Ordo from the full game set, paired balanced opening books, color-balanced pairings. For ±20 Elo at 95%: on the order of low-thousands to ~10k games depending on draw rate (fewer if draw rate is low, which an infinite-board no-draw game favors). Use a fast-chess-style **pentanomial (paired-game)** manager, which finishes faster. **Deduplicate identical games in deterministic regimes: effective n = number of DISTINCT games**, or force diversity via the opening book and non-deterministic play for Elo runs.

**Matched-compute comparison vs your MCTS+NN (~150 sims/move).** "Equal compute" is ambiguous; report *both*: (1) equal wall-clock at 0.5 s/move on identical hardware, and (2) equal node/eval budget — since one MCTS sim ≈ one NN eval, ~150 sims/move ≈ ~150 leaf evals for the MCTS side, which the alpha-beta side will exceed by 4+ orders of magnitude per unit time using the cheap incremental eval. Report **per-side compute (nodes, NN evals, wall-clock, threads, hardware)** in every match table. The fair headline is the 0.5 s/move wall-clock match; the node-matched match will look lopsided in the classical bot's favor precisely because the incremental eval is so cheap — report both to avoid over-claiming.

**First-player advantage.** Connect6 was designed to be "potentially fair" (Wu, Huang & Chang, ICGA Journal 2005) and self-play showed near-equal win rates; the 1-then-2 stone structure is the fairness mechanism. On 3 axes the reduced fork density (fewer double-threats) plausibly *further* favors the defender, but this is untested — measure it. Sieben's hex (1,2)-achievement result (most shapes losers for the maker) supports a defender-leaning tilt. Consequence: use **paired openings played from both sides** and report first-player win rate explicitly; build the opening book to balance it.

### G. Known theory

- **Connect6 family definition, fairness, complexity:** Wu & Huang, "A New Family of k-in-a-row Games," ACG 2005 / LNCS 4250 (2006); Wu, Huang & Chang, "Connect6," ICGA Journal 28(4):234–242 (2005/2006). Connect(m,n,k,p,q) formalism; "potentially fair."
- **Hex/triangular lattice k-in-a-row and achievement games:** Bode & Harborth, "Hexagonal polyomino achievement," Discrete Math. 212 (2000) 5–18; Sieben, "Hexagonal polyomino weak (1,2)-achievement games," Acta Cybernetica 16(4):579–585 (2004) — *exactly* the (1,2) hex turn structure, all-but-one shape (P7) classified. Harary-style achievement/avoidance theory (Harary 1982; Harary & Harborth). These are achievement games (form a fixed shape), a cousin — not identical — to your connect-6 goal; directional evidence of defender advantage, not exact win/loss.
- **Connect6 solver state of the art (as of 2026):** RZOP + PN/df-pn (Wu & Lin, IEEE TCIAIG 2(3):191–207, 2010) solved openings incl. Mickey Mouse; JL-PN (CG 2010, LNCS 6515) and job-level alpha-beta (IEEE TCIAIG, 2014) extended opening analysis; TD-learning improved NCTU6 (best version ~58% win rate vs original). No claim that full Connect6 is solved from the initial position; specific openings are solved. Modern gomoku SOTA is Rapfi (alpha-beta + distilled NNUE, GomoCup 2024 winner) and Katagomo (AlphaZero-style); Rapfi is the strongest CPU engine.

---

## Deliverable 1 — Technique Verdict Table

| Technique | What it is | Measured evidence (comparable games) | Transfer risk to hex/3-axes | Rust cost (person-days) | Verdict |
|---|---|---|---|---|---|
| PVS/NegaScout + ID + TT + aspiration | Core alpha-beta framework | Universal in Yixin/Embryo/Rapfi | Low (game-agnostic) | 10–15 | **BUILD** |
| Two-plies-internally + intra-turn key bit | Pair-move as two same-side plies; score in turns | Connect6 engines handle 2-stone turns; DBS-for-Connect6 | Medium (correctness-critical) | 8–12 | **BUILD** |
| Threat-first staged/lazy pair gen + dominance pruning | Order & prune the m²/2 pairs | NCTU6 threat-move gen; DBS 4.12×–50× speedup | Medium (threat defs change) | 12–20 | **BUILD** |
| Incremental pattern-codebook eval (NNUE-lite, 3 axes) | Distilled small net baked to codebook, incremental update | Rapfi: 428K/257K/104K nps; Mixnet +300–400 Elo vs ResNet; α-β+Mixnet ≈ +400 Elo vs Katagomo (CPU) | Medium (retrain, 3 directional maps) | 25–40 (+training) | **BUILD** |
| Handcrafted line-window eval (fallback/deterministic) | Classic pattern-table eval | Standard gomoku eval | Low | 8–12 | **BUILD** |
| TSS/DBS (dependency-based threat search) | Decompose independent threat regions | Solved Go-Moku; NCTU6 DBS 4.12×–50× | Medium-high (threat geometry) | 20–30 | **BUILD** |
| CTSS / conservative defense; CRZS | Sound proof/disproof vs 2-stone defender | CRZS solved 100% of Connect6 puzzle set | Medium-high | 15–25 | **BUILD (CTSS) / PROTOTYPE (CRZS)** |
| RZOP relevance-zone proof search | Lambda-search zone construction | Solved Connect6 openings incl. Mickey Mouse | Medium (zones re-derived) | 20–35 | **BUILD** |
| df-pn / Deep df-pn (+1+ε) + GHI (upgrade AND-OR solver) | Depth-first PN, seesaw-reduced | Deep df-pn "efficient" on Connect6 | Low-medium | 15–25 (reuses solver) | **BUILD** |
| Threat-only, zone-bounded quiescence | Horizon extension on forcing moves only | CTSS conservative-defense pattern | Medium | 8–12 | **BUILD** (fixes prior exp. 2) |
| Null-move (guarded) / verified null-move | Zugzwang-free ⇒ sound; guard vs double-threats | Gomoku engines use it; RZOP seminull concepts | Medium (sudden-death risk) | 4–6 | **PROTOTYPE** |
| LMR / futility / razoring | Depth reductions | Rapfi lists these | Medium (margins change) | 6–10 | **PROTOTYPE** |
| Lazy SMP | Simple shared-TT parallelism | Best NPS scaling in chess | Low | 6–10 | **BUILD** |
| ABDADA | Recursive parallel alpha-beta | Good speedups, easy | Low | 5–8 | **PROTOTYPE (if Lazy SMP under-scales)** |
| YBWC/DTS | Better time-to-depth, hard | Stockfish/Zappa data | Medium (impl. risk) | 20–30 | **SKIP initially** |
| Lazy per-cell Zobrist + 128-bit keys | Hashing on unbounded coords | Standard Zobrist practice | Low | 3–5 | **BUILD** |
| 12-fold symmetry TT canonicalization | Fold symmetric positions | Symmetry reduction in solvers | High (only wins near-empty) | 5–8 | **PROTOTYPE (opening only)** |
| GPU batched leaf eval under alpha-beta | Child-batching NN on GPU | Child-batching exists; DFS batching awkward | High (latency/stall) | 15–25 | **SKIP (online) / PROTOTYPE (research)** |
| ProbCut / multi-cut | Speculative forward pruning | No Connect6 evidence | High | 8–12 | **SKIP initially** |
| JL-PN / SPDFPN (parallel solver) | Distributed/parallel PN, offline | JL-PN solved Connect6 openings, ~linear speedup | Low-medium | 15–25 | **PROTOTYPE (offline book only)** |
| Opening book (solved lines, canonicalized) | Symmetry-folded book from solver | Connect6 opening catalogues | Medium | 8–12 | **BUILD (small, offline)** |

## Deliverable 2 — Prioritized Build Plan (0.5 s/move design point)

Cumulative brackets are *relative to the previous stage* and are engineering estimates, not measured Elo (measure each with SPRT).

**Stage 0 — Foundations (≈3–4 weeks).** Board model on axial (q,r) with radius-8 legal region; lazy 128-bit Zobrist with intra-turn phase bit; move generator (pair-moves via two internal plies); handcrafted 3-axis line-window eval; PVS + iterative deepening + TT + basic ordering; **deterministic fixed-depth (in turns) and fixed-node modes**. *Deliverable: a legal, correct, reproducible engine.* Baseline strength.

**Stage 1 — Tactical core (≈3–4 weeks). +large.** Threat-first staged pair generation + dominance pruning; killers/history/countermove on pair moves; threat-only zone-bounded quiescence; upgrade the AND-OR solver to relevance-zone Deep df-pn (+1+ε, GHI). *Where the engine starts refuting the tactical class your NN misjudges (your 6–8 turn prior).*

**Stage 2 — Cheap learned eval (≈4–6 weeks + training). +large.** Distill a Rapfi-style incremental pattern-codebook net (3 directional maps, length-11 axial windows), integer-quantized + SIMD, incremental update under 2-stone moves; train on your self-play + human corpora with distillation. *Expected the biggest single Elo jump, per Rapfi's +300–400 Elo vs ResNet and ~400 Elo vs Katagomo (CPU).*

**Stage 3 — Forcing-search depth (≈4–5 weeks). +medium-large.** Full TSS/DBS with independent-region decomposition; CTSS conservative defense; RZOP relevance zones integrated into online search for VCDT/VCST detection. *Extends decisive forcing lines to 15–30 turns.*

**Stage 4 — Parallelism & tuning (≈3–4 weeks). +medium.** Lazy SMP (8–32 cores); SPSA/Texel tuning of eval and search margins; PROTOTYPE guarded null-move, LMR, ABDADA and keep only SPRT-positive ones.

**Stage 5 — Opening book & test harness (≈2–3 weeks). +small-medium.** Offline JL-PN/SPDFPN solving of symmetry-distinct openings; canonicalized book; balanced-opening generator; full SPRT/BayesElo harness.

## Deliverable 3 — Depth/Strength Projection Table

| Budget | Uniform depth (turns) | Forcing-line depth | Rough strength bracket | Uncertainty |
|---|---|---|---|---|
| 1e6 nodes | 3–5 | 10–20 turns in sharp lines | Refutes shallow blunders | High (b_eff unknown) |
| 1e7 nodes | 4–6 | 15–25 | Solid club-level tactics | High |
| 1e8 nodes | 5–8 | 15–30 | Refutes the NN's tactical blind spots | Medium |
| 1e9 nodes | 7–10 | 20–30+ | Strong; near solver-grade in forcing positions | Medium |
| **0.5 s/move, 16 cores, no GPU** | **~5–8** | **15–30** | **Deployment target: beats NN+MCTS at equal wall-clock (per Rapfi analog)** | Medium |
| 0.5 s/move, 16 cores, +GPU eval | ~4–7 | 15–30 | Neutral-to-worse online than CPU | Medium-high |

Sources for projection: Rapfi throughput (428K–104K nps small–large, CPU incremental); NCTU6 VCST depth 25/λ-13; chess Lazy SMP scaling (~0.4–0.7 efficiency to 8–16 cores); well-ordered alpha-beta near-√ node count. All depth-in-turns numbers are projections, not measurements on this game.

## Deliverable 4 — Testing Protocol (ready to adopt)

- **Instruments:** fixed-depth (turns), fixed-nodes, fixed-time. Publish strength at fixed-node bars; wall-clock only for the 0.5 s calibration row, always paired with a fixed-node equivalent. Deterministic mode = single-thread or deterministic scheduling, stable tie-breaks, CPU-eval-only.
- **Depth ladder (secondary, low-effort):** expose D2/D4/D6/… fixed-turn-depth opponents (falls out of the deterministic fixed-depth mode). Use as a cheap internal calibration ladder; do not over-invest.
- **Opening book:** enumerate symmetry-distinct openings (12-fold group, origin-fixed first stone), filter to |eval| below a balance threshold, play each from both sides (paired).
- **SPRT:** GSPRT, normalized-Elo bounds e.g. elo0=0, elo1=3–5, α=β=0.05; games ≈ 640000/(norm-Elo)². Pentanomial (paired-game) manager.
- **Sample size:** ±20 Elo at 95% ≈ low-thousands to ~10k distinct games depending on draw rate; in deterministic regimes count **effective n = distinct games only**.
- **Matched-compute vs MCTS+NN:** report BOTH (a) equal 0.5 s wall-clock on identical hardware and (b) node/eval-matched (~150 evals for the MCTS side). Report per side: nodes, NN evals, wall-clock, threads, hardware, opening/color.
- **Reporting fields:** engine hashes, net hash, budget mode + value, hardware, thread count, book, n distinct, W/D/L, pentanomial, Elo ± CI, normalized Elo, LLR, first-player win rate.
- **Report conflicts explicitly** (e.g., the 10^140 vs 10^70 game-tree-complexity disagreement; do not average).

---

## Recommendations

1. **Commit to the Rapfi-analog architecture now** (alpha-beta PVS + incremental pattern-codebook eval + threat/df-pn forcing search). It is the only published approach that beat NN+MCTS at CPU-equal compute in this exact game family. Build Stages 0–1 first for a reproducible, tactically sound baseline, then Stage 2 (the learned eval) for the biggest Elo jump.
2. **Keep online evaluation on the CPU.** Use the GPU for offline distillation/training and optional offline opening solving only. Revisit only if you later adopt a much heavier net; the threshold is if a GPU leaf eval delivers enough eval-quality Elo to outweigh a ≥2× nodes/sec loss — measure with a node-matched SPRT.
3. **Treat the threat solver as core, not an add-on.** Most decisive strength in sudden-death connection games is forcing-line depth. Upgrade your existing AND-OR solver to relevance-zone Deep df-pn and wire it into online search for VCDT/VCST detection.
4. **Re-derive every threat number for 3 axes from scratch.** Do not import Connect6 threat counts, relevance-zone radii, or win densities. Instrument the engine to *measure* effective per-turn branching and forking density early — these drive every depth projection above and are your largest uncertainty.
5. **Fix the search-candidate policy empirically, separate from the radius-8 legal region.** Start with a tight threat-relevant candidate set and widen only via staged generation; your prior experiment already proved brute widening loses. Benchmark candidate-radius/threat-filter variants with fixed-node SPRT.
6. **Stand up the deterministic fixed-node harness in Stage 0**, before any strength claims. It is cheap once fixed-depth exists and is a hard requirement for every number you publish.
7. **Benchmarks that change the plan:** if measured per-turn b_eff exceeds ~20 even with threat ordering, uniform depth will be shallower than projected → lean harder on the forcing solver and more aggressive dominance pruning. If Lazy SMP efficiency falls below ~0.4 at 16 cores, switch to ABDADA. If the distilled net fails to clear +150 Elo over the handcrafted eval in node-matched SPRT, keep the handcrafted eval for determinism and invest the eval budget in search.

---

## Caveats

- **Every Rapfi/Connect6/NCTU6 number is square-board (4-axis).** They validate *algorithms and architecture*, not magnitudes. Note also that the paper reports two distinct ~400-Elo figures: α-β+Mixnet ≈ +400 Elo above Katagomo (CPU), and Mixnet +300–400 Elo (85–92% win rate) vs ResNet under MCTS. The *direction* (cheap incremental eval + alpha-beta beats NN+MCTS at equal CPU compute) is what transfers, not the exact Elo.
- **Depth-in-turns projections are extrapolations, not measurements** on this game; the effective per-turn branching factor on 3 axes with threat ordering is unknown and is the dominant uncertainty. Instrument and re-derive early.
- **Game-tree complexity is disputed** (~10^140 vs ~10^70 for square Connect6), and both are Go-board figures — your unbounded board is formally infinite, bounded in practice only by your candidate policy. Reported explicitly rather than averaged.
- **Hex (1,2)-achievement results are achievement games** (form a fixed polyomino), a cousin of your maker-maker connect goal, not identical; use as directional evidence of defender advantage, not as a win/loss verdict for hex Connect(6,2,1).
- **Fairness on 3 axes is untested.** Connect6 is "potentially fair" on 4 axes; the reduced fork density on 3 axes plausibly tilts toward the defender, but you must measure first-player win rate directly and balance the opening book accordingly.
- **Null-move and forward-pruning safety are not guaranteed** in sudden-death threat races; gate every such heuristic behind a threat-verification guard and SPRT, and keep a deterministic no-pruning mode for verification.
- **GPU nondeterminism and Lazy SMP nondeterminism** both threaten reproducibility; the deterministic bar must be single-threaded or deterministically scheduled and CPU-eval-only.
