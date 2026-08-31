# WP-2.0 — SELECTION: row (g), a labelling mode of the `arena` binary, on branch B

**Governed by** `docs/experiments/matrix_wp20_pipeline_shape.md` revision 3, after
two DECISION-RED-TEAM rounds. **This record quotes the red team's rows**, as
CLAUDE.md's Process requires, and records the strongest surviving attack.

---

## 0. THE FIELD RULING, taken under delegation and made visible so it can be reversed

The round-2 red team's one BLOCKING finding is not a defect in the matrix — it is
a question about the field, and under D-511 the field is the operator's. The
dispatch's scope line reads:

> A pipeline (**tools/, its own crate or scripts** per design) that: …

Rows (a), (g) and (h) are none of those three homes. **The operator delegated the
ruling to this session; it is taken here, with its ground, so that reversing it
costs one sentence.**

**RULED: the parenthetical is ILLUSTRATIVE AND DELEGATING, not exhaustive.** The
ground is the clause's own last two words. *"per design"* hands the choice to the
design; an exhaustive list would not also delegate, it would say *"one of"*. The
three named homes are the shapes the dispatch was picturing, and the sentence's
grammatical work is to say the home is the design's to choose.

**What the ruling costs if it is wrong, stated because a ruling that hides its
cost is not reversible in practice.** Under the literal reading, (a), (g) and (h)
leave the field; (c) is a cargo dependency cycle and (d) dies on the referee
finding; **the selection collapses to (b) by elimination** — its own crate, a new
workspace member, and a second `totals_of` carrying D-80 in two places. That is a
worse outcome reached by a narrower reading of one parenthesis, which is the
reason this ruling went the way it did and not the reason it is right.

---

## 1. WHAT WAS SELECTED

**Row (g): a third mode of the existing `arena` binary**, beside `--config` and
`--replay`, which reads a report the arena wrote, walks each recorded game
position by position, sends `newgame` before each ask, and asks at a **label**
`go_line` — writing one record per position. The games it labels are produced by
the **unmodified** SPRT path in a separate, earlier run.

**On branch B of its label seam**: `exchange::totals_of` is raised to
`pub(crate)` and widened so `score` and `pv` come out of the one parser both
clients already share — with the three existing lookups kept **load-bearing** and
the two new ones **non-fatal `Option`s**.

## 2. WHY, IN THE RED TEAM'S OWN TERMS

The second round was pointed at (g) by name, with four load-bearing claims broken
out and the operator's leaning quoted to it so it would attack the leaning rather
than infer it. Its answers:

| claim | verdict | evidence it gave |
|---|---|---|
| **ZERO SEAMS** | **YES on every clause** | no new crate, no manifest change, no `[workspace.dependencies]` entry, no dependency-name change; all four `workspace_shape_tests.rs` tests read manifests only. **A record sink needs no new dependency** — `pistol_cli::corpus::emit::Fixture` is public plain text over `fmt::Write` with an in-band sha256, and the workspace has no `serde_json` |
| **COLD BY CONSTRUCTION** | **YES** | traced verb → engine → searcher → table/heuristics/solver. `Searcher` has six fields; `clear` reaches three, `position` is rebuilt by `reset_to` at the top of every `search`, `params` is immutable, and `census` is `None` in every shipped path. **The solver's table is REBUILT by `reset`, so zeroing the epoch cannot resurrect anything.** Nothing survives |
| **THE UNCONFIRMED KILL** | **does NOT fire** | settled by reading, no run needed: the report writes every game's full move list, `transcript::read` replays it through pistol-core, every to-move position is a prefix — which is exactly what `position_line` sends — and **rule 4's one-stone winning turn round-trips as `Turn::Single`**. Nothing lossy on forfeit or turn cap |
| **FITS THE LICENSE** | **YES** | only `pistol-arena` is touched; nothing new on the wire, because the budget is per-`go` and `score` and `pv` are already on the totals line |

> **"Row (g) survives. I could not kill it."**

## 3. THE STRONGEST SURVIVING ATTACK, which the ADR line records

**It is MAJOR 3, and it is why this record picks a branch instead of deferring
one.** Revision 2 wrote that (g)'s label seam *"does not need to pick"*. The red
team's answer:

> A matrix may leave a design choice open; it may not leave open a choice whose
> two branches it has already priced as fatal to two other rows.

Branch A carries row (b)'s **registered kill condition** — a second totals reader,
D-80's discipline in two places inside one crate. Branch B carries row (e)'s
`?`-chain hazard, which round 1 proved would suppress `compute.add` and zero the
SPRT report's node counts. **Neither branch is free, and revision 2 charged both
to other rows while leaving (g) unpriced.**

Branch B is taken because round 1's own strongest failed attack shows how to make
it safe — non-fatal `Option`s, existing lookups load-bearing — whereas branch A
has no such construction: it is the duplication, and duplication of a
defect-bought discipline is the thing (b) is killed for.

**A second attack survives and is not answered by this selection**: `newgame` per
label is a **256 MiB memset** at every committed seat's `tt_bytes`. Revision 2
called coldness *"one line"*, which is true of the source and not of the cost.
**The pilot measures it; this document does not guess it** (D-500).

## 4. WHAT THE DESIGN NOW OWES, and what it may not re-open

**Owed, and named by the red team:**

1. **D-540's second clause** — a **fresh-process agreement criterion**, registered
   in the pilot's pre-registration, naming the defect class it excludes. Written
   without that, it repeats D-527's vacuous check.
2. **The seed (memo decision 5)** appears nowhere in the matrix. D-540 fixes that
   seeds attach to pipeline SAMPLING only; the design says what is sampled.
3. **The label budget's home (memo decision 4)** is **relocated, not dissolved**:
   `--workers` is a run mechanic deliberately excluded from `experiment_sha256`,
   so a label budget arriving the same way is excluded from the digest that
   identifies the run. The design says what identifies a labelling run.
4. **What `transcript::read` drops** — `result`, `forfeit_by`, and the
   `openings_*` records that dispatch requirements 2 and 5 name. Recoverable
   through pistol-core except for forfeited games; the design says how.
5. **The census-minimum rule for detector round 3** (D-537), which lands before
   any corpus exists so it cannot be fitted later — unchanged by this selection.

**Not re-openable without a new matrix**: which crate owns the pipeline.

## 5. WHAT THIS SELECTION IS NOT

It is not a claim that (g) is cheap — §3 names two prices it pays. It is not a
strength claim of any kind. And it does not authorise the labelling mode to be
written: the DESIGN is next, and it is reviewed before any implementation.
