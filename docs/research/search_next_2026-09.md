# Search work after a new eval — hex Connect(6,2,1), 2026-09

**What this is.** A read-only research input. It does **not** repeat
`docs/audit/search_gap_2026-09.md`, which already inventories C-1…C-9 against `d83ac01`
and ranks them; this answers the four questions that audit did not: *why* depth hurt,
what width control means for a **two-stone** turn, what remains in the forcing layer,
and what actually causes D-534's overshoot. **It schedules nothing.**

**Labels.** `MEASURED` (instrument named) · `DERIVED` · `ESTIMATED` · `ARCH`
(architectural literature fact — transfers) · `EMP` (empirical figure, tagged with its
game/board, **NON-TRANSFERABLE**). Repository claims carry `file:line`.

## §1 Why "more depth made it worse", and which parts are diagnosable today

### 1.1 The literature names eight mechanisms; five could apply here

| # | mechanism | primary source | kind | applies? |
|---|---|---|---|---|
| M1 | **value collapse** at high `b`: with a bounded-dependence eval the root values tie and move choice becomes arbitrary | Nau 1983, Thm 3.1 | THEORY | **plausibly** — pair moves give a very high `b` |
| M2 | **low eval granularity** — coarse eval × high `b` is the pathological corner | Nau, Luštrek, Parker, Bratko & Gams, *AI* 174 (2010) 1323-1338 | THEORY + EMP (kalah, chess endgames) | **measured below, and it does NOT hold** |
| M3 | low local similarity between siblings | same | THEORY | **no** — one stone changes a board incrementally, the *protective* regime |
| M4 | noise amplification: eval error must fall > 50 % per ply or minimax amplifies it | Pearl 1983 (via Nau 1983) | THEORY | in principle; needs an eval-error model nobody has |
| M5 | **no traps inside the horizon** — sudden-death terminals kill pathology, *if searched to* | Pearl 1983 | THEORY | **yes, and it cuts against us**: at median depth 2 turns this engine sees essentially no terminal nodes, so it gets none of the protection |
| M6 | **positive horizon effect** — commit to a threat too early, inside the horizon, rather than when it is unanswerable | Berliner 1973 | ARCH | **yes** — and with `q_depth_turns = 0` (D-428) there is no threat quiescence to prevent it |
| M7 | **short games under-sample the deeper engine's edge** | Junghanns, Schaeffer, Brockington, Björnsson & Marsland, *Advances in Computer Chess 8* (1997) | EMP (chess) | **yes** — sudden death scored in TURNS is a short game |
| M8 | depth bought by narrowing is not a depth experiment | methodological | — | **already discharged — see 1.2** |
| **M9** | **type-D nodes** — where *every* move loses — are the **sole** amplifier of eval error | Zuckerman, Wilson & Nau, *Comput. Intell.* 34(2), 2018, DOI 10.1111/coin.12162 | THEORY + EMP | **yes, and it is the sharpest and the only one testable from games already played — §1.5** |

Two literature notes that matter more than they look. Junghanns et al., verbatim: *"The
longer the game, the greater the chance that the shallower searching program will make
an error… In a short game, chances are that the 8-ply program will not make any
mistakes."* They had to *artificially shorten* chess games before diminishing returns
appeared, and **this game is already short** — the regime where an extra turn's
advantage does not convert into a result. And Nau et al.'s remedy for the pathological
corner is **not "search less"** but **raise granularity**, with *"the granularity
required to avoid lookahead pathology increases with the branching factor"* — an
independent literature route to where D-609/D-610 arrived by measurement.

### 1.2 M8 is not an open question here

D-610 already records it in the ruling's own text: the 8-turn seat reached its depth
*with* a Tier-T width cap and root re-ordering, *"so the two extra turns were bought
partly by looking at fewer moves"*. The disambiguation is also already on record: **S2's
arm carries no width change, reached one whole turn LESS on the same nodes, and still
lost 8 W / 61 L** (D-605). A successor should cite D-610 rather than re-derive the
confound.

### 1.3 M2 is MEASURED, and the answer is that it is not the cause

Nau et al. define granularity `g` as *"the size of [the evaluation function's] range,
i.e. the number of different values that it can return"*. Instrument:
`artifacts/research_2026-09/granularity.txt` (receipted, `training_pipeline_2026-09.md`
§9) — the committed v0 score over the corpus's own extracted rows, through the linear
form `tools/texel/features.py` pins against the engine.

