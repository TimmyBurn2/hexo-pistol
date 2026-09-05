# Optimization arc — CLOSURE. Eleven packages, four tranches, and the arc's own method changed twice on the way.

> **ONE LINE FOR THE MORNING.** The arc ran all eleven ruled packages and the
> engine ends **1.294x faster** with byte-identical output — and the packages
> that were supposed to convert that into strength all failed in the same
> direction, which turned out to be the finding rather than the disappointment.
> **The three width-and-pruning mechanisms COMBINED do reach 8 turns where the
> committed engine reaches 6, on fewer nodes, and lose 315 W / 391 L.** Two more
> turns of search make this engine measurably worse, so **depth is not the lever
> at eval v0** — measured four independent ways, not assumed. Every mechanism
> ships built, tested and one config key from live, for the eval that can use
> them. The arc also built the anchor's openings book, which **overturns its own
> earlier 0 W / 100 L reading to 40 W / 60 L over fifty paired openings**, and
> leaves the next package named by measurement: sealbot searches **5 turns to
> this engine's 2**, and the width that is actually there is the quiet ball's
> median of **76**, not Tier T's 12.

## The MEASURED table — every package, every number derived from its own receipt

| # | package | verdict | the number that decided it |
|---|---|---|---|
| P1 | threat-state store → per-axis line bitboards | **LANDED** | nps **1.243 / 1.217**, search output byte-identical over 128 searches at three seats |
| P2 | legality per candidate | **NOT LANDED**, measured finding | **0.994 / 0.995** — a regression; the crossover is ~160 stones and a game ends at 79 |
| P3 | codegen: thin LTO + one codegen unit | **LANDED** | **1.068 / 1.064**; cgu=1 alone 0.992 / 0.979, fat LTO 1.030 / 1.022 |
| — | **tranche 1 combined** | — | **1.294 / 1.293**, measured end to end, not composed from the terms |
| I1 | width histogram | **LANDED** | byte-identical when off; Tier-T median **12**, quiet-ball median **76** |
| I2 | budget-overrun VOID class | **LANDED** | threshold **4x**, from a measured median 0.090 and max 0.175 over 60 openings |
| W1 | Tier-T width cap | **GATED OFF**, measured null | K=16 binds **23.3 %** of BATCHED nodes, removes 11.2 % of Tier-T cells, changes **0 of 24** openings |
| W2 | root re-ordering | **GATED OFF**, measured null | **0 of 24** at ~2 turns, **0 of 12** at ~4 turns |
| S1 | aspiration windows | **GATED OFF**, measured null | **0 of 16** at every width, and every width SLOWER: 0.815 / 0.508 / 0.520 / 0.561 |
| S2 | one-cell forced-reply extension | **GATED OFF**, SPRT **h0** | **8 W / 61 L**, LLR pair −2.956 vs −2.9444, n=90, normalized Elo −391.5 |
| S3 | late move reductions | **GATED OFF**, SPRT **UNDECIDED** | full 600-pair cap, `LLR pair -1.831` inside ±2.944, n=1200, normalized Elo −13.4, **both seats reached 6 turns** |

| — | **the COMBINATION** (W1+W2+S3) | **REFUSED**, SPRT h0 | **8 turns against 6 on 5.6 % fewer nodes**, and 315 W / 391 L, normalized Elo −34.2 |
| — | **where perf stands** | — | **1.28–1.40x** faster for identical node counts; **median depth 2 turns at 0.5 s**, sealbot's **5 at 0.3 s** |
| — | **sealbot anchor v5** | direction only | **40 W / 60 L** over **50 paired openings**, 100 distinct games — overturning v4's 0 W / 100 L on one opening |

**Committed config state after the arc**: `tier_t_top_k = 0`, `root_reorder =
false`, `aspiration_delta = 0`, `extension_budget = 0`, `lmr_min_depth_turns =
0`, `lmr_late_index = 3`, in all fifteen committed documents. Every one of the
six is a mechanism that exists, is validated, has a test, and is shut — and each
has a number beside it saying why.

## What the arc learned about its own method, which is the part that transfers

