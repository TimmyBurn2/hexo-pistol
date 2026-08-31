# WP-2.0 — OPTION MATRIX: where the label pipeline LIVES

**REVISION 2**, after a fresh-context DECISION-RED-TEAM returned **FALLS** on
revision 1 (`28e1afd`) — 3 BLOCKING, 9 MAJOR, 4 MINOR. **Two of the three
BLOCKING findings are measurement errors in the block revision 1 headed
*"MEASURED, so no row is ranked on a guess"***, which is the sharpest place to
be wrong and is why this revision leads with them.

| the red team found | what revision 2 did |
|---|---|
| **B1.** The "reusable surface" was listed as five items. `pistol-arena/src/lib.rs` declares **22 `pub mod`**, 133 public items. Row (b)'s stated price — duplicating forfeit, overlong and out-of-turn handling — is wrong on all three: `Received::Overlong` and `Channel::unsolicited` are **public**, and forfeit is an SPRT concept a corpus does not need | the surface is re-measured, and row (b)'s price is restated as the ONE genuinely private thing: `exchange::totals_of`, which carries D-80 |
| **B2.** The coldness criterion never mentions the mechanism that provides coldness. `/usr/bin/grep -ic "newgame\|clear"` over revision 1 returns **0**. `Table::clear` is a true wipe, `newgame` reaches it, `seats.rs` already sends `NEW_GAME` per spawn, and an engine test pins a newgame'd engine **node-for-node equal to a fresh one** (D-7) | criterion 4 is rebuilt on `newgame`. **Coldness is cheap for every row**, so the criterion no longer discriminates by cost — it discriminates by WHERE a `newgame` may safely land |
| **B3.** Row (e) says it returns "the label fields it already parses". `totals_of` returns `(nodes, time, depth_turns)` and parses **no score and no pv** | (e) is re-priced as NEW parsing, with the failure mode the red team found: a `?`-chain that silently zeroes the SPRT report's node counts |
| **M9.** Neither WP-2.0 dispatch exists at a tracked path — D-469/D-511's class, already repaired once for Stage 3 | both are transcribed to `docs/experiments/wp20_dispatches.md`, which also answers row (g)'s kill condition from the governing text |
| rows **(g)** and **(h)** added | in the field below, with the red team's measured numbers |

**Nothing is selected.** Selection follows the red team's verdict on THIS
revision.

---

## 1. THE AXIS, quoted where its unit is consumed (D-477)

The unit is **SEAM CROSSINGS** — how many places know how to speak the line
protocol, and which crate owns each.

| the statement that consumes the unit | what it means there |
|---|---|
| `crates/pistol-cli/tests/workspace_shape_tests.rs`, `pistol_arena_manifest_names_only_core_engine_and_cli` — *"pistol-cli is admitted because it owns the line protocol's one spelling (docs/decisions.md D-5, D-167): the arena is a CLIENT of that protocol, and **a second spelling of the verbs in the arena would be a second protocol**"* | the protocol has ONE spelling, in `pistol-cli`; a driver is a CLIENT of it |
| `crates/pistol-arena/src/exchange.rs` — `position_line` builds from `pistol_cli::protocol::POSITION`; `totals_of` reads `pistol_cli::report::INFO_PREFIX` / `TOTALS_MARKER` | the arena is that client, naming the tokens rather than re-spelling them |
| `crates/pistol-arena/Cargo.toml` — `pistol-cli = { workspace = true }` | **and this is why row (c) is dead: a `pistol-cli` dependency on `pistol-arena` would be a cargo dependency CYCLE.** Revision 1 killed (c) on a pinning test, which is a thing criterion 5 says a row may buy its way out of; a cycle is not |

**The red team attacked the axis itself and it survives**, but with a
qualification revision 1 did not state: seam crossings decide (c) and (d)
outright and decide *nothing* between (a), (b), (e), (g) and (h), which all
cross zero new seams. **Among those five the axis is silent and criteria 2-5 do
the work** — so the axis is a filter, not a ranking, and this revision says so.

## 2. WHAT THE FIELD IS RANKED AGAINST

1. **PROTOCOL SPELLINGS.** How many places speak the verbs; one today.
2. **THE JUDGE'S PURITY.** ~~A row that adds a non-verdict output path to the
   arena must say what that costs.~~ **WITHDRAWN AS A DISCRIMINATOR** — the red
   team measured that `bin/arena.rs` **already** ships a non-verdict output path
   (`--replay` over a written report). The criterion did not distinguish any two
   rows and read as though it did. What survives is narrower and is criterion 2':
   **a row must not change what the SPRT path OUTPUTS.**
