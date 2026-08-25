# WP-1.7 DECISION-RED-TEAM — the §4 option matrix (M1–M7)

**Reviewed revision:** `232a175b6f4c7789cef76cb3a5324c4afe749cf3` (HEAD at review time;
working tree clean). The dispatch named `232a175b6f4c7789cf76cb3a5324c4afe749cf3` —
the same commit with one character dropped (`cef76` → `cf76`); treated as a match,
since HEAD is the rev-2 commit `232a175 docs(experiments): WP-1.7 design rev 2` and
`docs/experiments/wp17_design.md` at HEAD self-identifies as revision 2.

**Method.** Fresh context; authored nothing here. Every code citation in the matrix's
rows was checked against the tree; the M1 grep was re-run; the cited ADRs (D-9, D-5,
D-51, D-52, D-192, D-215, D-291, D-396, D-428) and `minimax_report.md` line 83 were
read. No repository modification; no scratch experiment was needed beyond greps —
every attack below is grounded in code or in arithmetic on the document's own stated
mechanism.

---

## M1 — where the config gates live. **SURVIVES WITH WOUND**

**Verified.** `rg -n 'StagedParams \{' crates/` returns 11 hits, one of which is the
struct definition (`params.rs:59`); the other 10 are exactly the sites the row lists
(`instance.rs:183`, `quiescence.rs:566`, and the eight test-tree sites at the cited
lines). The count is right, and the precedent is real in the committed schema:
`q_depth_turns`/`q_triggers` live inside the `kind = "staged"` variant of the
`deny_unknown_fields` tagged enum (`config.rs:177-222`), and `quiet_top_k`/
`widen_schedule` are the established case of variant keys validated then dropped at
the boundary (`instance.rs:181-182`) — variant-scoping keys exactly one path reads is
the house style, so (b)'s dead-key cost is described accurately.

**Wounds.**
1. **`quiescence.rs:566` is test code.** It sits inside `#[cfg(test)] mod tests`
   (`quiescence.rs:475-476`). The row's phrasing — "`instance.rs:183`,
   `quiescence.rs:566`, and eight in the test tree" — reads as two production sites
   plus eight test sites; the truth is **one** production construction site and nine
   test sites. The count survives; the categorization does not. Under D-291's
   discipline a MEASURED claim that is right in number and wrong in kind is a
   finding: the production blast radius of (a) is half what the row presents (which
   strengthens (a), but a measured claim is not allowed to be wrong in the
   direction that happens not to matter).
2. **The D-396 citation is loose.** D-396's full text (`decisions.md:848`, 1164
   chars, read in full) records the architect ruling transferring arbitration to
   SPRT; it does not mention `q_triggers`, variant scoping, or config placement.
   The scoping precedent is verifiable directly in `config.rs:177-222` (and
   `config.rs:227` itself cites D-396 for the `[search.candidate_policy.q_triggers]`
   key — the citation is inherited, not checked). The argument's substance stands;
   the citation names the wrong ADR line for it.

**Omitted option (named, not fatal):** a conditionally-required top-level
`[search.ordering]` (required only when `kind = "staged"`) would avoid (b)'s dead
keys, but fights rule 1's missing-key-is-an-error simplicity and has no precedent;
its omission is defensible.

## M2 — how history orders. **SURVIVES WITH WOUND**

**Verified.** The cap-at-one-cell claim is literally true: §3.3's item 4 promotes at
most one cell per node. Option (c)'s dismissal is code-accurate: `delta_rank`
(`staged.rs:340-348`) computes the delta scores into a local `scored` vector and
discards them — `StagedSet` carries only `cells`, so by the time `pvs::visit` holds
the set the scores no longer exist, and touching the ranking means touching
generation. Option (b)'s dismissal (stable history sort lets history dominate
delta) is also accurate. The cost claim "one map lookup per unforced candidate plus
one rotation" matches the mechanism.

**Wounds.**
1. **The dilution stack is unowned.** History's single promotion is the LAST of the
   heuristic promotions — behind the TT move, two killer slots, two pair-killer
   cells, and the countermove (§3.3's order) — so in heuristic-rich nodes its cell
   is tried 5th–7th, and its per-node influence is one cell *behind up to six
   others*. No row acknowledges this; the "caps history's influence at one cell"
   phrasing presents the cap without the position it is capped into.
2. **The composite-minimality is unowned.** M2(a) (one cell), M5(a) (floor-halving
   decay), and M7(a) (flat +1) each minimise history along a different axis, and no
   row states what the three choices jointly deliver: a mechanism with one cell of
   influence, ~one search of memory, and frequency-only signal, on top of a
   delta-ranked candidate set that §1 itself expects to make the heuristics largely
   redundant. The h0-expectation framing in §1 absorbs this honestly at the WP
   level, but the matrix never confronts that it is selecting the weakest viable
   member of each option set simultaneously.
3. **Omitted middle option.** Top-K history promotions (K = 2–3) sit between (a) and
   (b), cost K−1 extra rotations, and are the natural first relaxation if the SPRT
   reads null; the row's option space is {one, all, tie-break} and skips it. (Also
   minor: M2's "one map lookup per unforced candidate" is a numeric claim presented
   unmarked, where M1 marks its equivalent — an inconsistency in the marking
   discipline the row otherwise keeps.)
