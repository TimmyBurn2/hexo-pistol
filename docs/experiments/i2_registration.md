# I2 — REGISTRATION: the budget-overrun VOID class

**Registered before implementation.** The dispatch: *"The instrument VOIDs a
game whose first iteration exceeds a registered multiple of the node budget; the
multiple is derived from the landed runs' measured first-iteration ratios
(artifact-cited), never from the pathological reproducers."*

## What is being detected, and why it is the instrument's job

A-01 (`docs/audit/repo_audit_2026-09.md`): under a node budget the FIRST
iteration is not abortable (`crates/pistol-search/src/search.rs:423`,
`abortable = depth_turns > 1 || fallback.is_some()`), so a position whose
root turn is wide enough answers a `go nodes 500` with 28.66 M nodes. **The
budget is not a bound on a legal position.** D-578 ruled that this is *"a known
limitation of instrument mode on pathological positions"*, that it has never
contaminated a landed run — *"every landed run of this project spent exactly its
budget, because every one of them ran on book positions"* — and that it gets a
DETECTION here and a fix later.

The detection belongs to the arena and not to the engine: it is a statement
about whether a GAME is admissible evidence, which is the instrument's question,
and rule 6's equal-per-side-compute premise is what it protects.

## The multiple, DERIVED from real runs

`first_iter.sh` over the first **60 openings of `random_openings_v2.txt`** — the
book the governed runs draw from — at the committed instrument seat and the
governed budget `go nodes 50000`, reading each search's `info depth_turns 1`
line against the budget it was given:

| statistic | first-iteration nodes ÷ budget |
|---|---|
| min | 0.0014 |
| median | 0.0898 |
| p90 | 0.1527 |
| **max** | **0.1754** |
| mean | 0.0826 |

**REGISTERED MULTIPLE: 4.** A game is VOID when any search's first iteration
exceeds **four times** the node budget that search was given.

**Why four.** It is **23×** the largest ratio 60 governed-book positions
produce, so no position of the kind these runs actually draw can reach it by
being merely awkward; and it is **four orders of magnitude** below the
pathological reproducer's 57 000×, so the class the detector exists for cannot
hide under it. The number is derived from the ratios above and from nothing
else — in particular not from the reproducers, which is what the dispatch
forbids. **It does not move after a run** (D-374).

## What a VOID does, and what it must not do

- A VOID game is **not scored**: it enters neither the pentanomial nor the LLR,
  and it is not a forfeit — nobody played illegally, the instrument declined to
  read the game.
- It is **counted and reported** by its own name, so a run whose VOIDs are
  numerous is visibly not the run its `n` claims.
- **A run whose VOID count is above zero is reported with it**; whether that
  invalidates the run is the reading party's call and this document registers no
  threshold for it, because none is derivable yet.
- The refusal path touches the verdict, so it is red-teamed: the failure mode
  to attack is a VOID that silently changes `n`, a pair whose two games are
  scored asymmetrically because one voided, and a VOID that reads as a forfeit
  in any report field.
