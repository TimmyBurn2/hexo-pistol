# WP-2.0 — OPTION MATRIX: where the label pipeline LIVES

**REVISION 1. Nothing is selected.** CLAUDE.md's Process makes a named decision
with more than one viable option a matrix decision, attacked by a fresh-context
DECISION-RED-TEAM **before** selection; *"an option adopted without a matrix, or a
matrix never attacked, is the same breach as silent architecture drift."*

**Why this decision and not the others.** The WP-2.0 premise memo enumerates
twelve forced decisions. D-539 and D-540 settle four of them outright — the
census leaves the package (1, 2), seeds attach to sampling only (5), and labels
are cold by construction (8). Of the rest, **only decision 3 is architectural**:
it decides whether the pipeline is a new home, an extension of the crate
CLAUDE.md calls *"the judge every later work package is tried by"*, or a second
spelling of the protocol driver. The others are schema and policy choices the
DESIGN records once this one is settled, and they do not survive being decided
before it.

---

## 1. THE AXIS, quoted where its unit is consumed (D-477)

The unit this field is ranked in is **SEAM CROSSINGS** — how many places would
end up knowing how to speak the line protocol, and which crate owns each. It is
the unit because the project has one registered rule about it and one test that
enforces the rule.

| the statement that consumes the unit | what it means there |
|---|---|
| `crates/pistol-cli/tests/workspace_shape_tests.rs`, `pistol_arena_manifest_names_only_core_engine_and_cli` — *"pistol-cli is admitted because it owns the line protocol's one spelling (docs/decisions.md D-5, D-167): the arena is a CLIENT of that protocol, and **a second spelling of the verbs in the arena would be a second protocol**"* | the protocol has ONE spelling, in `pistol-cli`, and a driver is a CLIENT of it |
| `crates/pistol-arena/src/exchange.rs` — `position_line` builds its line from `pistol_cli::protocol::POSITION`, and `totals_of` reads `pistol_cli::report::INFO_PREFIX` / `TOTALS_MARKER` | the arena is that client, and it names the protocol's tokens from `pistol-cli` rather than re-spelling them |
| `crates/pistol-cli/tests/workspace_shape_tests.rs`, `pistol_cli_manifest_names_only_...` — pistol-cli's dependency list is pinned to exactly `pistol-core`, `pistol-engine`, `serde`, `serde_path_to_error`, `toml`, with the message *"a dependency arrived in pistol-cli without a line in docs/decisions.md saying why"* | **`pistol-cli` may not depend on `pistol-arena`**, so a driver living inside `pistol-cli` cannot reuse the arena's channel |

**A row that adds a seam crossing is a different kind of row from one that moves
an existing one**, and this matrix keeps them apart.

## 2. WHAT THE FIELD IS RANKED AGAINST

Five criteria, fixed before any row is scored.

1. **PROTOCOL SPELLINGS.** How many places speak the verbs. The registered
   answer today is one, and a row that makes it two is a breach unless it argues
   the case.
2. **THE JUDGE'S PURITY.** `crates/pistol-arena/src/lib.rs` states the crate's
   purpose — *"Hard Rule 6 makes SPRT over paired balanced openings the judge of
   every search and eval change, so this crate is the judge every later work
   package is tried by"* — and its own boundary section, *"What this crate is
   not"*. A row that puts a non-verdict output path into the judge must say what
   that costs.
3. **THE LABEL SEAM.** `exchange::ask` returns `Answer::Move(Turn) | Forfeit`,
   and `totals_of` parses `nodes`, `time`, `depth_turns` from the totals line —
   **discarding the `score` and `pv` that are on that same line**. Every row must
   say how the label gets out, because no row gets it from `ask` as it stands.
4. **COLD BY CONSTRUCTION (D-540).** The label must not be produced by a `go` on
   a table another `go` warmed. Each row states the mechanism that makes that a
   property of its shape rather than a thing a criterion re-establishes.
5. **WHAT THE PINNING TEST BECOMES.** Every row that changes a dependency list
   changes `workspace_shape_tests.rs`. The dispatch's own words for WP-2.0b
   apply here too: the test is **UPDATED deliberately, not deleted**, and a row
   states the new shape it would pin.

**MEASURED, so no row is ranked on a guess** (D-291: an estimate that could have
been measured in seconds is a finding):

- `pistol-arena` is **4,240 lines over 19 `src/*.rs` files**, largest
  `transcript.rs` at 379; its `pistol-*` dependencies are exactly
  `pistol-cli`, `pistol-core`, `pistol-engine`.
- `pistol-cli` already ships **five binaries** — `pistol`, `corpus-audit`,
  `corpus-census`, `corpus-extract`, `random-openings` — so a sibling binary is
  an established shape there, not a new one.
- The reusable surface the arena already exports is `channel::Channel`,
  `channel::Received`, `exchange::Answer`, `exchange::ask`,
  `exchange::position_line`. **`ask` is where the label dies.**
- `Rules.go_line` is a **caller-supplied `&str`** (`game.rs`, built in
  `bin/arena.rs` from `config.go_line()`), so a SECOND budget needs no schema
  change to be *expressible* — only a second `Rules`. That is a fact about the
  seam, not a recommendation.

---

## 3. THE ROWS

Each row states the MECHANISM, the SEAM COST, what it does about the LABEL SEAM
and about COLDNESS, and its KILL CONDITION.

### (a) EXTEND `pistol-arena`

**Mechanism.** Add to the existing crate: a second budget in `ArenaConfig` (with
the `ARENA_SCHEMA_VERSION` bump `config.rs` shows is the established move), a
per-position record beside `GameRecord`, a corpus sink, and a widened
`exchange::Answer` carrying score and pv.