4. **The countermove-before-history order is an unmatrixed choice** that directly
   conditions this row's cap: the report's line 83 stack treats "history/countermove"
   as ONE tier, and the design's split of it (countermove first, history last) is
   decided in §3.3 prose with no matrix row and no stated reason.

## M3 — do pair killers earn a slot. **SURVIVES WITH WOUND**

**Verified.** `minimax_report.md` line 83 says verbatim "Adapt killers/history to
*pair* moves by keying on the completing stone and on the pair. BUILD." — the row's
citation is exact, and the design source of record names both keyings. The claim
that (b) "loses the pair's FIRST stone" is accurate given §3.2: history is
incremented only for the cutoff cell `c`, never for the turn's first stone `prev`,
and the single-cell killer at the phase-Second ply covers only the completing
stone — so under the design's own update rules, nothing else records `prev`.

**Wound — the row is conditioned on an unmatrixed adjacent decision.** §3.2's credit
rule (history credits the cutoff stone only) is itself a named design decision with
a viable alternative — credit `prev` too, or credit it at the phase-First ply —
that would supply exactly the first-stone hint M3(a) exists to provide, through the
history table this WP already adds, with no new table and no rule-4/canonical
validation seam. The M3 row's option space {pair-killer table, nothing} never weighs
it, so the recommendation's necessity rests on a premise settled outside the matrix.
(The unforced-only update boundary that §3.2 also fixes — and that §9 ADRs — is
likewise matrix-less; its prose reasoning checks out against the code, but the
matrix is smaller than the decision set feeding §9's ADR lines.)

## M4 — pair promotion shape. **SURVIVES WITH WOUND (strongest code-grounded attack)**

**Verified.** The call survives: promoting each present cell beats (b)'s
require-both, and the "never a wrong move" claim holds — promotions only rotate
existing unforced candidates (§3.3), per-placement win detection runs in `pvs.rs:322-338`
either way, and the final position of a turn is order-independent once the
first-stone-wins class is excluded, which §3.4's rule-4 check does.

**Wounds.**
1. **A viable option is omitted: promote in PLAY order.** The write site knows which
   cell was the pair's first stone (`prev` is `played()`'s last entry, §3.2); storing
   `(prev, c)` ordered instead of `canonical(prev, c)` costs nothing, and promoting
   in play order *composes with the single-cell killer at the phase-Second ply* —
   M3's own coverage argument — to reconstruct the whole refuting pair: `prev` first
   via the pair killer, `c` second via `killers[ply+1]`. Canonical promotion breaks
   that reconstruction whenever `c < prev`.
2. **The row understates its own stated failure mode.** "Can promote a pair's second
   cell as a turn's first stone" is framed as the one-cell-missing case ("(b) wastes
   the hint when one cell is occupied/off-set"), but canonical-order promotion
   inverts play order **even when both cells are present**, whenever the completing
   stone sorts before the first stone — the stored pair discards play order at write
   time (§3.2: `canonical(prev, c)`; `Coord`'s derived order is `(q, r)` lexicographic,
   `coord.rs:33`, D-5). Not a correctness defect; a weaker hint than the row implies,
   from an option the row never considered.

## M5 — history aging. **SURVIVES WITH WOUND**

**Verified.** No determinism defect: the lifecycle is coherent with the existing
gates — the same-process determinism test compares two FRESH searchers plus a
distraction-then-clear arm (`search_determinism_tests.rs:46-73`), so per-Searcher
heuristic state cannot bleed into the law's "same position + budget twice" reading,
and `Searcher::clear` (`search.rs:199-201`) is the single newgame seam the design
extends. Warm-replay reproducibility holds for the same reason TT persistence
already holds: a replay that drives the same `newgame`/`go` sequence reproduces the
same table states.

**Wounds.**
1. **The stated discriminator against (b) overstates what (a) delivers.** With floor
   halving at `begin_search` and a flat `+1` per cutoff, a score of 1 halves to 0:
   **a cell that cutoff once contributes nothing to any later search**; a cell needs
   ≥2 cutoffs in one search to survive a single aging (2→1), ≥4 to survive two, and
   aged-1 is outranked by any cell with one fresh cutoff. The "cross-search signal
   within a game" that the row invokes to reject (b) is, as specified, a thin residue
   with a memory of roughly one search, available only to repeatedly-cutting cells —
   close to the distinction-without-a-difference CLAUDE.md deletes rather than
   refines. Either the aging constant is wrong for the stated purpose or the stated
   purpose is wrong; the row picks neither.