| population | rows | **distinct static values (`g`)** | static range | distinct label values |
|---|---|---|---|---|
| all `eval` rows | 74 672 | **1 877** | −5 210 … 776 | 3 339 |
| quiet (R6) rows | 45 271 | **790** | −1 330 … 690 | 2 676 |

**The literature's measured pathological region is `g ≤ 10` at `b ≈ 13–17` (KBBK, KQKR)
and `b = 4–6` (kalah)** `[MEASURED-ON: chess endgames, kalah]` **EMP**, and this engine's
`g` is two orders of magnitude above it. Its `b` is far larger too, and Nau et al. say
the required `g` rises with `b` — but **extrapolating their surface across two orders of
magnitude in `b` is not licensed, and this measurement does not place the engine in the
region they measured.** M2 is *not established* as the cause, and "the eval is too
coarse, hence pathology" is unsupported by the one dial checkable cheaply. What the run
does show is **concentration**: 90 % of quiet rows fall in **275** distinct static
values, the commonest (−36) covering **3 037 of 45 271** (6.7 %). Suggestive of M1, not
a test of it — M1 is about ties **among siblings at a node**, this is a distribution over
*corpus positions*, and **the sibling-tie count is the actual test.**

### 1.4 What is diagnosable from artifacts that already exist, and how

| mechanism | instrument that already exists | what to compute | cost |
|---|---|---|---|
| **M1** | instrument mode + the golden seat, `configs/instrument_v0.toml` | at fixed depths 1…6 turns over the 24 governed openings, the count of root moves tied at the best score, and its trend in depth. M1 predicts the tie count rising with depth | minutes; **needs the root scores on the wire**, which `SearchInfo` does not expose per move — that is the one gap |
| **M2** | none needed — done above | — | done |
| **M6** | `tools/wp15b_attribution_check.py`, `tools/wp16_warm_attribution_check.py` (warm-replay attribution, already used by WP-1.5b/1.6) re-run on the combination SPRT's own games | classify the deeper seat's losses by whether the losing turn placed a threat-completing stone that the shallower seat delayed | hours, no new code |
| **M7** | `artifacts/opt_arc_combo_sprt_v1.txt` (`sha256 8c64e8fd…`), `s2_sprt_v1.txt`, `s3_sprt_v1.txt` | the pentanomials are already recorded: combo **p0 82 / p1 77 / p2 180 / p3 67 / p4 49** at n = 910 with **204 capped**; S3 **p0 73 / p1 102 / p2 273 / p3 92 / p4 60**. A short-game/under-sampling story predicts a large `p2` mass and a high capped rate — both are present (S3: 273 of 600 pairs split 1-1) | free, already recorded |
| **M9** | the same three SPRT receipts, plus the root score the seat already reports | partition pairs by root-eval sign; compare the deeper arm's loss rate in behind-positions against ahead-positions | hours, no new games — **§1.5** |
| **M4, M5** | — | not diagnosable without an eval-error model / a deeper reference engine | — |

**The cheapest decisive experiment is M1's**, and its only obstacle is that root move
scores are not reported. That is a `SearchInfo` field, not a search change.

### 1.5 The sharpest mechanism, and it is testable from games already played

**Zuckerman, Wilson & Nau (2018)** narrow "pathology" from a property of a game to a
property of a NODE. With a static eval correct with probability `1 − e`, at branching
factor 2 there are four node types, with depth-1 minimax error at the parent:

| type | children | error |
|---|---|---|
| A | win, win | `e²` |
| B / C | win, loss | `e(1 − e)` |
| **D** | **loss, loss** | **`1 − (1 − e)²`** |

with the ordering `error(D) ⩾ e ⩾ error(B) ⩾ error(A)` *"for any error e ∈ [0, 0.5]"*.
Verbatim:

> "Only in type-D nodes is the error at the root greater than the error at the leaves, and, since any
> depth-d search can be seen as a combination of d depth-1 searches, we can conclude that **type-D
> nodes are the source of search pathology.**"