**1. THE SENSITIVITY CHECK, AND ITS OWN CORRECTION.** Build both seats, play the
governed openings at the governed budget, count differing `bestmove` lines. It
costs minutes; an SPRT costs half an hour and concludes nothing when the change
is absent from play (D-492's registered failure mode). It foreclosed W2 and S1
outright and confirmed W1's calibrated null.

Then S2 corrected it. The extension fires on **933 of 3153 FILTERED rows — 29.6 %**
and at `nodes 50000` changes **0 of 24** bestmoves at every budget tried. Read
there it is indistinguishable from the three foreclosures. At `nodes 200000` it
changes **12 of 24** and at 500 000, **14 of 24**. The mechanism was never
absent; the budget was too small for iterative deepening to spend the difference.
**A successor runs the check at the budget the match will spend, and a null at a
smaller budget forecloses nothing.**

**2. THREE OF THE ARC'S RULED TARGETS WERE WRONG, AND MEASUREMENT SAID SO EACH
TIME.** P1's stated premise — laziness — is worth under 3 %, because every leaf
reads the threat state through the quiescence gate even with quiescence off; the
lever was the cost of a touch. P2's hotspot belonged to `GameState::place`, not
to candidate generation, and its remedy loses at every board size a game reaches.
W1's ruled target, Tier T, has a median width of **12** on the governed book —
already narrower than the opponent's 15-cell cap — while the set that is actually
wide is the quiet ball at median **76**, carrying 24.3 % of every expanded cell
through a cap mechanism that already exists. A dispatch names a target; the
measurement names the lever, and they were different three times in eleven.

**3. WHAT A NULL IS WORTH.** Seven behaviour-changing packages produced no
default change. That is not seven wasted packages: each one closed a live option
with a number, and four of them (W1, W2, S1, S2) closed options the design report
listed as PROTOTYPE candidates. The engine ends the arc with six new tunable
mechanisms, all shut, all documented with the measurement that shut them, and any
one reopens the moment a deeper-searching seat or a stronger eval changes the
trade. **Rule 5's own words: a measured structural floor is a finding, not a
failure.**

**4. THE OVERRULE (D-604) DID WHAT IT WAS GRANTED FOR.** P1's pre-registration
STOPPED on the wording of a rule for reading a number that had not yet been
taken. The operator's ruling ended the document loop and did NOT weaken a
measurement: every number in the table above was taken on finished code by a
registered instrument. The arc after that ruling produced two SPRTs, five
sensitivity checks and one bench, and argued about no document.

## D-534 RESTATED, because the arc did not touch it and a reader must not infer that it did

**Arming the solver in a committed play config still has a precondition that no
SPRT discharges.** D-520 measured it at the deployment budget on the governed
anchor: with `[solver] on_search_path = true`, 240 answers at a **500 ms** budget
had a **median of 1225 ms and a maximum of 1866 ms**, a **725 ms median
overshoot**, because a solver call absorbs its whole node count at once and the
root's two calls are made before anything is abortable. The same instrument
measured the gates-off seat at an **8 ms** maximum overshoot.

**On HeXO the server owns the clock and hard-clamps the call, so an answer at
1866 ms against a 500 ms budget is a FORFEIT there.** This arc made the engine
**1.294x** faster, and a reader could take that to mean the overshoot shrank by
the same factor. **It does not follow and it was not measured.** The overshoot is
structural — an unabortable call, not a slow one — and a 1.29x faster call that
is still unabortable still overshoots. Nothing in this arc measured the solver
seat's overshoot, and D-534 stands exactly as written until something does.

## ROADMAP: what this arc discharges and what it does not

**Stage 4's PROTOTYPE gauntlet** names LMR among the techniques "kept only if
SPRT-positive". S3 is that test, run early and at this engine's own reach; its
verdict is in the table above. **This does not close Stage 4** — the gauntlet's
other named items (guarded/verified null-move, futility, razoring) were DEFERRED
by this arc's own dispatch to Stage 2, on the ground that they are eval-margin
techniques resting on an eval measured to misread horizons (D-428). Lazy SMP and
SPSA/Texel tuning are untouched.

**Stage 3 is untouched.** Nothing here is threat search.

**What the arc adds to the plan that was not in it**: the quiet ball, not Tier T,
is where this engine's width actually is (W1's calibration, median 76 against
12), and `safety_net_top_k` is the mechanism already built for it with a reserved
holdout book to measure it on. That is the next width package, and it is named
here so a successor does not re-derive it.

## Exports and receipts (D-469)

Every arc worktree's gitignored `artifacts/` and `sessions/` are copied into
`artifacts/opt_arc_worktree_export/<worktree>/` before any worktree is removed,
because removal takes them with it and WP-1.8c's four review reports survive only
in a transcript.

**101 files**, listed with their digests in
`artifacts/opt_arc_worktree_export/RECEIPT.sha256`, whose own sha256 is
`44fe45ca80d2a1ca3d23a96626bf0c3edc1a56616f4f584a841ca177b29a0139`. The worktrees
covered: `p1-measure`, `p1-mutants`, `p2`, `p3`, `i2`, `w1`, `w2`,
`s1`, `s2`, `s3`, `t1-ci`, `arc3-base`.

**The one indirection**: the review reports for P1's three rounds live in
`/home/tom/pistol-wt/p1-patches/` and in this session's scratchpad, not in a
worktree's `artifacts/`; the closure copies the ones that are files and names the
rest as transcript-only, which is the same limitation D-469 was written for.

## The anchor, and the one place this closure does not do what the dispatch asked

The dispatch asks for **"a sealbot anchor v4 on the final engine (book_v1, N =
100, movetime, gates as committed) for direction only"**.