2. **The countermove exemption's stated reason is fallacious.** §3.1: countermove
   "is left alone — its key means the same thing throughout a game." History's key
   `(Player, Coord)` means exactly the same thing throughout a game, yet history is
   aged — key stability cannot be the discriminator. The sound discriminator —
   countermove is a single-slot last-write-wins table that replaces its own stale
   entries, while history is an accumulator that cannot — is never stated anywhere.
   Worse, the anti-staleness principle the row deploys against (c) ("stale-opening
   scores would dominate late-game argmax") applies unaddressed to a countermove
   entry written in the opening and read at full authority in the endgame whenever
   its key recurs. The asymmetry is a *choice*, and a lineage-standard one, but it is
   a choice made in §3.1 prose with a wrong reason, and countermove's lifecycle has
   no matrix row at all despite being one of the three tables M5's decision spans.
3. **Minor omitted option:** subtract-a-constant decay (or round-up halving) would
   let single-cutoff signal survive one search; not weighed.

## M6 — killer slots per ply. **SURVIVES**

The only row that marks its benefit **ESTIMATED** and says so plainly ("zero measured
hex evidence, consistent with §1's honesty"). The cost claim (two rotations, arrays
sized `MAX_PLY` = `2*64+2+32` plies, `search.rs:81`) is trivially code-true. The
1-vs-2 cut is the right minimal question; >2 slots is lineage-marginal and its
omission is defensible. Nothing to attack that the row has not already said about
itself.

## M7 — history bonus shape. **SURVIVES WITH WOUND**

**Verified / defensible.** The call is defensible, and there is an argument for flat
`+1` the row does not even make: under the argmax-only reading, flat bonuses create
score TIES that fall back to the delta order (§5's left-to-right scan over the
delta-ranked candidates), while depth-scaled bonuses create distinct values whose
depth noise overrides delta among equal-frequency cells — flat is weakly *safer*
given M2(a). The tuning-axis argument (no hex evidence behind any exponent) also
stands.

**Wound — the row's central claim is false across searches.** "The argmax-only
reading (M2a) makes the two shapes differ only in relative order among candidates
with different cutoff depths" is true within one search and false under M5(a)'s
floor-halving: magnitudes determine persistence, so a flat `+1` dies at the next
`begin_search` while a depth-scaled bonus of ~50 survives ~6 agings — the shapes
differ in MEMORY HORIZON, a first-order coupling with M5 the row treats as
independent and second-order. The conclusion survives; the reasoning has a hole,
and it is the same hole as M5's: no row owns the M5×M7 composite that actually
determines what history does.

---

## Bottom line

**All seven recommendations STAND; none dies.** No row's call is overturned by code
or by an omitted option that dominates the recommendation. Five rows carry wounds
that should be closed in a revision before selection is treated as final — all are
documentation/argument defects, not wrong calls:

- **M1 (a) stands.** Strongest surviving attack: the MEASURED site list
  miscategorises `quiescence.rs:566` (inside `#[cfg(test)]`, `quiescence.rs:475`) as
  production churn, and cites D-396 for a variant-scoping precedent D-396's own text
  does not contain.
- **M2 (a) stands.** Strongest surviving attack: no row owns the composite — one
  cell of influence (behind up to six other promotions), one search of memory, and
  frequency-only signal, chosen simultaneously on three axes with no statement of
  what the composite is expected to do beyond null.
- **M3 (a) stands.** Strongest surviving attack: its necessity premise — nothing
  else records the refuting pair's first stone — is an artifact of §3.2's unmatrixed
  credit-only-the-cutoff-stone rule, whose obvious alternative (credit the first
  stone in history) would erase the need for the new table.
- **M4 (a) stands.** Strongest surviving attack: play-order promotion — free at
  write time, omitted from the options — composes with the phase-Second single-cell
  killer to reconstruct the refuting pair, which canonical-order promotion breaks
  whenever the completing stone sorts first.
- **M5 (a) stands.** Strongest surviving attack: floor-halving plus flat `+1`
  preserves almost none of the cross-search signal that is the row's entire stated
  reason for rejecting (b), and the countermove exemption from aging is justified by
  a key-stability argument that applies equally to the table it exempts history
  from.
- **M6 (a) stands.** Strongest surviving attack: none stronger than the row's own
  ESTIMATED confession — zero measured hex evidence for the second slot.
- **M7 (a) stands.** Strongest surviving attack: the "differ only in relative order"
  claim is false across searches under M5(a) — the shapes differ in persistence, not
  just ordering, so the row dismisses depth-scaling by an argument that ignores the
  M5 coupling it actually rides on.

**One process-level finding cutting across rows:** the matrix is smaller than the
decision set it feeds — the unforced-only update/promotion boundary and countermove's
cross-search lifecycle are named, ADR-bound decisions (§9) with viable alternatives,
each settled in §3 prose without a row, and each load-bearing for exactly the rows
(M3, M5) where the wounds above landed.