**Seam cost.** ZERO new protocol spellings — the arena already is the client.
No dependency changes, so `workspace_shape_tests.rs` is untouched.

**Label seam.** `Answer` widens in place; every existing caller must be updated,
and the SPRT path starts carrying a field it does not use.

**Coldness.** The arena respawns a process per game (`seats.rs`), not per
position, so a per-position cold guarantee is NEW work inside the judge.

**Kill condition.** The added output path changes an SPRT-relevant behaviour, or
the crate's own boundary statement has to be edited to admit a non-verdict
output — at which point the row is amending the purpose of *"the judge every
later work package is tried by"* to make room for a data pipeline.

### (b) NEW CRATE depending on `pistol-arena`

**Mechanism.** A new workspace member (`pistol-labels`) depending on
`pistol-arena`, `pistol-cli`, `pistol-core`. It reuses `Channel` and
`position_line`, drives its own read loop for the label `go`, and owns the
schema, the sink and the ledgers.

**Seam cost.** ONE new crate that speaks the protocol — but it names the
protocol's tokens through `pistol_cli::report` and `pistol_cli::protocol` exactly
as the arena does, so whether that is a "second spelling" or a second client is
the question this row lives or dies on. `workspace_shape_tests.rs` gains a
manifest test for the new crate.

**Label seam.** Its own read loop, so `Answer` need not widen and the SPRT path
is untouched — at the cost of a second read loop that must handle the same
forfeit, overlong-line and out-of-turn cases `exchange::ask` already handles.
**That duplication is this row's real price and it is not small**: those cases
are D-172's and D-80's, each bought with a defect.

**Coldness.** Free to spawn a fresh process per label without touching the judge.

**Kill condition.** The second read loop's error handling diverges from
`exchange::ask`'s, which makes the two clients disagree about what a forfeit is.

### (c) NEW BINARY inside `pistol-cli`

**Mechanism.** A sibling of `corpus-extract` in `crates/pistol-cli/src/bin/`.

**Seam cost.** **FATAL BY MEASUREMENT, and this row is in the field to record
that.** `pistol-cli`'s dependency list is pinned to five names by a test whose
failure message is *"a dependency arrived in pistol-cli without a line in
docs/decisions.md saying why"*, and `pistol-arena` is not among them — so this
binary cannot reuse `Channel` and must re-implement the process driving. That is
the second spelling the arena's own pinning test exists to forbid, arriving in
the crate that owns the first.

**Label seam / coldness.** Not reached.

**Kill condition.** Already fired, by inspection.

### (d) A `tools/` SCRIPT driving the shipped binary

**Mechanism.** Python or shell over `target/release/pistol`'s stdin/stdout,
beside `tools/bench_block.sh` — which already does exactly this shape, one
position at a time.

**Seam cost.** A THIRD spelling, in a language with no compiler to check it
against `pistol_cli::report`'s constants. `bench_block.sh` is the precedent FOR
the shape and also the warning: it hard-codes the protocol's words.

**Label seam.** Trivial — it parses the whole `info totals` line and keeps what
it wants.

**Coldness.** Trivial and free: a process per label is what a script does anyway.

**Kill condition.** A protocol token changes and nothing fails to compile. Rule 9
and the tools/ review checklist apply, and `tools/` has no schema-versioned
output discipline.

### (e) EXTEND the arena for the SEAM ONLY, own the pipeline elsewhere

**Mechanism.** A minimal, SPRT-neutral widening of `exchange::ask` so the label
fields it already parses are returned rather than dropped, plus a new crate (as
(b)) that owns schema, sink, ledgers and the label budget. The arena gains no
output path, no second budget and no corpus concept.

**Seam cost.** One new crate; the arena's change is confined to one function's
return type and the struct it returns.

**Label seam.** Fixed at the place the memo found it — *"the label this package
exists to collect is already on a wire the arena is already reading and already
throwing away"* — and fixed once, for both clients.

**Coldness.** The new crate spawns per label; the arena is unchanged.

**Kill condition.** Widening `Answer` cannot be made byte-identical for the SPRT
path, i.e. the change is not SPRT-neutral after all.

### (f) NULL — WP-2.0 emits no labels

**Mechanism.** None. **Kill condition inverted**: this row wins if every other
row's kill condition fires, and it is in the field so that *"the pipeline has no
home that does not cost more than it is worth"* is an answer the field can
express rather than a conclusion argued outside it.

---

## 4. RECOMMENDATION

**REGISTERED SLOT — written only after a fresh-context DECISION-RED-TEAM has
attacked this revision, and quoting its rows.** Nothing is selected until then.

What this revision may say is what it measured: **row (c) is dead by inspection**
against a pinning test, and **rows (a), (b), (d) and (e) differ mainly in where
they pay for the label seam** — a widened `Answer` inside the judge, a duplicated
read loop outside it, an unchecked third spelling, or one narrow SPRT-neutral
widening. Which of those prices this project should pay is the red team's to
attack and the operator's to confirm.

## 5. WHAT THIS MATRIX DOES NOT DECIDE

The other eight forced decisions of the premise memo — schema version and
storage format, label policy, transposition dedup, the per-ply-or-per-turn
record axis, the score field's NAME (the memo's decision 10, where `cp` is not
centipawns and a wrong name makes every label wrong in a way no loader test
catches), provenance, the `book_v2` range, and the corpus manifest's digest
boundary. **They belong to the DESIGN and the design is written after this
selection**, because several of them read differently depending on which crate
owns the record.

**And one thing this matrix must not be read as touching**: the census-minimum
rule for detector round 3 (D-537). That rule lands in WP-2.0's design regardless
of this decision, before any corpus exists, so it cannot be fitted later.