**A type-D node is one where the side to move is already lost**, and under two-stone
turns those are not exotic: once the opponent holds a live double threat (LAW-OVERLOAD's
`t ≥ 3`), a whole swath of the subtree is type-D, and **depth spent inside it amplifies
eval error instead of reducing it.** The paper's own caution: *"This is not to say that
any time one reaches a type-D node, a shallower search should be preferred — it may be
that each child of a type-D node is a type-A node."*

**THE DIAGNOSTIC NEEDS NO NEW GAMES.** Partition the combination SPRT's 910 pairs by
whether the root evaluation says the mover is behind, and check whether the deeper arm's
loss concentrates in the losing-side positions. M9 predicts it does, M7 predicts it does
not — **the one test here that discriminates between two live mechanisms**, on games
already recorded.

Its remedy inverts the usual instinct and is worth naming: **Error-Minimising Minimax
propagates a value *and* an error bound and cuts off — substituting the static value —
wherever the propagated error exceeds the eval's own**, searching *shallower* in the
pathological regions (MEAS, NON-TRANSFERABLE: Pearl board-splitting game, Kalah). And
one caveat against the tempting escape: Nguyen & Ramanujan (ICAPS 2024,
arXiv:2212.05208) find **UCT pathological in the same game family, and their Appendix J
finds αβ so too** — **switching paradigm escapes nothing**, and the older trap
literature is not an MCTS-only weakness.

## §2 Width control for a two-stone turn

### 2.1 The measured shape this must fit

`docs/experiments/opt_arc_ledger.md` §I1 (**MEASURED**): BATCHED emitted mean 17.1 /
median 12; **Tier T's own union median 12**; **the quiet ball mean 76.2 / median 76, on
1 187 of 21 746 BATCHED nodes** — i.e. the fat set is reached on 5.5 % of nodes yet
carries ~24 % of expanded cells. **The width is in the tail, not the body.** W1's cap at
K = 16 binds 23.3 % of BATCHED nodes, removes 11.2 % of Tier-T cells, and changed **0 of
24** openings — a cap on the body is a cap on nothing. D-613 orders the quiet-ball cap
after Phase 1 for the right reason: which moves are safe to discard is a property of the
eval that ranks them.

### 2.2 The compound-turn literature, and it contradicts a standing decision

The closest prior art is **Arimaa**, four steps per turn. David J. Wu, *Designing a
Winning Arimaa Program*, **ICGA Journal, March 2015, 19-40** (SHARP, 2015 Arimaa
Challenge winner), §5.4 states both formulations and the failure of the per-step one,
verbatim (**ARCH**):

> "Since alpha-beta search is depth-first, it reduces the effectiveness of move
> ordering by forcing the search to consider moves that begin with the same steps
> together… in the event that the search first explores Ee6s… and the move turns
> out not to be good, it will be forced to then uselessly explore every other move
> in the subtree beginning with Ee6s before being allowed to try better
> alternatives."

SHARP's answer is a **hybrid move list**: at each node it *"contains a mix of moves
ranging from one to four steps"*, with **TT and killer moves always recorded with the
full four steps** and capture generators emitting *"the entire capture sequence rather
than merely the step that begins"* it; single-step entries are depth-reduced, and *"this
reduction occurs at each level of recursion"*. `[MEASURED-ON: Arimaa 8×8]` **EMP**: the
stepwise→hybrid switch alone was *"about a 20 % speedup for 9-step searches and much
more for deeper searches"*, and tactical move generators *"immediately gained 80 Elo…
the largest gain any single change to the search has produced in the last several
years"* while *"capturing about 97 % of the evaluation difference between expert moves
and passing… while only generating about 3 % of all of the legal moves"*.