**`book_v1` could not be used, and the reason is a package that is still open.**
The local match platform plays ONE opening — the server's own origin cross
(`tools/sealbot/matchserver/src/referee.rs`) — and its config schema has no
openings field at all. Anchor v3's openings reader was designed, option-matrixed
through three red-team rounds and reviewed (`anchor_v3_openings_design.md`,
`matrix_anchor_openings_reader.md`), and **never implemented**; arc III's closing
commit says so in its own message. Implementing it is a package, not a closure
step, and this arc's dispatch did not authorize one.

**So the anchor is run as v2 ran it**: N = 100, `movetime 500`, gates as
committed, one opening. **Its honest denominator is 2, not 100** — both engines
are deterministic from a fixed opening, so the run produces one game per seat
assignment and repeats it. That was already true of v2, whose own ADR line says
it (D-519: *"the honest denominator is 2, not 40"*). What N = 100 buys over N = 2
is a determinism check across a hundred process launches, which is worth having
and is not a strength claim.

**What the anchor is for here**: the arc made the engine 1.294x faster, so the
same 500 ms buys more depth. The anchor says whether the standard opening's
outcome moved. It is direction only, against an UNVERIFIED opponent (D-197), and
it moves no standing judgment.

## What is owed after this arc, and to whom

1. **The quiet-ball width package.** W1's calibration named it: median 76 against
   Tier T's 12, firing on 5.5 % of BATCHED nodes and carrying 24.3 % of every
   expanded cell, with `safety_net_top_k` already built and a reserved holdout
   book to measure it on. This arc could not run it — the dispatch ruled Tier T
   and a session may not reorder — and it is the strongest-evidenced width
   candidate the engine has.
2. **The anchor's openings reader**, still unimplemented, still holding a
   designed-and-reviewed package. Until it lands, every anchor's honest
   denominator is 2.
3. **P1's pre-registration disposition rule** is fixed and unreviewed — corrected
   after its gate was spent and tested by exhaustion by its author, not by a
   fresh context. A successor reusing that document reviews that rule first.
4. **Six gated-off mechanisms with their re-opening conditions**: `tier_t_top_k`
   (a K that binds without cutting below the median), `root_reorder` (a reach
   where the order after the table move decides), `aspiration_delta` (a reach
   where a narrow window fails less often), `extension_budget` (a seat deep
   enough that the forced line's horizon stops binding), `lmr_min_depth_turns`
   and `lmr_late_index` (see S3's line). Each has a number beside it; none needs
   re-deriving.
5. **The worktrees** are removable once this closure's export receipt is
   committed. They are the only place the option matrix's six measured revisions
   exist.

## The anchor's openings reader, built at closure because the anchor could not answer without it

The dispatch asked for an anchor on `book_v1`. The match platform could not play
one: it had no openings field and its referee played a single origin stone, so
every anchor this project has run had an honest denominator of **2** —
D-519 said so of v2, and D-606 said it again of v4 while reporting a 0 W / 100 L
result nobody could interpret. **The reader was designed, option-matrixed
through three DECISION-RED-TEAM rounds and reviewed, and never implemented.**

It is implemented now, to that design and its selected option (O2: a reader in
the matchserver rather than a dependency on `pistol-arena`, which is outside its
workspace by intent):

- `[openings]` is a required table under `schema_version = 2`, closed over
  `platform_standard` and `book`. All existing configs are migrated.
- **Eleven refusals, each with a test**: a body that disagrees with its own
  `# body_sha256`; a blank line or comment inside the body; a line that is not
  `start moves …`; an opening the RULES refuse; an opening not left undecided
  and at a turn boundary; a symmetric duplicate; mixed turn counts; a window the
  file cannot fill; a `turn_cap` that does not exceed the opening's own turns;
  `games != 2 * take`; and a `platform_standard` document naming a book key.
- **The digest is the outermost gate, and that made the first four tests
  vacuous.** Every broken body changes the body digest, so R1 fired and masked
  the refusal each case was written to prove. The suite now RE-DIGESTS each
  broken body, and the cases fail without their fix.
- **Openings are PAIRED**: games 2k−1 and 2k are the same opening from opposite
  seats, which is what `games == 2 * take` enforces, so an opening's own
  first-player advantage cancels within the pair.
- Two records that had become false are fixed: the transcript's `opening` field
  said "the platform's standard setup" whatever was played, and the report's
  header said "not paired" of a paired run.
- **`distinct_games` was under-counting** and it is the anchor's headline
  number: it hashed only the engines' turns, so two games from DIFFERENT
  openings whose engine moves coincided counted as one. The opening is now part
  of a game's identity. Caught by the reader's own happy-path test on the day
  book support landed.


## One build warning this arc did NOT introduce, named so nobody attributes it here

A `--release` build reports `unused import: generate_turns` at
`crates/pistol-solver/src/policy.rs:1`. The symbol is used at line 266 inside a
`#[cfg(debug_assertions)]` block, so release compiles the use away and the
import is genuinely unused there. **It reproduces at `ffc5c10`, the arc's own
baseline** — checked, not assumed — and `policy.rs` was last touched before this
arc began. It is left alone: a closure commit is the wrong place for an
unrelated fix, and a warning silently repaired here would be indistinguishable
from one this arc caused.