3. **THE LABEL SEAM.** `exchange::ask` returns `Answer::Move | Forfeit`;
   `totals_of` parses `nodes`, `time`, `depth_turns` and **no score, no pv**.
   Every row says how the label gets out. **No row gets it from `ask` as it
   stands, and no row gets it from `totals_of` without new parsing.**
4. **COLDNESS, REBUILT ON THE MECHANISM (D-540).** The label must not be
   produced by a `go` on a table another `go` warmed. **The mechanism is
   `newgame`, not a fresh process**: `crates/pistol-search/src/tt/mod.rs`'s
   `Table::clear` fills every bucket with `EMPTY` and zeroes `generation` and
   `used`; `newgame` reaches it through the protocol; and
   `crates/pistol-engine/tests/engine_tests.rs`'s
   `new_game_forgets_the_position_and_everything_learned` pins that an engine
   which *"has played a different game and been told `newgame`, must agree node
   for node (D-7)"*. **So coldness costs one line, not a process spawn** — and
   the criterion is no longer "how expensive is coldness" but **"where can a
   `newgame` land without disturbing a game in progress"**, which is a real
   difference between a one-pass and a two-pass row.
5. **WHAT THE PINNING TEST BECOMES.** Updated deliberately, never deleted.

**MEASURED, and this block is the one revision 1 got wrong** (D-291):

- `pistol-arena` is **4,240 lines over 19 `src/*.rs` files**, largest
  `transcript.rs` at 379. Its `pistol-*` dependencies are exactly `pistol-cli`,
  `pistol-core`, `pistol-engine`. **REPRODUCED.**
- `pistol-cli` ships **five binaries**, so a sibling binary is an established
  shape there. **REPRODUCED** — and irrelevant, because (c) dies on the cycle.
- **`crates/pistol-arena/src/lib.rs` declares 22 `pub mod`.** Revision 1 said the
  reusable surface was five items and priced row (b) on that. **CORRECTED.**
  Public and reusable: `channel::Channel` (with `unsolicited`, D-172's own
  guard), `channel::Received` (including `Overlong`), `exchange::ask`,
  `exchange::Answer`, `exchange::position_line`, `transcript::read`, `replay`'s
  walk, and `seats::with_seats`. **Private and therefore the real duplication
  cost: `exchange::totals_of`**, which is where D-80's totals-marker discipline
  lives — *"a driver that billed compute to the wrong one would under-count every
  interrupted iteration"*.
- `Rules.go_line` is a **caller-supplied `&str`**, built in `bin/arena.rs` from
  `config.go_line()`. **REPRODUCED.**
- **`seats::with_seats` sends `pistol_cli::protocol::NEW_GAME` on every fresh
  spawn.** Revision 1 missed this because it grepped for the lower-case word and
  the constant is `NEW_GAME`. **This is decision 8's answer, already
  implemented.**
- **The re-ask loop is NOT unprecedented.** The premise memo called the composite
  operation *"no precedent anywhere in this tree"*; `crates/pistol-arena/src/replay.rs`'s
  `walk` replays a recorded game position by position, asking the engine at each.
  **That memo claim is corrected here rather than left standing**, because row
  (g) is built on the shape it said did not exist.

## 3. THE ROWS

Each row states the MECHANISM, the SEAM COST, the LABEL SEAM, COLDNESS, and its
KILL CONDITION.

### (a) EXTEND `pistol-arena` — one pass, play and label together

**Mechanism.** A second budget in `ArenaConfig` (with the `ARENA_SCHEMA_VERSION`
bump `config.rs` shows is the established move), a per-position record beside
`GameRecord`, a corpus sink, and a widened `Answer`.

**Seam cost.** ZERO new spellings; no manifest change.

**Label seam.** `Answer` widens in place and every existing caller changes,
including the SPRT path.