**And SHARP lands in this engine's depth regime.** At 60-120 s per move with `b ≈ 16
000` it reaches only **~3 turns**, against 2 turns at 0.5 s here — and **its ~600 Elo of
search gain came from width control, not depth** **EMP**: an independent instance of
D-609's conclusion in the other well-documented compound-turn game.

**Footnote 16 is the safety valve and it transfers exactly:** *"In the presence of a
goal threat… no reduction in depth is performed."* Reductions switch off when a terminal
threat is live. pistol's analogue is already computed at every node — the FILTERED /
WIN-NOW classification.

**The tension to put in front of the architect.** D-111 forbids a node's *answer*
mid-turn — *"a mover who still owes a stone has not finished doing anything"* — and
S2/S3 honoured it by reducing in whole TURNS, which at six turns is a third of the
remaining depth (D-607). SHARP's hybrid does not violate D-111's letter: it never
returns a static value mid-turn, it *orders* whole pairs first and lets the residue be
explored one stone at a time with a depth debit. **Whether that is admissible here is an
ADR question**, and it is the only published route out of "whole-turn reductions are too
coarse" — D-111's own text already draws the distinction, `Eval::value` being read at
phase-1 nodes for *ordering* where *"neither read is an answer"*.

### 2.3 Which pruning device generalises to a game with no material

**ARCH:** futility prunes on a *value* bound (`eval + margin ≤ alpha`); move-count
pruning prunes on an *ordinal* bound (index in the move list beyond a depth-dependent
count). **This game has no piece values, so a futility margin has no natural unit; a
move-count bound needs only an ordering.** That is an argument for C-2/C-3 over C-5/C-6
that does not depend on the eval, and it survives Stage 2. (Attributions: futility —
Schaeffer 1986 thesis; extended futility — Heinz, *ICCA Journal* 21(2), 1998; move-count
pruning has no single originator. LMR likewise *"has no clean single-author origin"*,
popularised via Fruit/Glaurung in 2005.)

Aspiration widening, verbatim from the same lineage (Kaindl, Shams & Horacek, *IEEE
TPAMI* 13(12), 1991): **only the bound that failed is widened**, the other kept, and
modern engines widen the failing bound exponentially. The audit's C-1 risk stands — mate
scores must widen straight to full width.

### 2.4 What a design here must test, and it is not what chess tests

1. **Colony blindness** — a cap that drops a distant defensive cell loses to an
   overline-capable double threat the FILTERED row would have caught a turn later
   (`docs/research/sealbot_notes.md`, "What NOT to copy"; sealbot's root "colony" cell
   at centroid distance `max_r + 3` patches exactly this).
2. **Overline doubles** — wins at 7+ are legal, so a "quiet" stone can complete a seven;
   only the FILTERED row protects against that, and it must be exempt from every cap and
   every reduction. **The safety valve is SHARP's footnote 16 generalised**: no
   reduction and no cap on any row where a terminal threat is live.
3. **A tie-count instrument** (§1.4 M1), because a cap that removes tied-value siblings
   is invisible to a bestmove-diff sensitivity check — exactly how W1 measured "0 of 24"
   while binding 23.3 % of nodes.

## §3 Forcing search — most of it is built; what is missing is a schedule and a budget

### 3.1 What already exists in `crates/pistol-solver`

Relevance zones with graded orders (`zone.rs:25-102`, `add_graded`, `union_with`,
`is_subset_of`, `cell_within_reach`) — the RZOP order-≤3 machinery
`threat_calculus_v1.md` ZONE-R names; df-pn with the **1+ε** loosening (`dfpn.rs:84`,
`:344`, `:508`); a proof DAG carrying a zone per node (`dfpn.rs:40-57`); threat-pair and
blocking-pair generation and the overload test (`policy.rs:54,190,290`); exact threat
number by minimum hitting set (RULE-EXACT). **Deep df-pn, RZOP zones and 1+ε are not
proposals here — they are in the tree.**

### 3.2 The binding constraint is the call budget, and it is measured

`docs/experiments/stage3_detector_CLOSURE.md`: because search and solver *"spend from
the same pot of nodes"*, dropped calls hand their nodes back to the search, which then
wants to call again — so the gate-on seat *"currently makes about 24 expensive solver
calls per search and needs to make about 2"*, and a detector must be **~380× more
selective**, not 6×. D-620 licensed detector round 3 on the census (118 classes, 248
keys, **63 roots against 28**), and D-624 still schedules it after Stage 2. Nothing in
this document reorders that.

### 3.3 A citation correction the ADR log should carry

`docs/research/minimax_report.md:138` and `threat_calculus_v1.md` §9 treat **CTSS** as
an established technique with a citable source. **VCDT and VCST are real and published**
— VCDT in Wu & Lin's RZOP paper (*IEEE TCIAIG* 2(3):191-207, 2010, Fig. 6) and in Zhang,
Huang, Zhang & Liu, *2018 CCDC*, DOI `10.1109/ccdc.2018.8408300`; VCST in Wu, Su, Li,
Zhang & Zhou, *2021 33rd CCDC*, DOI `10.1109/ccdc52312.2021.9601901`. **"CTSS" could not
be traced to a defining paper**: it appears as prior work in Yang & Yen's CRZS paper
(TAAI 2011, DOI `10.1109/taai.2011.65`) and nowhere that defines it. The *property* —
**conservative defence: a search may reject a real win but must never accept a false
one** — has a clean primary citation in **Allis 1994 §5.3.3**. Cite the property, not
the acronym. Likewise `CRZS "solves 100 % of tested puzzles"` (quoted at
`minimax_report.md:138`) is traceable only to a secondary summary and should be marked
UNVERIFIED until a PDF is read.

### 3.4 DBS and the pair rule — the literature says the compound turn is the hard part

Wu, Kang, Lin, Lin, Wei & Chang, *Dependency-Based Search for Connect6*, CG 2013, LNCS,
DOI `10.1007/978-3-319-09165-5_1`, abstract verbatim: *"**Unfortunately, the rule that
two pieces are played per move in Connect6 makes DBS extremely difficult to apply to
Connect6 programs.** This paper is the first attempt…"* — eight years after the game was
defined — with a measured speedup of **4.12× average, up to 50× on hard positions**
`[MEASURED-ON: Connect6 19×19]` **EMP, NON-TRANSFERABLE**. `threat_calculus_v1.md`
LAW-DECOMP already states the licence (disjoint stars ⇒ `t` additive, proofs decompose);
what the literature adds is that **the pair rule is what breaks the dependency
relation**, so a hex DBS is a package, not a port. The hex independence rate is unknown
and ADOPT-DBS says so.

### 3.5 Hex is the other sudden-death lineage, and it says domain knowledge dominates

Hex shares what matters — sudden death, no captures, monotone accumulation, a hexagonal
lattice — and differs in the win condition and the single-stone turn. Two numbers from
it outrank everything else here. **MEAS, NON-TRANSFERABLE (Hex 6×6, Expected Work Search
— Randall, Müller, Wei & Hayward, arXiv:2405.05594, IJCAI-24 pp. 7003-7011):** solving
the board took **93 963 192 nodes with no domain knowledge and 26 nodes with virtual
connections plus inferior-cell fill-in.** That is a ~3.6-million-fold reduction. **Every
algorithmic delta anywhere in this review is a single- to low-double-digit multiple** —
DBS 4.12×, the hybrid turn +20 %, mustplay 6.6×. **A threat calculus is worth more than
any pruning package, by orders of magnitude, and this is the number that says so.**

**Mustplay** — the intersection of the opponent's threat carriers — is Hex's FILTERED
row, and the largest single lever measured there: alone it cuts search to **6 % of time
and 2 % of nodes**, and removing it costs **6.6× time and 25× nodes**, beating the
transposition table, decompositions and symmetry combined. An **empty** mustplay is a
proven loss without search — LAW-OVERLOAD by another name. Derived, sound, free, and
*distinct* from a relevance zone: this engine needs both.

One geometric note: **Havannah is a hexagonal board of hexagons**, the closest published
geometry to this lattice, and its studied technique is **decisive / anti-decisive move
testing** — win-now and block-win-now at every node, which `PROTO-NODE` steps 1-2 are.

### 3.6 Allis's conservative reply is SOUND here, and the reason is a rule of this game

The transformation that makes threat search tractable — forcing the defender to answer a
multiple-stone reply with a single stone — rests on one premise, verbatim: *"Clearly, in
free-style go-moku, having extra stones on the board is never a disadvantage. Thus, if a
variation wins for the attacker when the defender is allowed to play replies consisting
of multiple stones, then the variation wins also if the defender is forced to select one
stone from each multiple-stone reply."*

**That premise is `threat_calculus_v1.md`'s LEM-MONO, and it holds here because
overlines win and there are no bans** — it would **not** hold under a renju-style ban,
where an extra stone can lose. A conservative two-stone-to-one-stone reduction is
therefore a *sound* device here, not a heuristic: a rules-level fact worth an ADR line
rather than a rediscovery.

**The price is quantified by Allis himself:** TSS is roughly **an order of magnitude
cheaper in nodes** and **fails to find a win in 3 of 12 test positions**, his own caveat
being that there *"the defending player has almost no interrupting threats"*. The
incompleteness direction is his invariant: **may miss a win, never claim a false one.**

## §4 Time management — D-534's cause is neither named remedy

**Both remedies commonly proposed are already in the tree.** (1) **Abort granularity is
already right** — the deadline *"is not masked at all… tested at every abortable node,
and inside the move-ordering scoring loop every `ORDER_CHECK_INTERVAL` cells"*
(`crates/pistol-search/src/stop.rs:37-41`); the 1024-node mask applies to NODE budgets
only (`stop.rs:42`). (2) **Partial-iteration harvest already exists** —
`Provenance::PartialRoot` (`crates/pistol-search/src/info.rs:334-340`, produced at
`search.rs:560`): *"the move was fully searched at one turn DEEPER than `depth_turns`
inside the aborted iteration, its score exact for that move there — a lower bound"*,
which is **exactly Stockfish's rule** that a root move updates its score and PV only if
its entire subtree completed (`search.cpp:1434`, `:1470-1505`, **ARCH**).

**So what remains is the solver's call.** `stop.rs:17-26` states it: with the solver on
the search path *"its nodes are absorbed in whole calls, so the stopping point is not a
multiple of anything"*. D-520 is consistent — 240 answers at a 500 ms budget had a
**median of 1 225 ms and a maximum of 1 866 ms** with the solver armed, against an **8
ms** maximum overshoot with the gates off (D-534). **The overshoot is the solver call,
entirely**, and the fix shape is a **time-denominated bound on the call**, not a finer
node check: it must take a deadline and abort against it, or be refused when the
remaining wall budget cannot cover its worst case.

**What other engines do, for calibration (ARCH):**

- Under a bare `go movetime N` Stockfish **skips its whole adaptive block**
  (`use_time_management()` is false with no clock given), leaving only the hard abort
  `limits.movetime && elapsed >= limits.movetime`, with **no move overhead subtracted**;
  Ethereal, Berserk and Crafty are the same. Only CPW-Engine chronos subtracts one, via
  a **two-level check** — `sd.movetime = movetime − TIMEBUFFER` (500 ms) tested
  *between* iterations, the raw value *inside* the search. **That two-level shape is the
  direct answer to an un-abortable unit of work.**
- **Never abort the first iteration** — Stockfish, CPW-Engine chronos (`if (sd.depth <=
  1) return 0;`) and Crafty (never at `iteration <= 2`) all agree. `stop.rs:28-35`
  already does this; it is convention, not a defect.
- **A node-counted poll cannot see time spent in a path that does not tick it — and
  Stockfish lost a TCEC game to exactly that.** Commit `0f9ae0d1` (PR #5896,
  2025-02-24): *"the mainThread did not search sufficient nodes (512 in > 1s) to trigger
  the stop in check_time."* The fix was a **second, redundant bound checked at iteration
  boundaries**. **This is D-534's shape exactly** — a solver call spends wall time
  without ticking the search's counter — and the remedy is already published.
- **Stockfish does not fully guard an unresolved fail-low** at the first root move,
  mitigating at the *scale* level instead; **Ethereal does** (`revert_best_line()`,
  `completed = depth − 1`). Under a hard `movetime` wall there is no panic-time headroom
  to spend, so **Ethereal's rule is the safer one here**, and it is worth one test.
- **Do not move instantly on a forced move** (500 ms Stockfish, 250 ms Berserk): the
  reported score still has to be correct, and in an instrument mode a shortcut changes
  `SearchInfo`, not just the move. Stockfish also guards a bogus decisive score from an
  aborted search (commit `46ac9a7e6a65`, *"prevent unproven mated-in scores in game
  play"*) — a `PartialRoot` answer here is a lower bound by construction, so that hazard
  is already handled.

## §5 2024-2026 reading, one line each

- **Rapfi**, Jin, Duan & Hang, **arXiv:2503.13178** (2025) — the closest modern
  analogue: VCF at every leaf, SPSA tuning, and the candidate radius as **config with
  six named options** plus a second, tighter radius inside the threat search — a
  shipping existence proof of the rule-2 separation. Caveats: a **preprint with no venue
  and no search ablation in the paper**, whose third author wrote the baseline it
  reports beating.
- **Rapfi's de-facto ablation is in its source comments** — SPSA/SPRT-derived, gomoku
  15×15, **NON-TRANSFERABLE**: **futility ~121, move-count ~107, singular-response
  extension ~77, razoring-with-VCF ~68**, against **null-move ~3, IID ~1**. It agrees
  with `search_gap_2026-09.md` on C-4 and disagrees on C-5/C-6 — but every figure was
  taken with a real VCF quiescence in place, the component D-428 gated off here.
- **Cohen-Solal, arXiv:2505.09639** (2025) — the central negative: over 22 Olympiad
  games **αβ −41.79 %**, Connect6 **αβ −77 ± 3 %**, against Unbounded Best-First
  Minimax. **Four caveats exempt this engine**: expensive GPU neural eval with child
  batching, which he explicitly brackets off (*"These algorithms seem to require a cheap
  evaluation function"*); **no quiescence or threat extension** in their αβ; the winner
  is **not MCTS** (MCTS is worst, −93.55); and their Connect6 is a square board with no
  radius rule. **UBFM is an un-considered option in this design space.**
- **Cohen-Solal & Cazenave, *Minimax Strikes Back*, arXiv:2012.10700, AAMAS 2023 pp.
  1923-1931** — same engine, same opponent, same think time: **ID-αβ scores 92 % on
  Othello 8×8 and 7 % on 10×10.** Cite it whenever a square-board figure is proposed for
  this lattice.
- **Nguyen & Ramanujan**, ICAPS 2024, **arXiv:2212.05208** — UCT is pathological too
  (§1.5).
- **Randall, Müller, Wei & Hayward**, IJCAI-24, **arXiv:2405.05594** — Expected Work
  Search, and §3.5's 93 963 192 → 26; their *"solved up to the 10×10 board"* is wrong
  (two openings only).
- **Wu (2015), SHARP / Arimaa**, *ICGA Journal* — §2.2. **Wu et al., DBS for Connect6**,
  CG 2013, DOI 10.1007/978-3-319-09165-5_1 — §3.4. **Zhang et al. (2018) VCDTS**, **Wu
  et al. (2021) VCST**, CCDC — §3.3. **Zuckerman, Wilson & Nau (2018)**, DOI
  10.1111/coin.12162 — §1.5.
- **NEGATIVE, and it is a finding:** **no Connect6 paper has appeared since 2020**, and
  **no hexagonal-lattice k-in-a-row literature exists at all** — the nearest neighbours
  are Connect6 (square lattice, pair moves) and Hex (hex lattice, single stones,
  connection win), and **nothing combines them.** That is why `threat_calculus_v1.md`'s
  scope rule is load-bearing, not cautious.

## §6 What is worth doing regardless of the eval

1. **The two `SearchInfo` gaps** — root-move scores (for the M1 and M9 tests) and a
   width-tie counter. Neither changes a search decision, both unblock a diagnosis, and
   the width histogram (I1) is the precedent for doing it byte-identically when off.
2. **The solver's time bound** (§4): an abort-responsiveness defect that **no SPRT
   discharges** (D-534), blocking any play arming of the solver, independent of Stage 2.
3. **The CTSS/CRZS citation corrections** (§3.3) — two governing documents cite an
   acronym to no paper.

**Three published structural floors, for the day rule 5's "a measured structural floor
is a finding, not a failure" is invoked:** Hex 8×8 new patterns moved 796 051 → 795 121
nodes because *"the slowdown almost exactly compensates"*; isomorphy detection bought 8×
fewer nodes at 4× the cost and was dropped; Stockfish's `EasyMove` removal measured
under 2 Elo, bounds straddling zero.

Everything else — C-1 aspiration, C-2/C-3 width and reductions, C-5/C-6/C-7 the
eval-margin family — waits on Stage 2 by D-609/D-610, with the mechanisms already built
and gated at 0 in every committed config (`tier_t_top_k`, `safety_net_top_k`,
`aspiration_delta`, `extension_budget`, `lmr_min_depth_turns`,
`configs/instrument_v0.toml:72-99`). **Nothing here needs building before the eval
lands, and two of the three items above are instruments.**