**Coldness.** **This is the row where coldness is genuinely hard.** A `newgame`
before each label `go` in a one-pass shape lands *inside the game being played*
and wipes the table the game's own next search would use — so either the game is
played cold at every turn (a different game from the SPRT path's) or the label is
warm. **Neither is acceptable under D-540**, and the row's answer must be a
second engine process, which is the cost revision 1 attributed to every row and
in fact belongs only to this one.

**Kill condition.** The one-pass shape cannot give a cold label without a second
engine process; or the widened `Answer` changes the SPRT path's output
(criterion 2').

### (b) NEW CRATE depending on `pistol-arena`

**Mechanism.** `pistol-labels`, depending on `pistol-arena`, `pistol-cli`,
`pistol-core`; reuses `Channel`, `position_line`, `transcript::read`; owns
schema, sink, ledgers.

**Seam cost.** One new workspace member; `workspace_shape_tests.rs` gains a
manifest test.

**Label seam.** Its own totals reader. **The duplication is ONE function —
`totals_of` — and not three**, because `Received::Overlong` and
`Channel::unsolicited` are public and forfeit is an SPRT concept a corpus does
not need. That function carries D-80, so duplicating it duplicates a
defect-bought discipline; that is a real price and it is one line of parsing, not
a read loop.

**Coldness.** Free — it may spawn or `newgame` at will, touching no SPRT run.

**Kill condition.** A second `totals_of` diverges from the arena's, so the two
clients disagree about what a totals line says.

### (c) NEW BINARY inside `pistol-cli` — **DEAD, and by a stronger fact than revision 1 gave**

**Mechanism.** A sibling of `corpus-extract`.

**Seam cost.** **FATAL.** `crates/pistol-arena/Cargo.toml` declares
`pistol-cli = { workspace = true }`, so a `pistol-cli` dependency on
`pistol-arena` is a **cargo dependency cycle** — not a policy, not a test, a
build failure. Revision 1 killed this row on `workspace_shape_tests.rs`, which
criterion 5 explicitly says a row may buy its way out of by updating the test
deliberately; **a cycle cannot be bought out of.** The red team checked all four
escapes — dev-dependency, workspace path, per-`[[bin]]` dependencies, and
optional plus `required-features` — and none reaches a shipped binary.

**Kill condition.** Already fired, and now for the right reason.

### (d) A `tools/` SCRIPT driving the shipped binary

**Mechanism.** Python or shell over `target/release/pistol`, beside
`tools/bench_block.sh`.

**Seam cost.** A third spelling, unchecked by any compiler.

**Label seam.** Trivial — it parses the whole totals line.

**Coldness.** Trivial and free.

**Kill condition — CORRECTED.** Revision 1 killed this on "a protocol token
changes and nothing fails to compile", which is true and weak. **The real kill
is game-over detection**: a driver cannot link `pistol-core` as referee, so it
must discover that a game ended by parsing the error string `set_position`
returns — and an error-string parse is `docs/process.md`'s named tools/ defect
class. Rule 2 forbids re-implementing win detection, so the row has no third
option.

### (e) WIDEN the arena's label seam only, own the pipeline elsewhere

**Mechanism.** A narrow, SPRT-neutral widening so the label fields come out of
one place for both clients, plus a new crate as (b).

**Seam cost.** One new crate; the arena's change confined to one function and one
struct.

**Label seam — RE-PRICED, and this is B3.** Revision 1 said the widening returns
*"the label fields it already parses"*. **It does not parse them.**
`totals_of` returns `(nodes, time, depth_turns)`; score and pv are on the line
and are never read. So this is NEW parsing inside the judge, with a failure mode
the red team names: `totals_of` is a `?`-chain, and a score field that fails to
parse would make the whole function return `None`, **silently suppressing
`compute.add` and zeroing the SPRT report's node counts**.

**Coldness.** Free, as (b).

**Kill condition.** The widening cannot be made output-neutral for the SPRT path.
**The red team could not prove it inherently non-neutral** — its own strongest
failed attack — and gives the shape that works: keep the three existing lookups
load-bearing and make score and pv **non-fatal `Option`s**. So this row is
implementable; revision 1 simply did not know why.

### (f) NULL — WP-2.0 emits no labels

**Mechanism.** None. **Kill condition inverted**: wins if every other row's
fires. In the field so that *"the pipeline has no home worth its cost"* is an
answer the field can express.

### (g) A SECOND MODE of the existing `arena` binary, over a written report

*Added by the round-1 DECISION-RED-TEAM under D-511's precedent.*

**Mechanism.** A third arm in `bin/arena.rs`'s mode match, beside `--config` and
`--replay`: read a report the arena wrote (`transcript::read`), walk each game's
recorded moves through `replay.rs`'s shape, but send `newgame` before each ask
and ask at the **label** `go_line`. The games are produced by the **unmodified**
SPRT path in a separate, earlier run.

**Seam cost.** **ZERO — and it is the only row of which that is true in every
sense.** No new crate, no manifest change, no `[workspace.dependencies]` entry,
no dependency-name change; all four tests in `workspace_shape_tests.rs`
untouched. `bin/arena.rs` already dispatches modes, and `arena-stub-engine`
shows a second `[[bin]]` is an established shape in this crate.

**Label seam.** Either (e)'s non-fatal widening, shared by both clients, or a
label-only reader in the new mode reusing the public `Channel`. **The row does
not need to pick, and this matrix does not pick for it.**

**Coldness.** `newgame` before each label `go`. **And the two-pass decomposition
is what makes that safe**: because the labelling pass runs over a WRITTEN REPORT,
no `newgame` ever lands inside a game the SPRT path is playing. Game isolation is
inherited — `seats::with_seats` already spawns per game and sends `NEW_GAME`.

**What it dissolves.** The premise memo's **decision 4** disappears: the label
budget arrives on the command line exactly as `--workers` does, for the reason
`bin/arena.rs` already gives — *"on the command line because there is no config
document here to state it and no code-side default for a tunable (CLAUDE.md rule
1)"*. No second `ArenaConfig` budget, no schema bump, no second document.

**Kill condition — and it is now ANSWERED from the governing text.** The row dies
if WP-2.0 requires games to be played and labelled in ONE pass. **The dispatch,
now transcribed to `docs/experiments/wp20_dispatches.md`, does not require it**:
*"Plays self-play games"* and *"Emits one record per position"* are separate
numbered requirements and the pilot is called *"full pipeline end to end"*
without saying how many passes that is. **The secondary kill stands and is a
measurement nobody has taken**: whether `transcript.rs`'s report carries enough
to reconstruct every position. `replay.rs` walking recorded games suggests it
does; no run has confirmed it, and the design owes that dry run.

### (h) A SECOND `[[bin]]` inside `pistol-arena`

*Added by the round-1 DECISION-RED-TEAM.*

**Mechanism.** `crates/pistol-arena/src/bin/label.rs`, a third target beside
`arena` and `arena-stub-engine`, consuming the crate's `pub` modules.

**Seam cost.** ZERO new spellings, zero dependency changes — the manifest already
declares two `[[bin]]`s.

**Label seam / coldness.** As (g).

**Kill condition.** Nothing distinguishes it from (g) except whether the mode
belongs behind a flag of one binary or in a binary of its own — a question about
the crate's command surface, not about the pipeline.

---

## 4. RECOMMENDATION

**REGISTERED SLOT — written only after a fresh-context DECISION-RED-TEAM has
attacked THIS revision.** Nothing is selected until then.

What revision 2 may say is what it measured, and the field has changed shape
since revision 1 said it:

- **Two rows are dead on facts, not preferences.** (c) is a cargo dependency
  cycle; (d) must parse an error string to learn a game ended, which rule 2 and
  `docs/process.md` between them foreclose.
- **Five rows cross zero new seams** — (a), (e), (g), (h) and, at the cost of one
  workspace member, (b). **The axis does not rank them**, and revision 1 implied
  it did.
- **Coldness is cheap and its cost is not the discriminator revision 1 made it.**
  `newgame` is a true wipe pinned node-for-node against a fresh engine. What
  discriminates is **where a `newgame` may land**: in a two-pass row it lands
  between recorded games and disturbs nothing; in the one-pass row (a) it lands
  inside a game in progress, and that row alone must buy a second engine process.
- **The label seam costs new parsing in every row**, because nothing parses score
  or pv today. The question is where that parsing lives and whether it can be
  made non-fatal.

## 5. WHAT THIS MATRIX DOES NOT DECIDE — SEVEN, and revision 1 said eight

The premise memo's forced decisions that survive to the design:

| # | decision | status |
|---|---|---|
| 6 | which `book_v2` range WP-2.0 consumes | open — and constrained by D-540: pipeline experiment configs only, never a committed one |
| 9 | what the label's `depth` and `nodes` mean, given `Provenance` is off the wire | open |
| 10 | what the score field is CALLED in the schema — `cp` is not centipawns, the sign is root-side-to-move, `mate T` counts both sides' turns | open, **and the memo's own note stands: a wrong name here makes every label in the corpus wrong in a way no loader test catches** |
| 11 | one record per PLY or per TURN — D-477's own unit question | open |
| 12 | the corpus manifest's digest boundary | open |
| — | storage format and schema version; label policy; transposition dedup | open (the dispatch's own list) |

**Three corrections to revision 1's deferral list.** **Decision 4** (how two
budgets are expressed) is **dissolved, not deferred**, if (g) or (h) is selected
— the label budget is a command-line argument. **Decision 7** (game-over
detection) is **settled and revision 1 dropped it**: rule 2 forbids
re-implementing win detection, the arena links `pistol-core` as referee, and any
row but (d) inherits that. **Decision 8** (process isolation) is **not settled by
D-540 alone**, as revision 1 implied — D-540 fixes the REQUIREMENT (cold by
construction); `seats::with_seats` sending `NEW_GAME` per spawn is the
IMPLEMENTATION, and the two are different claims.

**And one thing this matrix does not touch**: the census-minimum rule for
detector round 3 (D-537). It lands in WP-2.0's design regardless of this
selection, before any corpus exists, so it cannot be fitted later.
