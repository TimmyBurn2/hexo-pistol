# WP-2.0 premise memo — the five premises, quoted where their units are consumed

**Revision.** `2434e32300f9690df10e9ee1d45105533ec3cfbb` (`dev`), which is HEAD at the
time of writing. **Tree state:** clean except for ONE untracked file,
`docs/trigger_coverage_ledger.md` — the D-536 ledger, written but not yet committed.
No tracked file is modified; no premise below is quoted from an uncommitted change.

**What was run.** `git`, `/usr/bin/grep`, `git grep`, `sed`, `awk`, `python3`, `cat -n`,
and file reads. Every recorded search used `/usr/bin/grep` or `git grep` with
`LC_ALL=C sort` (D-265).

**What was refused.** `cargo` in every form, and `tools/ci.sh` — a CI run owns the build.
So no claim below rests on a build, a test run, or a fresh measurement. Every figure
quoted is read out of a committed source file, a committed document, or an artifact
already on disk. Where a premise would need a run to settle it, this memo says so
rather than guessing.

**THE CITATIONS WERE MECHANICALLY AUDITED, AND THE AUDIT FOUND DEFECTS IN THIS
MEMO.** Every `path:line` citation appearing inside a quotation block below — **396 of
them** — was re-read from the file at this revision and compared to the text quoted
beside it. The first pass over a 35-citation sample found **3 wrong**; a full pass then
found **4 more blocks** off by one (`crates/pistol-cli/src/report.rs`,
`crates/pistol-engine/src/engine.rs`, `docs/book_v2_ledger.md`, and
`crates/pistol-search/src/info.rs`, which had a duplicated line number). All are
corrected, and a re-run reports **396 checked, 0 mismatched, 1 legitimate elision**.
This is recorded rather than quietly fixed because it is D-500's own repeat defect —
*"the matrix cited that overflow at `grid.rs:112`, a PRE-`rustfmt` line number, against
a POST-`rustfmt` governing revision — the defect D-291, D-318 and D-324 already name
three times"* (`docs/decisions.md:1068`) — arriving in the document whose entire
purpose is that citations are right. A reader may take the line numbers below as
checked by machine; they may not take that as evidence the READING of each quotation
is right, which is a reviewer's job and not a script's.

**What this memo is.** D-477's exercise: *"a matrix's own axis is a premise and is
quoted at `file:line` like any other, and where the axis is a unit … the quotation
names the line where that unit is CONSUMED"*
(`docs/decisions.md:1022`). What follows quotes each premise at the line that
consumes it.

---

## VERDICT TABLE

| premise | verdict |
|---|---|
| **P1** — the protocol seam | **HOLDS.** Score, best move, depth and nodes are all on the wire, in one named-field line. |
| **P2** — the census seam | **FAILS as stated in requirement 3.** The census is reachable ONLY by linking `pistol-search`; it is not on the wire, not in any config, and `pistol-cli` does not even depend on the crate that owns it. And the census row carries NO position identity, which D-537 requires. |
| **P3** — `book_v2` and its ledger | **HOLDS WITH A GAP.** The book, its sha pin, its grammar and a contiguous-range mechanism are all real. But NO committed config references v2, the ledger is empty, and the book's 4500 was sized for two other claimants. |
| **P4** — deep-search scores and cost shape | **HOLDS WITH A GAP.** Scores, units and the cost shape are all pinned; `Provenance` is the one field a label would want that the wire does not carry. |
| **P5** — the arena | **HOLDS FOR THE SEAM, FAILS FOR THE RECORD.** It already drives engines as black-box subprocesses over the line protocol — WP-2.0's hardest constraint is met by shipped, CI-gated code. It emits no per-position record, reads only three fields off `info totals`, and has no seed. |

---

# P1 — THE PROTOCOL SEAM: **HOLDS**

A driver can play a complete game and extract a deep-search label using only the line
protocol. Every field the package's requirement 2 names — score, best move, depth,
nodes — is on the wire today, and an existing committed instrument already drives the
shipped binary this exact way.

## P1.1 Every verb the protocol accepts

Five verbs, and the set is a named constant:

```
crates/pistol-cli/src/protocol.rs:9    pub const HANDSHAKE: &str = "pistol";
crates/pistol-cli/src/protocol.rs:11   pub const NEW_GAME: &str = "newgame";
crates/pistol-cli/src/protocol.rs:13   pub const POSITION: &str = "position";
crates/pistol-cli/src/protocol.rs:15   pub const GO: &str = "go";
crates/pistol-cli/src/protocol.rs:17   pub const QUIT: &str = "quit";
crates/pistol-cli/src/protocol.rs:20   pub const VERBS: [&str; 5] = [HANDSHAKE, NEW_GAME, POSITION, GO, QUIT];
```

The dispatch that consumes them — `crates/pistol-cli/src/protocol.rs:89-127` — is a
closed match, and an unknown verb is a named refusal, never a no-op:

```
crates/pistol-cli/src/protocol.rs:119       other => Err(protocol(
crates/pistol-cli/src/protocol.rs:122                   "unknown verb `{}`; the verbs are {}",
```

What each answers:

| verb | consumed at | answers |
|---|---|---|
| `pistol` | `protocol.rs:93-97` → `handshake`, `protocol.rs:136-150` | five `id` lines (`name`, `version`, `protocol`, `mode`, `budgets`) plus whatever the binary added, then `pistolok` |
| `newgame` | `protocol.rs:98-102` → `self.engine.new_game()` | nothing on the wire |
| `position` | `protocol.rs:103-106` → `Session::position`, `protocol.rs:153-157` | nothing on success; `error …` on refusal |
| `go` | `protocol.rs:107-110` → `Session::go`, `protocol.rs:168-176` | one `info` line per completed depth, then one `info totals` line, then one `bestmove` line |
| `quit` | `protocol.rs:111-114` | nothing; ends the conversation |

**A refusal is an output line and the engine stays alive** —
`crates/pistol-cli/src/protocol.rs:78`'s doc: *"Never fails: a refusal is an output
line, because the engine stays alive (docs/decisions.md D-5)"*, implemented at
`protocol.rs:81-84`. The refusal goes to STDOUT, which is the hazard
`tools/bench_block.sh:8-16` records by name.

## P1.2 How a position is set — and how a move is made

**There is no move verb.** A position is stated whole, every time. The grammar is
`PositionSpec`'s `FromStr`, consumed at `crates/pistol-cli/src/protocol.rs:154`:

```
crates/pistol-cli/src/protocol.rs:154        let spec = PositionSpec::from_str(rest)
```

and the two forms are fixed at `crates/pistol-engine/src/position_token.rs:10-14`:

```
crates/pistol-engine/src/position_token.rs:10   pub const START_FORM: &str = "start";
crates/pistol-engine/src/position_token.rs:12   pub const MOVES_KEYWORD: &str = "moves";
crates/pistol-engine/src/position_token.rs:14   pub const SET_FORM: &str = "set";
```

dispatched at `crates/pistol-engine/src/position_token.rs:67-77`:

```
crates/pistol-engine/src/position_token.rs:68       Some((&START_FORM, rest)) => parse_start(rest).map_err(reject),
crates/pistol-engine/src/position_token.rs:69       Some((&SET_FORM, rest)) => parse_set(rest).map_err(reject),
```

`start` is the canonical form and is the one a game driver wants —
`crates/pistol-engine/src/position.rs:8-10`: *"The game from the beginning, as the
turns that were played. The canonical encoding (docs/decisions.md D-6); it always
names a position at a turn boundary"*. Its grammar is consumed at
`crates/pistol-engine/src/position_token.rs:84-98`:

```
crates/pistol-engine/src/position_token.rs:87       Some((&MOVES_KEYWORD, [])) => {
crates/pistol-engine/src/position_token.rs:90       Some((&MOVES_KEYWORD, tokens)) => tokens
crates/pistol-engine/src/position_token.rs:92           .map(|token| token.parse::<Turn>().map_err(|error| error.why))
```

so the line is `position start` (empty board) or
`position start moves <turn> <turn> …`. A turn token is one cell or two joined by `/`,
consumed at `crates/pistol-core/src/turn.rs:229-242`:

```
crates/pistol-core/src/turn.rs:107   pub const PAIR_SEPARATOR: char = '/';
crates/pistol-core/src/turn.rs:229       let Some((left, right)) = token.split_once(PAIR_SEPARATOR) else {
crates/pistol-core/src/turn.rs:230           return Ok(Turn::Single(cell(token)?));
```

and a pair must be spelled canonically — `crates/pistol-core/src/turn.rs:158-159`:

```
crates/pistol-core/src/turn.rs:158           Turn::Single(_) => true,
crates/pistol-core/src/turn.rs:159           Turn::Pair(first, second) => first < second,
```

**So "making a move" is: the driver appends the turn to its own move list and
re-issues the whole `position start moves …` line.** That is exactly what the
interim HeXO adapter already does —
`tools/sealbot/pistol_hexo_adapter.py:116-119`:

```
tools/sealbot/pistol_hexo_adapter.py:116        position = position_line([*self.setup, *self.places])
tools/sealbot/pistol_hexo_adapter.py:117        if position:
tools/sealbot/pistol_hexo_adapter.py:118            self.send(position)
tools/sealbot/pistol_hexo_adapter.py:119        self.send(f"go movetime {int(op['time_ms'])}")
```

with the move-list spelling built at `tools/sealbot/pistol_hexo_adapter.py:157-163`.

**A consequence the design must own (see decision 1 in §"what the design must
decide"): the protocol will not tell a driver the game is over.** `set_position`
refuses a decided position by name —
`crates/pistol-engine/src/position.rs:68-73`:

```
crates/pistol-engine/src/position.rs:68           if let Outcome::Win { winner, turn } = state.outcome() {
crates/pistol-engine/src/position.rs:69               return Err(EngineError::illegal_position(format!(
crates/pistol-engine/src/position.rs:70                   "{winner} completed a line on turn {turn}: a won position is terminal, so \
crates/pistol-engine/src/position.rs:71                    there is no move to ask this engine for (rule 4)"
```

— so a driver that plays on into a win gets an `error`, not a result. The clean
answer already exists and is not an engine change: `pistol-cli` links
`pistol-core` for exactly this, `crates/pistol-cli/Cargo.toml:19-21`:

```
crates/pistol-cli/Cargo.toml:19   # The turn and stone tokens, and the rules a driver validates its game against
crates/pistol-cli/Cargo.toml:20   # (CLAUDE.md rule 2).
crates/pistol-cli/Cargo.toml:21   pistol-core = { workspace = true }
```

and the `Engine` trait says the same thing at
`crates/pistol-engine/src/engine.rs:24-29`: *"A shared borrow of the rules' own
state, so a driver checks the game it is running against pistol-core rather than
against a second opinion (CLAUDE.md rule 2)."* **Linking `pistol-core` is required
by rule 2 and is not a breach of the black-box constraint, which is about engine
internals.**

## P1.3 What a `go` reports — score, bestmove, depth, nodes are ALL on the wire

**All four are on the wire.** The one place a report line is built is
`crates/pistol-cli/src/report.rs:82-97`, and it is an explicit ordered field list:

```
crates/pistol-cli/src/report.rs:82       let mut line = format!(
crates/pistol-cli/src/report.rs:83           "{INFO_PREFIX}{marker} depth_turns {} seldepth {} nodes {}{solver_field} {NPS_FIELD} {} \
crates/pistol-cli/src/report.rs:84            {TIME_FIELD} {} hashfull {} score {} pv",
crates/pistol-cli/src/report.rs:85           info.depth_turns,
crates/pistol-cli/src/report.rs:86           info.seldepth_turns,
crates/pistol-cli/src/report.rs:87           info.nodes,
crates/pistol-cli/src/report.rs:88           info.nps,
crates/pistol-cli/src/report.rs:89           info.time_ms,
crates/pistol-cli/src/report.rs:90           info.hashfull_permille,
crates/pistol-cli/src/report.rs:91           score_token(info.score),
crates/pistol-cli/src/report.rs:92       );
crates/pistol-cli/src/report.rs:93       for turn in &info.pv {
```

The contract is stated once at `crates/pistol-cli/src/report.rs:32-38`:

```
crates/pistol-cli/src/report.rs:33   /// The key set is exactly this ordered list, and a driver keys on the names
crates/pistol-cli/src/report.rs:34   /// rather than on positions: `[totals] depth_turns seldepth nodes nps time
crates/pistol-cli/src/report.rs:35   /// hashfull score pv`. `score` is one of `cp <n>`, `mate <turns>` or `-mate
crates/pistol-cli/src/report.rs:36   /// <turns>`; `pv` comes last because it is variable-length, and it is never
crates/pistol-cli/src/report.rs:37   /// empty — a completed iteration always has a move
```

- **DEPTH** — `depth_turns` (and `seldepth`), `report.rs:83, 85, 86`.
- **NODES** — `nodes`, `report.rs:83, 87`. When the solver gate is on, the line
  additionally carries `search_nodes` / `solver_nodes` and the four call counters,
  `report.rs:62-81` — **and only then**, `report.rs:62`:
  `let solver_field = if info.solver_nodes > 0 {`.
- **SCORE** — `score`, `report.rs:84, 91`, spelled by `score_token`,
  `report.rs:153-159`.
- **BEST MOVE** — twice over. As `pv`'s first element (`report.rs:93-96`; and
  `SearchOutcome`'s own doc at `crates/pistol-search/src/info.rs:218`: *"`best` is
  always `info.pv[0]`"*), and as its own line:

```
crates/pistol-cli/src/report.rs:11   pub const BESTMOVE_PREFIX: &str = "bestmove";
crates/pistol-cli/src/report.rs:107  pub fn bestmove_line(best: Turn) -> String {
crates/pistol-cli/src/report.rs:108      format!("{BESTMOVE_PREFIX} {best}")
```

The order in which a `go` emits them is fixed at
`crates/pistol-cli/src/protocol.rs:170-174`:

```
crates/pistol-cli/src/protocol.rs:170           let outcome = self
crates/pistol-cli/src/protocol.rs:171               .engine
crates/pistol-cli/src/protocol.rs:172               .go_reporting(budget, &mut |info| out(&info_line(info)))?;
crates/pistol-cli/src/protocol.rs:173           out(&totals_line(&outcome.info));
crates/pistol-cli/src/protocol.rs:174           out(&bestmove_line(outcome.best));
```

**The line a label must be read from is the `info totals` one, not the last plain
`info`.** The marker exists for exactly that reason —
`crates/pistol-cli/src/report.rs:29`: `pub const TOTALS_MARKER: &str = "totals";`,
whose rationale at `report.rs:20-28` is: *"a driver keying on field names could not
tell which one to bill the search to; per-side compute is a reporting requirement
(CLAUDE.md rule 6), so the distinction is in the grammar rather than in the reader's
memory of what came before"*. And `crates/pistol-cli/src/protocol.rs:161-167` says
which line carries what: the totals line has *"the last completed depth's line and
score, with the **totals** for the whole search"*.

**One flush per input line**, so a request/response driver cannot deadlock —
`crates/pistol-cli/src/serve.rs:18-21` and `serve.rs:61`: `output.flush()?;`.

## P1.4 The budget grammar

Three kinds, and `nodes` is spelled `nodes`:

```
crates/pistol-cli/src/budget_token.rs:10   pub const DEPTH_TURNS_BUDGET: &str = "depth_turns";
crates/pistol-cli/src/budget_token.rs:12   pub const NODES_BUDGET: &str = "nodes";
crates/pistol-cli/src/budget_token.rs:14   pub const MOVETIME_BUDGET: &str = "movetime";
```

consumed at `crates/pistol-cli/src/budget_token.rs:56-70`:

```
crates/pistol-cli/src/budget_token.rs:58       DEPTH_TURNS_BUDGET => Ok(Budget::DepthTurns(count(line, kind, amount)?)),
crates/pistol-cli/src/budget_token.rs:59       NODES_BUDGET => Ok(Budget::Nodes(count(line, kind, amount)?)),
crates/pistol-cli/src/budget_token.rs:60       MOVETIME_BUDGET => Ok(Budget::MovetimeMs(count(line, kind, amount)?)),
```

The line shape is exactly `go <kind> <amount>` — two words, no more, no fewer,
`crates/pistol-cli/src/budget_token.rs:34-52`:

```
crates/pistol-cli/src/budget_token.rs:35       [] => Err(EngineError::BudgetMissing),
crates/pistol-cli/src/budget_token.rs:44       [kind, amount] => budget_of(line, kind, amount),
crates/pistol-cli/src/budget_token.rs:45       [_, _, extra, ..] => Err(protocol(
```

An absent budget is a named error, never a default —
`crates/pistol-cli/src/budget_token.rs:27-31`: *"A `go` with no budget at all is
[`EngineError::BudgetMissing`] … a budget is always explicit"* (CLAUDE.md rule 1,
D-4). Note the protocol spelling is `movetime`, while the engine-side key is
`budget.movetime_ms` (`crates/pistol-engine/src/budget.rs:32`) — two spellings for
one kind, and a driver writes the protocol one.

**Instrument mode refuses `movetime` by name**, which is what makes a node-budgeted
pipeline the only kind an instrument config will run —
`crates/pistol-engine/src/budget.rs:72-79`:

```
crates/pistol-engine/src/budget.rs:74           EngineMode::Instrument if !self.is_reproducible() => {
crates/pistol-engine/src/budget.rs:75               Err(EngineError::InstrumentBudgetUnsupported)
```

with reproducibility fixed at `crates/pistol-engine/src/budget.rs:37-42`. The
handshake advertises which kinds this mode honours —
`crates/pistol-cli/src/protocol.rs:142-145` reading
`crates/pistol-cli/src/budget_token.rs:87-93` — so a driver can assert
`id budgets depth_turns nodes` before it runs, and a real transcript shows exactly
that: `artifacts/wp19b_bench_landing_v2.txt`, line
`bench_delta: identity id budgets depth_turns nodes`.

## P1.5 `newgame` exists, and what it clears

```
crates/pistol-cli/src/protocol.rs:98            NEW_GAME => {
crates/pistol-cli/src/protocol.rs:99                no_arguments(line, verb, rest)?;
crates/pistol-cli/src/protocol.rs:100               self.engine.new_game();
```

The contract is `crates/pistol-engine/src/engine.rs:31-38`:

```
crates/pistol-engine/src/engine.rs:31   /// Start a new game: the initial position, and nothing remembered from the
crates/pistol-engine/src/engine.rs:32   /// last one.
crates/pistol-engine/src/engine.rs:34   /// Infallible, and total. Everything a new game needs to forget is
crates/pistol-engine/src/engine.rs:35   /// forgotten here, which is what the determinism law requires of it: two
crates/pistol-engine/src/engine.rs:36   /// searches of the same position, one of them after a different game and a
crates/pistol-engine/src/engine.rs:37   /// `new_game`, must agree node for node (CLAUDE.md rule 4,
crates/pistol-engine/src/engine.rs:38   /// docs/decisions.md D-7).
```

What it actually clears, at the search: `crates/pistol-search/src/search.rs:229-239`
— the transposition table, the ordering heuristics, and the solver's own state:

```
crates/pistol-search/src/search.rs:230       pub fn clear(&mut self) {
crates/pistol-search/src/search.rs:231           self.table.clear();
crates/pistol-search/src/search.rs:232           self.heuristics.clear();
crates/pistol-search/src/search.rs:236           if let Some(solver) = self.solver.as_mut() {
crates/pistol-search/src/search.rs:237               solver.reset();
```

## P1.6 The seam is not hypothetical — a committed instrument already uses it

`tools/bench_block.sh` is a black-box driver over this exact protocol, and it is the
instrument every Stage-3 figure was taken with. The four-line conversation:

```
tools/bench_block.sh:247            printf 'newgame\nposition %s\ngo %s\nquit\n' "$entry" "$BUDGET" |
tools/bench_block.sh:248                "$ENGINE" --config "$CONFIG" >"$WORK/out" 2>&1 || true
```

the refusal guard (which exists because refusals go to STDOUT):

```
tools/bench_block.sh:253            errors="$(grep -c '^error ' "$WORK/out" || true)"
tools/bench_block.sh:254            if [ "$errors" -ne 0 ]; then
```

and the label extraction:

```
tools/bench_block.sh:260            count="$(grep -c '^info totals ' "$WORK/out" || true)"
tools/bench_block.sh:264            totals="$(sed -n 's/^info totals //p' "$WORK/out")"
```

Its own header states the isolation discipline the pipeline inherits,
`tools/bench_block.sh:28-31`:

```
tools/bench_block.sh:28   # ONE INVOCATION PER (ENTRY, REP), never one session over the whole fixture:
tools/bench_block.sh:29   # the defect above is a refused entry LEAKING into the next entry's number, and
tools/bench_block.sh:30   # per-invocation isolation plus a per-invocation refusal check is what removes
tools/bench_block.sh:31   # the class rather than narrowing it.
```

and it engine-digests what it ran, `tools/bench_block.sh:170` and `:235-238` —
which is where requirement 4's "engine SHA" already has an implementation to copy.

**P1 VERDICT: HOLDS.** Score, best move, depth and nodes are all on the wire in one
named-field line; the budget grammar has `nodes`; `newgame` exists and clears what
D-7 requires; and a committed instrument already plays this seam. No engine change is
needed for requirement 1, 2 or 4's protocol half.

---

# P2 — THE CENSUS SEAM: **FAILS** (as requirement 3 is written), TWICE OVER

Requirement 3 says census logging runs on every game, and the package says the driver
never links engine internals. **Those two cannot both be true today.** And separately,
the census row does not carry the position identity D-537 demands.

## P2.1 The hard question, answered: the census is reachable ONLY by linking `pistol-search`

Three independent facts, each quoted:

**(a) The switch is a `pistol-search` method, and no config can reach it.**

```
crates/pistol-search/src/search.rs:199       /// Collect one row per solver trigger firing from here on.
crates/pistol-search/src/search.rs:201       /// An INSTRUMENT and never a knob: no search reads a row back, so a
crates/pistol-search/src/search.rs:202       /// searcher collecting a census answers exactly what one not collecting
crates/pistol-search/src/search.rs:203       /// answers (CLAUDE.md rule 4). Off unless a caller asks, and no committed
crates/pistol-search/src/search.rs:204       /// config can ask — the only callers are this crate's own tests and the
crates/pistol-search/src/search.rs:205       /// `trigger_census` example.
crates/pistol-search/src/search.rs:206       pub fn collect_trigger_census(&mut self) {
crates/pistol-search/src/search.rs:207           self.census = Some(Vec::new());
crates/pistol-search/src/search.rs:208       }
```

and the reader:

```
crates/pistol-search/src/search.rs:210       /// The rows collected since the last take, leaving collection ON.
crates/pistol-search/src/search.rs:212       /// # Panics
crates/pistol-search/src/search.rs:213       /// If no census was asked for. A caller that reads a census it never
crates/pistol-search/src/search.rs:214       /// started has a bug in ITS ordering, and an empty vector would be that
crates/pistol-search/src/search.rs:215       /// bug wearing a plausible answer (CLAUDE.md rule 3).
crates/pistol-search/src/search.rs:216       pub fn take_trigger_census(&mut self) -> Vec<crate::census::TriggerObservation> {
crates/pistol-search/src/search.rs:220               .expect("take_trigger_census without collect_trigger_census");
```

The field's own doc repeats the "no shipped path" claim —
`crates/pistol-search/src/search.rs:71-74`: *"`None` in every shipped path, and no
search ever reads a row back"*.

**(b) The line protocol cannot carry it, because `report.rs` renders an explicit
field list.** `crates/pistol-cli/src/report.rs:82-97` (quoted in §P1.3) names every
field; no census field appears. The general principle is stated at
`crates/pistol-search/src/info.rs:7-12` for the sibling stage counters:

```
crates/pistol-search/src/info.rs:7    /// All zero under `CandidatePolicy::Radius`, where the staged dispatch never
crates/pistol-search/src/info.rs:8    /// runs. **The line protocol does not carry these** — `report.rs` renders an
crates/pistol-search/src/info.rs:9    /// explicit field list, so no protocol output changes; the rates are read
crates/pistol-search/src/info.rs:10   /// through a committed harness in the `pistol-search` test tree that calls
crates/pistol-search/src/info.rs:11   /// `Searcher::search` directly
```

That is the SAME shape the census is in, said by the code about a sibling counter.

**(c) `pistol-cli` does not depend on `pistol-search` at all, on purpose.**

```
crates/pistol-cli/src/lib.rs:29   //! This crate depends on pistol-core (for the stone and turn tokens, and for the
crates/pistol-cli/src/lib.rs:30   //! rules a driver validates its own game against) and on pistol-engine (for the
crates/pistol-cli/src/lib.rs:31   //! seam). It deliberately depends on neither pistol-search nor pistol-eval: what
crates/pistol-cli/src/lib.rs:32   //! this crate says to an engine, it says through the trait, and the engine
crates/pistol-cli/src/lib.rs:33   //! re-exports the reporting types it hands out.
```

confirmed by the manifest, which lists exactly four dependencies and none of them is
`pistol-search`: `crates/pistol-cli/Cargo.toml:18-31`.

**(d) MEASURED, not asserted, and proved by ENUMERATION rather than by keyword.**
A keyword grep proves absence only over the word searched, so the engine config's key
set was enumerated exhaustively instead. `Config`
(`crates/pistol-engine/src/config.rs:82-99`) has exactly six members —
`schema_version`, `engine`, `search`, `eval`, `instrument`, `play`, `solver` — and the
complete set of leaf keys beneath them is: `on_search_path`, `per_call_node_cap`,
`trigger`, `epsilon_num`, `epsilon_den`, `zone_orders`, `free_stone_radius`,
`tt_entries`, `attacker_policy` (`:115-132`); `mode` (`:158`); `tt_bytes`,
`candidate_policy` (`:194-196`); `backend`, `weights_file` (`:293-297`); `threads`,
`tie_break` (`:324-326`); `movetime_epsilon_ms` (`:343`); plus the candidate-policy
variants' own `radius` (`:210`) and `quiet_radius`, `quiet_top_k`,
`safety_net_top_k`, `widen_schedule`, `tier_t_own_count`, `tier_t_opponent_count`,
`q_depth_turns`, `q_triggers`, `killers`, `history`, `countermove` (`:219-268`).
**There is no census key under any name**, and `#[serde(deny_unknown_fields)]` means
one cannot be smuggled in by a document. Separately, the keyword search is also clean:
every `census` hit in `configs/` is comment prose
(`configs/tactical_staged_v0.toml:14, 15, 34`, referring to `wp15b_census.rs`), and
every hit in `crates/pistol-cli` is the unrelated `corpus-census` symmetry tool.

**Therefore: the only caller shape that exists is the example, which links the crate
directly** — `crates/pistol-search/examples/trigger_census.rs:31-34`:

```
crates/pistol-search/examples/trigger_census.rs:31   use pistol_search::params::{SolverTrigger, SolverWiring};
crates/pistol-search/examples/trigger_census.rs:32   use pistol_search::{
crates/pistol-search/examples/trigger_census.rs:33       CandidatePolicy, OrderingHeuristics, QTriggers, SearchParams, Searcher, StagedParams, Stop,
crates/pistol-search/examples/trigger_census.rs:34   };
```

and builds its own `SearchParams` in code rather than loading a config —
`crates/pistol-search/examples/trigger_census.rs:119-150`. **It does not speak the
line protocol, does not load an engine config, and constructs its own weights path**:

```
crates/pistol-search/examples/trigger_census.rs:120       let weights_path =
crates/pistol-search/examples/trigger_census.rs:121           std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../configs/eval_v0_weights.toml");
```

**P2 VERDICT ON THE SEAM: FAILS.** Requirement 3 (census on every game) and the
black-box constraint in P1 are in direct conflict at HEAD. A black-box driver over
the line protocol cannot switch the census on, cannot read a row, and cannot even
name it in a config. The design must resolve this, not route around it.

## P2.2 How the census is invoked today, and what it prints per firing

Invocation, from its own usage block:

```
crates/pistol-search/examples/trigger_census.rs:13   //! Usage:
crates/pistol-search/examples/trigger_census.rs:14   //!   trigger_census --fixture <path> --nodes <n> --cap <n> [--quiet-radius <n>]
crates/pistol-search/examples/trigger_census.rs:15   //!                  [--gate on|off]
```

with every argument required-or-refused, `crates/pistol-search/examples/trigger_census.rs:110-116`:

```
crates/pistol-search/examples/trigger_census.rs:112       nodes: nodes.ok_or("--nodes is required: the budget is never guessed")?,
crates/pistol-search/examples/trigger_census.rs:113       cap: cap.ok_or("--cap is required: a call count without its cap is not a quantity")?,
```

Its fixture reader is a THIRD implementation of the `position` grammar, written
because the crate cannot reach `PositionSpec` —
`crates/pistol-search/examples/trigger_census.rs:50-53`:

```
crates/pistol-search/examples/trigger_census.rs:50   /// The `position` verb's own grammar, read here rather than through
crates/pistol-search/examples/trigger_census.rs:51   /// `PositionSpec` because this crate does not depend on the one that owns it —
crates/pistol-search/examples/trigger_census.rs:52   /// a turn is one cell or two separated by `/`, and the rules judge each stone
crates/pistol-search/examples/trigger_census.rs:53   /// as it goes down (CLAUDE.md rule 2).
```

The per-firing `row` line — the exact field list, including the two `cover` columns:

```
crates/pistol-search/examples/trigger_census.rs:215       for row in engine.take_trigger_census() {
crates/pistol-search/examples/trigger_census.rs:218               "trigger_census: row entry {entries} turns {} mover_hot {} opp_hot {} \
crates/pistol-search/examples/trigger_census.rs:219                mover_w1 {} opp_w1 {} mover_l3 {} opp_l3 {} cover {} covers {} \
crates/pistol-search/examples/trigger_census.rs:220                att_visits {} att_proved {} \
crates/pistol-search/examples/trigger_census.rs:221                def_asked {} def_visits {} def_proved {}",
crates/pistol-search/examples/trigger_census.rs:229               columns.cover.token(),
crates/pistol-search/examples/trigger_census.rs:230               columns.cover.count(),
```

The two `cover` columns are a CLASS and a COUNT, and the class token set is closed at
`crates/pistol-search/src/census.rs:81-87`:

```
crates/pistol-search/src/census.rs:83               CoverClass::NothingToBlock => "none",
crates/pistol-search/src/census.rs:84               CoverClass::Impossible => "impossible",
crates/pistol-search/src/census.rs:85               CoverClass::Minimal(_) => "minimal",
```

with the count zero for the two classes that hold none,
`crates/pistol-search/src/census.rs:91-96`. The reason the class and not the cells is
recorded is at `crates/pistol-search/src/census.rs:63-65`: *"a census row wants the
class and the count, because a detector reading this column decides on whether a
cover exists and not on where it is"*.

There is also a per-ENTRY line, which is the one the OFF seat prints alone:

```
crates/pistol-search/examples/trigger_census.rs:202               "trigger_census: entry {entries} search_nodes {} solver_nodes {} firings {} \
crates/pistol-search/examples/trigger_census.rs:203                invocations {} proofs {} root_nodes {}",
```

Both shapes are visible in a committed artifact, e.g.
`artifacts/stage3c_census_corpus_r0_OFF_v1.txt` line 3:
`trigger_census: entry 0 search_nodes 50176 solver_nodes 0 firings 0 invocations 0 proofs 0 root_nodes 0`.

## P2.3 The cold-table discipline: `engine.clear()` between fixture entries, and why

The call, with the reason in the code:

```
crates/pistol-search/examples/trigger_census.rs:188           // EVERY ENTRY IS A DIFFERENT GAME, so every entry starts cold. Without
crates/pistol-search/examples/trigger_census.rs:189           // this the transposition table carries across positions and a later
crates/pistol-search/examples/trigger_census.rs:190           // entry's node count depends on an earlier entry's — the hazard
crates/pistol-search/examples/trigger_census.rs:191           // `crate::tt` states in its own words (D-7) — and the counts stop being
crates/pistol-search/examples/trigger_census.rs:192           // comparable with `tools/bench_block.sh`, which runs one `newgame` per
crates/pistol-search/examples/trigger_census.rs:193           // entry in a fresh process and is the seat the bracket's SHARE was
crates/pistol-search/examples/trigger_census.rs:194           // derived on.
crates/pistol-search/examples/trigger_census.rs:195           engine.clear();
```

`crate::tt`'s own words, which that comment cites —
`crates/pistol-search/src/tt/mod.rs:105-108`:

```
crates/pistol-search/src/tt/mod.rs:105       /// Forget everything. This is what a new game does: a table carried across
crates/pistol-search/src/tt/mod.rs:106       /// games would let one search's node count depend on another's
crates/pistol-search/src/tt/mod.rs:107       /// (docs/decisions.md D-7).
crates/pistol-search/src/tt/mod.rs:108       pub fn clear(&mut self) {
```

D-527 is the ADR that put the call there, `docs/decisions.md:1122`:

> D-527: **THE TRIGGER CENSUS WAS TAKEN ON A WARM TRANSPOSITION TABLE, THE DEFECT IS
> FIXED, AND THE CORRECTED SEAT REPRODUCES TWO COLD-SEAT ANCHORS THE CONTAMINATED ONE
> MISSED.** … it *"constructed ONE `Searcher` and searched every fixture entry through
> it, never calling `Searcher::clear()`, while `tools/bench_block.sh` runs one
> `newgame` in a fresh process per entry per rep. **A census fixture is 24 distinct
> games (or 20, or 2), and `crate::tt`'s own words name the hazard**"* … *"THE
> PREDICTED CONFIRMING VALUES CAME BACK EXACTLY: the red team read three trigger-rich
> OFF entries as 239 / 186 / 180 against the bench's **7,795 / 7,742 / 6,886** and
> said a `clear()` between entries would produce the bench figures; it does, to the
> digit."*

**This is a live constraint on WP-2.0, not history.** A self-play pipeline plays MANY
games in sequence. If it holds one engine process across games, D-7's hazard is back
— the same defect D-527 cost a whole matrix revision to find. The two disciplines
that already exist are `tools/bench_block.sh:28-31`'s one-process-per-entry and this
example's `clear()`; the protocol's `newgame` (§P1.5) is the black-box equivalent.

## P2.4 The census row does NOT carry position identity — a D-537 GAP

D-537's condition, at `docs/decisions.md:1142`:

> *"(1) The minimum is counted in **WIN-PROVING FIRINGS ON DISJOINT POSITIONS**, not
> in firings and not in games — which is exactly the denominator D-532 had to correct
> twice, once for positions and once for direction."*

The row that would have to carry that identity is
`crates/pistol-search/src/census.rs:13-22`:

```
crates/pistol-search/src/census.rs:13   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
crates/pistol-search/src/census.rs:14   pub struct TriggerObservation {
crates/pistol-search/src/census.rs:16       pub columns: TriggerColumns,
crates/pistol-search/src/census.rs:18       pub attacker: TriggerAnswer,
crates/pistol-search/src/census.rs:21       pub defender: Option<TriggerAnswer>,
crates/pistol-search/src/census.rs:22   }
```

and its columns at `crates/pistol-search/src/census.rs:42-59`:

```
crates/pistol-search/src/census.rs:44       pub turns_from_root: u32,
crates/pistol-search/src/census.rs:46       pub mover_hot: u32,
crates/pistol-search/src/census.rs:48       pub opponent_hot: u32,
crates/pistol-search/src/census.rs:50       pub mover_win_in_one_ply: u32,
crates/pistol-search/src/census.rs:52       pub opponent_win_in_one_ply: u32,
crates/pistol-search/src/census.rs:54       pub mover_live_three: u32,
crates/pistol-search/src/census.rs:56       pub opponent_live_three: u32,
crates/pistol-search/src/census.rs:58       pub cover: CoverClass,
```

**Nine numbers and a class token. Not one of them names the position.** The only
locator is `turns_from_root: u32` — a DEPTH, not an identity; the row's own doc calls
it *"Turns from the search root. 0 is the root's own firing"*
(`crates/pistol-search/src/census.rs:43-44`). The printed line adds `entry {entries}`
(`crates/pistol-search/examples/trigger_census.rs:218`), which identifies the FIXTURE
LINE the search started from — not the in-tree position the firing happened at. The
struct derives `Copy` (`census.rs:13`), so today it cannot hold a `Vec` or a `String`
without a shape change.

**The identity exists and is cheap** — `pistol-core` already computes it, and it is
in scope at the firing site. `crates/pistol-core/src/state.rs:122-136`:

```
crates/pistol-core/src/state.rs:122       /// The position's zobrist key: its stones, whose move it is, and how far
crates/pistol-core/src/state.rs:123       /// into the turn they are (docs/decisions.md D-8).
crates/pistol-core/src/state.rs:129       /// The turn *number* is deliberately absent, and nothing is lost: for an
crates/pistol-core/src/state.rs:130       /// ongoing game the stone count fixes the turn, the phase and the mover
crates/pistol-core/src/state.rs:131       /// together, so two positions this key cannot tell apart are the same
crates/pistol-core/src/state.rs:132       /// position.
crates/pistol-core/src/state.rs:134       pub fn key(&self) -> Key128 {
```

and the search already reads it one method above the firing —
`crates/pistol-search/src/pvs.rs:249`: `let key = self.position.state().key();`,
with the solver gate at `crates/pistol-search/src/pvs.rs:275-282` inside the same
function. The push site that would carry it is
`crates/pistol-search/src/pvs.rs:737-751` (`Run::observe`) and its root counterpart
`crates/pistol-search/src/search.rs:757-772` (`push_root_census`).

**P2 VERDICT ON IDENTITY: HOLDS WITH A GAP — and the gap is load-bearing.** Without a
position key on the row, "win-proving firings on DISJOINT POSITIONS" cannot be
counted from census output at all; the best available denominator is the fixture
entry, which is the very substitution D-537 says D-532 had to correct twice.

## P2.5 What is already true and does not need re-deciding

- The census cannot move a move. Stated three times in the code:
  `crates/pistol-search/src/census.rs:11-12` (*"**It is an observation and never an
  input.** Nothing in the search reads one back, so recording them cannot move a move
  (CLAUDE.md rule 4)"*), `crates/pistol-search/src/search.rs:201-203`, and
  `crates/pistol-search/src/pvs.rs:73-76`.
- The census must live on the SEARCHER, not the run, because the root fires before a
  run exists — `crates/pistol-search/src/search.rs:76-79`: *"a census missing the one
  firing that can cost two whole caps would rank an option field on the wrong rows
  (docs/decisions.md D-516)"*. The root's own row is pushed separately at
  `crates/pistol-search/src/search.rs:757-772`, with `turns_from_root: 0` set at
  `crates/pistol-search/src/search.rs:745-746`.
- A firing's row exists whether or not it proved —
  `crates/pistol-search/src/pvs.rs:735-736`: *"Called on every exit of
  [`Run::solver_verdict`] that fired, so a firing's row exists whether or not the
  firing proved."* Both directions are on the row (`attacker`, `defender`), which is
  what D-535 (`docs/decisions.md:1138`) requires: *"the census gate ranks BOTH
  directions per D-512 as registered"*.
- **The gate is off in every committed config**, so a census run needs a seat of its
  own: `configs/instrument_staged_snk_v0.toml:74-78`, `on_search_path = false`, whose
  comment says *"gate OFF in every committed config until an SPRT says otherwise"*.

---

# P3 — `book_v2` AND ITS LEDGER: **HOLDS WITH A GAP**

The book exists, is committed, is sha-pinned, has 4500 entries and a grammar with
three independent consumers. The ledger exists and is EMPTY. The gap: **no committed
config points at it, and the only range mechanism is a contiguous window with no
seed.**

## P3.1 The fixture: path, header, count, grammar

**Path:** `crates/pistol-cli/tests/fixtures/random_openings_v2.txt` (tracked; blob
`892f8f1e16dd405173078039fabd62b09430244d`).

**Count: 4500 entries**, measured rather than asserted:

```
$ /usr/bin/grep -v -e '^#' -e '^[[:space:]]*$' \
    crates/pistol-cli/tests/fixtures/random_openings_v2.txt | /usr/bin/wc -l
4500
```

(4576 lines total, 76 header lines, 0 blank; header is lines 1-76, body 77-4576.) The
count is also pinned in-tree at
`crates/pistol-cli/tests/random_openings_document_tests.rs:183-195`.

**Header — the generation parameters, which are the whole provenance:**

```
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:45   # param k_stones 5
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:46   # param n_openings 4500
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:47   # param max_radius 5
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:48   # param seed 20260830
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:49   # param rng splitmix64
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:50   # param draw uniform_over_the_unused_cells_of_the_ball
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:51   # param dedupe canonical_form_over_the_12_lattice_symmetries
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:52   # param on_collision discard_the_candidate_and_draw_the_next
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:53   # param emission_order generation_order
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:54   # derived turn_structure p1@origin,p2,p2,p1,p1
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:55   # derived ball_cells 91
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:56   # derived candidates_drawn 4505
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:57   # derived symmetry_collisions 5
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:58   # derived openings 4500
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:76   # body_sha256 62bca58ebb9bc8bfb94d6682105b3f87608247c8b984e6ad0e89a9a7012fa48d
```

Two header claims WP-2.0 must not violate:

```
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:9    # One position per line, in the canonical move-list encoding (docs/decisions.md
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:10   # D-6) — the exact tail the `position` verb takes. There is NO commentary column
```

```
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:71   # Emission order is GENERATION order, and that is deliberate. The draws are
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:72   # independent, so a prefix of this file is already a sample; sorting it by
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:73   # anything — coordinates, digest — would make a prefix a slice of whatever was
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:74   # sorted on instead
```

**Line grammar, quoted where it is CONSUMED (D-477).** The reader is in the ARENA,
not in the generator, and it delegates in three layers:

Layer 1 — line handling, `crates/pistol-arena/src/openings.rs:155-198`:

```
crates/pistol-arena/src/openings.rs:171       // Everything from " #" onward is commentary (docs/decisions.md D-143).
crates/pistol-arena/src/openings.rs:172       let tail = match raw.find(" #") {
crates/pistol-arena/src/openings.rs:177       let spec = PositionSpec::from_str(tail).map_err(|error| {
crates/pistol-arena/src/openings.rs:180       let PositionSpec::Start { moves } = &spec else {
crates/pistol-arena/src/openings.rs:184           "an opening is a move list (`start moves ...`), which is the canonical encoding of a \
crates/pistol-arena/src/openings.rs:190       spec.replay()
```

Layer 2 — the field split, `crates/pistol-engine/src/position_token.rs:66-68` and
`:84-98` (already quoted in §P1.2 — **the SAME parser the `position` verb uses**).
Layer 3 — the turn token, `crates/pistol-core/src/turn.rs:229-242`.

Representative entries:

```
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:77     start moves 0,0 -4,-1/1,1 -2,-2/-2,-1
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:78     start moves 0,0 0,4/4,0 3,-4/4,-1
crates/pistol-cli/tests/fixtures/random_openings_v2.txt:4576   start moves 0,0 -2,1/4,-1 -3,-1/2,3
```

**Grammar as consumed: five whitespace-separated words —
`start moves <single> <pair> <pair>`**, matching `# derived turn_structure
p1@origin,p2,p2,p1,p1` at fixture line 54. **This is byte-for-byte the tail the
`position` verb takes** (§P1.2), so a black-box driver feeds an entry to the engine
by prefixing `position `. `tools/bench_block.sh:189-230`'s `--grammar tail` is a third
independent consumer of the same shape.

**Sha pinning**, at `crates/pistol-cli/tests/random_openings_document_tests.rs:16-18`:

```
crates/pistol-cli/tests/random_openings_document_tests.rs:16   /// The SHA-256 of the committed `fixtures/random_openings_v2.txt`.
crates/pistol-cli/tests/random_openings_document_tests.rs:17   const RANDOM_OPENINGS_V2_SHA256: &str =
crates/pistol-cli/tests/random_openings_document_tests.rs:18       "829361a9ae61d0d4369b5291bfc893133fa8160867f11cc638b11f432b6cc29a";
```

asserted at `:162-169`, and regenerated byte-for-byte by
`random_openings_v2_is_what_this_build_produces` at `:151-160`. The arena additionally
verifies the IN-BAND body digest before it will read the book —
`crates/pistol-arena/src/openings.rs:60-68`.

## P3.2 `docs/book_v2_ledger.md` — its content and its row shape

53 lines. The row shape, at `docs/book_v2_ledger.md:39`:

```
docs/book_v2_ledger.md:39   | `openings_skip` | `openings_take` | range | consumed by | pre-registration |
```

**Rows filled in: ZERO**, stated by the document itself:

```
docs/book_v2_ledger.md:42   *(empty: no range of `book_v2` has been consumed)*
```

The rule that governs a WP-2.0 draw, at `docs/book_v2_ledger.md:16-19`:

```
docs/book_v2_ledger.md:16   **The rule.** A new pre-registration takes the next unconsumed range, adds its
docs/book_v2_ledger.md:17   row here in the same commit that adds its arena config, and never re-reads a
docs/book_v2_ledger.md:18   range this table already holds. Reading a consumed range for a CLOSED verdict
docs/book_v2_ledger.md:19   is not a new use and needs no row.
```

and when the row lands, at `docs/book_v2_ledger.md:4-8`:

```
docs/book_v2_ledger.md:4    of `crates/pistol-cli/tests/fixtures/random_openings_v2.txt`, recording the
docs/book_v2_ledger.md:5    range it consumed. A slice appears here **when its pre-registration is
docs/book_v2_ledger.md:6    committed**, not when its run finishes — a range reserved by a document that
docs/book_v2_ledger.md:7    was never run is still spent, because a later run over it would be a second
docs/book_v2_ledger.md:9    reading of a sample someone else chose.
```

**WP-2.0 is not alone at the table.** Two standing claimants already have their hands
up, `docs/book_v2_ledger.md:46-49`:

```
docs/book_v2_ledger.md:48   | The Stage-3 detector's SPRT | SCHEDULED | one slice of the standing shape |
docs/book_v2_ledger.md:49   | The WP-1.5d ±21.5 resolution run | **LICENSED, NOT SCHEDULED** (D-505, D-492) | a slice large enough to resolve an interval that spanned zero at 500 pairs; …
```

and the book's 4500 was sized for exactly those two and nobody else — D-518,
`docs/decisions.md:1104`: *"`n_openings = ceil_to_500(P + 500)` … **MEASURED: `P =
4000` … so 4500.** The `+ 500` is the Stage-3 detector's own standing slice, which
must not compete with the licensed-not-scheduled WP-1.5d resolution run for the same
openings."*

**So WP-2.0's self-play slice was not in the sizing.** That is a fact, not a
prohibition — but it is a decision the design must take rather than assume.

## P3.3 D-518's freshness/range rules, and what "consuming a range" means operationally

D-518 lives at `docs/decisions.md:1104` and is cited by exactly one other line
(D-537, `docs/decisions.md:1142`, and only for its power-rule discipline —
`git grep -n "D-518"` returns two hits total). Its operational clauses:

> **FRESHNESS IS A CLAIM ABOUT RANGES AND AT THAT LEVEL IT IS ABSOLUTE**: different
> seeds, so no range of v2 is a range of v1 and no governed run over it can re-read a
> consumed sample. At the level of an individual POSITION it is not, and cannot be —
> both books draw independently from one pool of `C(90,2) x C(88,2) = 15,331,140`
> assignments, about 1.28 million once the twelve lattice symmetries are folded — so
> the MEASURED overlap of **1 identical line and 10 positions up to symmetry** against
> expectations of 0.59 and 7.04 is what chance gives, is 0.22% of v2, and is **pinned
> exactly**… **Loadability is receipted over every entry** by the guarded bench block
> (D-475), and the consumed-ranges ledger `docs/book_v2_ledger.md` starts EMPTY. Flips
> when a governed pre-registration draws its first slice, which adds a ledger row
> rather than amending this line.

The ledger says the same thing once, in the section that owns it (D-423's rule),
`docs/book_v2_ledger.md:21-31`, ending: *"**So this ledger's only job is to keep v2's
own ranges disjoint from each other.**"*

**"Consuming a range", operationally, is three things and all three are code today:**
`openings_skip`, `openings_take`, and the refusal when they do not fit. Consumed at
`crates/pistol-arena/src/openings.rs:97-114`:

```
crates/pistol-arena/src/openings.rs:98        if skip.saturating_add(take) > total {
crates/pistol-arena/src/openings.rs:102               "{} holds {total} openings and the run asks for {take} after skipping {skip} \
crates/pistol-arena/src/openings.rs:103                (run.openings_skip); taking fewer silently would make the run a different \
crates/pistol-arena/src/openings.rs:104                experiment from the one written down",
crates/pistol-arena/src/openings.rs:109       let mut taken: Vec<Opening> = parsed.drain(skip..skip + take).collect();
```

driven from the one call site, `crates/pistol-arena/src/bin/arena.rs:220-225`.

## P3.4 How a governed config references the book — and the finding

The schema, `crates/pistol-arena/src/config.rs:48-65`:

```
crates/pistol-arena/src/config.rs:48   pub struct RunSection {
crates/pistol-arena/src/config.rs:50       pub openings_file: PathBuf,
crates/pistol-arena/src/config.rs:59       pub openings_take: usize,
crates/pistol-arena/src/config.rs:60       /// How many openings to SKIP before taking, so two runs can draw DISJOINT
crates/pistol-arena/src/config.rs:61       /// samples from one book: skip 0/take t and skip t/take t share nothing
crates/pistol-arena/src/config.rs:62       /// (docs/decisions.md D-202). `skip + take` must fit inside the file, or
crates/pistol-arena/src/config.rs:63       /// the run is a different experiment from the one written down. Skip
crates/pistol-arena/src/config.rs:64       /// changes which games are played, so it is part of `experiment_sha256`.
crates/pistol-arena/src/config.rs:65       pub openings_skip: usize,
```

under `#[serde(deny_unknown_fields)]` (`crates/pistol-arena/src/config.rs:47`), with
`ARENA_SCHEMA_VERSION = 2` because of that very key
(`crates/pistol-arena/src/config.rs:7-10`). **A book reference is a PATH STRING, not
an enum** — there is no `BookVersion` on the arena side; the closed set lives only in
the generator (`crates/pistol-cli/src/random_openings/mod.rs:30-48`).

A whole governed config for the shape, `configs/arena_wp15d_cap_vs_staged.toml:7-17`:

```
configs/arena_wp15d_cap_vs_staged.toml:7    [run]
configs/arena_wp15d_cap_vs_staged.toml:8    openings_file = "crates/pistol-cli/tests/fixtures/random_openings_v1.txt"
configs/arena_wp15d_cap_vs_staged.toml:9    openings_take = 500
configs/arena_wp15d_cap_vs_staged.toml:10   openings_skip = 1500
configs/arena_wp15d_cap_vs_staged.toml:11   turn_cap = 40
configs/arena_wp15d_cap_vs_staged.toml:12   n_workers = 4
configs/arena_wp15d_cap_vs_staged.toml:13   hang_timeout_ms = 120000
configs/arena_wp15d_cap_vs_staged.toml:15   [budget]
configs/arena_wp15d_cap_vs_staged.toml:16   kind = "nodes"
configs/arena_wp15d_cap_vs_staged.toml:17   value = 50000
```

**MEASURED FINDING — no committed config uses `book_v2`.** Every one of the thirteen
`openings_file` values names v1 or `openings_v1.txt`
(`/usr/bin/grep -rn '^openings_file' configs/ | LC_ALL=C sort`), and

```
$ /usr/bin/grep -rn 'random_openings_v2' configs/ | LC_ALL=C sort
configs/random_openings_v1.toml:28:# so this key stays "v1" forever. A v2 book is configs/random_openings_v2.toml,
configs/random_openings_v2.toml:10:#     --config configs/random_openings_v2.toml \
configs/random_openings_v2.toml:1:# pistol — the parameters `random_openings_v2.txt` was generated with.
```

— three hits, all inside the GENERATOR document. Consistent with
`docs/experiments/book_v2_registration.md:9-10`: *"No committed config flips to v2 in
this arc"*.

## P3.5 What a subrange can be today

**A contiguous window `[skip, skip+take)` and nothing else.** Stated twice in the
code: `crates/pistol-arena/src/openings.rs:47` (*"A contiguous window"*) and
`crates/pistol-arena/src/config.rs:53` (*"A contiguous window rather than a selection"*).
No stride, no index list, no predicate. Two properties the pipeline inherits:

- The WHOLE file is parsed, digest-verified and symmetry-deduped BEFORE the window is
  cut — `crates/pistol-arena/src/openings.rs:52-53`: *"so a defect outside the window
  still refuses the file"*. Cost scales with 4500, not with the slice.
- Opening indices in the report are WINDOW-relative; the absolute book position is
  `openings_skip + index` — `crates/pistol-arena/src/openings.rs:110-114` and D-202
  (`docs/decisions.md:450`), which calls this *"A named trap"*.

**P3 VERDICT: HOLDS WITH A GAP.** The book, its pin, its grammar and its range
mechanism are all real and all reusable. The gaps: (i) no config references v2, so
WP-2.0 writes the first one; (ii) the ledger is empty and WP-2.0's slice would be its
first row, landing in the same commit as its config; (iii) the 4500 was sized for two
other claimants and WP-2.0's demand was not in that arithmetic.

---

# P4 — DEEP-SEARCH SCORES AND THEIR COST SHAPE: **HOLDS WITH A GAP**

## P4.1 Where a score is produced, and in what units

The encoding, `crates/pistol-search/src/score.rs:5-19`:

```
crates/pistol-search/src/score.rs:3    /// A completed line, scored at zero distance. No position ever holds this
crates/pistol-search/src/score.rs:4    /// score: a win is always at least one turn away from the node that reports it.
crates/pistol-search/src/score.rs:5    pub const MATE: i32 = 30_000;
crates/pistol-search/src/score.rs:9    pub const INFINITY: i32 = MATE + 1;
crates/pistol-search/src/score.rs:16   pub const MAX_MATE_TURNS: u32 = 1_000;
crates/pistol-search/src/score.rs:19   pub const MATE_THRESHOLD: i32 = MATE - MAX_MATE_TURNS as i32;
```

so a mate is `30000 - turns` (`crates/pistol-search/src/score.rs:43`:
`MATE - turns as i32`), and the two bands are kept apart by a COMPILE-TIME assert —
`crates/pistol-search/src/score.rs:24-29`:

```
crates/pistol-search/src/score.rs:24   // The two bands may not meet: a saturated static evaluation must still read as
crates/pistol-search/src/score.rs:25   // a value, or the search would announce a mate it never found.
crates/pistol-search/src/score.rs:26   const _: () = assert!(
crates/pistol-search/src/score.rs:27       EVAL_MAX < MATE_THRESHOLD,
```

The reading, consumed by the protocol, `crates/pistol-search/src/score.rs:53-72`:

```
crates/pistol-search/src/score.rs:54   pub enum ScoreKind {
crates/pistol-search/src/score.rs:55       /// A static evaluation, positive for the side to move.
crates/pistol-search/src/score.rs:56       Eval(i32),
crates/pistol-search/src/score.rs:57       /// The side to move completes a line this many turns from now.
crates/pistol-search/src/score.rs:58       MateIn(u16),
crates/pistol-search/src/score.rs:59       /// The opponent does.
crates/pistol-search/src/score.rs:60       MatedIn(u16),
crates/pistol-search/src/score.rs:64   pub fn classify(score: i32) -> ScoreKind {
crates/pistol-search/src/score.rs:65       if score >= MATE_THRESHOLD {
crates/pistol-search/src/score.rs:66           ScoreKind::MateIn(distance(MATE - score))
crates/pistol-search/src/score.rs:67       } else if score <= -MATE_THRESHOLD {
crates/pistol-search/src/score.rs:68           ScoreKind::MatedIn(distance(MATE + score))
```

**Point of view: the side to move at the ROOT.** Stated where the field is declared,
`crates/pistol-search/src/info.rs:153-155`:

```
crates/pistol-search/src/info.rs:153   /// The score of the position from the point of view of the side to move at
crates/pistol-search/src/info.rs:154   /// the root; read it with [`crate::score::classify`].
crates/pistol-search/src/info.rs:155   pub score: i32,
```

**Units, and the mate convention — both are protocol contract**,
`crates/pistol-cli/src/report.rs:145-158`:

```
crates/pistol-cli/src/report.rs:147   /// `mate <turns>` counts every turn from the root, both sides', so a win for the
crates/pistol-cli/src/report.rs:148   /// side to move is always an odd distance and a loss an even one; `-mate
crates/pistol-cli/src/report.rs:149   /// <turns>` is that loss (docs/decisions.md D-3, D-72). `cp <n>` is the static
crates/pistol-cli/src/report.rs:150   /// evaluation in the integer units pistol-eval works in — there is no pawn on
crates/pistol-cli/src/report.rs:151   /// this board to be a hundredth of, and inventing a conversion factor would
crates/pistol-cli/src/report.rs:152   /// make the number less honest, not more familiar.
crates/pistol-cli/src/report.rs:155           ScoreKind::Eval(value) => format!("cp {value}"),
crates/pistol-cli/src/report.rs:156           ScoreKind::MateIn(turns) => format!("mate {turns}"),
crates/pistol-cli/src/report.rs:157           ScoreKind::MatedIn(turns) => format!("-mate {turns}"),
```

The binary's own usage repeats the warning for a driver author,
`crates/pistol-cli/src/bin/pistol.rs:40-43`:

```
crates/pistol-cli/src/bin/pistol.rs:40     `score mate T` counts EVERY turn from the root, both sides', not the winner's
crates/pistol-cli/src/bin/pistol.rs:41     own turns: an odd T is a win for the side to move at the root, an even T a loss,
crates/pistol-cli/src/bin/pistol.rs:43     A driver that assumes the other convention halves every distance it reads.
```

**`cp` is NOT centipawns.** A label schema that calls the field `centipawns` would be
asserting a conversion this project refuses to invent.

## P4.2 What `SearchInfo` carries

`crates/pistol-search/src/info.rs:131-179`. The label-relevant fields:

```
crates/pistol-search/src/info.rs:138   pub depth_turns: u32,
crates/pistol-search/src/info.rs:143   pub seldepth_turns: u32,
crates/pistol-search/src/info.rs:145   pub nodes: u64,
crates/pistol-search/src/info.rs:147   pub nps: u64,
crates/pistol-search/src/info.rs:149   pub time_ms: u64,
crates/pistol-search/src/info.rs:152   pub pv: Vec<Turn>,
crates/pistol-search/src/info.rs:155   pub score: i32,
crates/pistol-search/src/info.rs:157   pub hashfull_permille: u32,
crates/pistol-search/src/info.rs:161   pub stages: StageCounters,
crates/pistol-search/src/info.rs:166   pub search_nodes: u64,
crates/pistol-search/src/info.rs:172   pub solver_nodes: u64,
crates/pistol-search/src/info.rs:175   pub solver_refusals: u32,
crates/pistol-search/src/info.rs:178   pub solver_calls: SolverCallCounters,
```

`depth_turns` is guaranteed to be a COMPLETED depth,
`crates/pistol-search/src/info.rs:133-137`: *"always a depth that was actually
COMPLETED … partial-iteration work is never attributed to a completed depth"*.

**THE GAP: `Provenance` is on `SearchOutcome` but NOT on the wire.**

```
crates/pistol-search/src/info.rs:237   pub struct SearchOutcome {
crates/pistol-search/src/info.rs:239       pub best: Turn,
crates/pistol-search/src/info.rs:242       pub info: SearchInfo,
crates/pistol-search/src/info.rs:244       pub provenance: Provenance,
crates/pistol-search/src/info.rs:250   pub enum Provenance {
crates/pistol-search/src/info.rs:254       CompletedDepth,
crates/pistol-search/src/info.rs:258       PartialRoot,
crates/pistol-search/src/info.rs:262       SolverProof,
crates/pistol-search/src/info.rs:266       Fallback,
```

`crates/pistol-cli/src/report.rs:82-97` renders no provenance field. Its doc says why
it matters, `crates/pistol-search/src/info.rs:230-235`: *"`provenance` says which,
because a score whose kind cannot be read from the data is the silent widening
CLAUDE.md rule 10 forbids."*

**Under a NODE budget this is a narrow gap, and the design should know exactly how
narrow.** `PartialRoot` and `Fallback` are wall-clock-only —
`crates/pistol-search/src/info.rs:251-254`: `CompletedDepth` is *"The only provenance a
reproducible stop can produce, and therefore the only one a strength claim ever
quotes"*. But `SolverProof` is NOT wall-clock-only: it fires before deepening
(`crates/pistol-search/src/info.rs:259-262`), and under it `depth_turns` is the
PROOF's depth and `nodes` is the solver's nodes alone —
`crates/pistol-search/src/search.rs:790-800`:

```
crates/pistol-search/src/search.rs:793               depth_turns: depth,
crates/pistol-search/src/search.rs:794               seldepth_turns: depth,
crates/pistol-search/src/search.rs:795               nodes: solver_nodes,
crates/pistol-search/src/search.rs:796               search_nodes: 0,
```

So under a node budget a label is always exact, but a `SolverProof` label's `depth`
and `nodes` mean something different from a `CompletedDepth` label's — **and the wire
cannot tell them apart** except by the indirect tell that `search_nodes` is 0. Note
this is only reachable with the solver gate ON, which no committed config sets
(§P2.5).

## P4.3 The committed instrument configs and their budgets

**A budget is NOT in an engine config.** No `configs/instrument_*.toml` has a budget
key — `git grep -n "budget" -- configs/` returns `[budget]` sections only in
`arena_*.toml` files. The budget reaches the engine on the `go` line (§P1.4) or
through `[budget]` in an arena document
(`crates/pistol-arena/src/config.rs:91-112`).

**"The standing 50 000 seat" is therefore a BUDGET, not a config**, and it is spelled
`nodes 50000` in three independent places:

```
configs/arena_wp15d_cap_vs_staged.toml:15   [budget]
configs/arena_wp15d_cap_vs_staged.toml:16   kind = "nodes"
configs/arena_wp15d_cap_vs_staged.toml:17   value = 50000
```

```
artifacts/wp19b_bench_landing_v2.txt   bench_delta: config configs/instrument_v0.toml, nodes 50000, depth_turns 2, reps 5
```

```
artifacts/stage3c_census_corpus_r0_OFF_v1.txt:1   trigger_census: argv --fixture crates/pistol-cli/tests/fixtures/bench_positions_v1.txt --nodes 50000 --cap 2048 --gate off
```

The committed instrument configs and what distinguishes them (all
`mode = "instrument"`, all `threads = 1`, all `tie_break = "lexicographic"`, all
`on_search_path = false`):

| config | candidate policy | `tt_bytes` | `safety_net_top_k` |
|---|---|---|---|
| `configs/instrument_v0.toml` | staged, `quiet_radius 2`, `quiet_top_k 16` | 268435456 | 0 |
| `configs/instrument_staged_v0.toml` | staged | 268435456 | 0 |
| `configs/instrument_staged_snk_v0.toml:27,35,37,47` | staged, `quiet_radius 2`, `quiet_top_k 16` | 268435456 | **16** |
| `configs/instrument_r2_v0.toml` | radius 2 | — | — |

`configs/instrument_staged_snk_v0.toml:65-67` for the instrument block:

```
configs/instrument_staged_snk_v0.toml:65   [instrument]
configs/instrument_staged_snk_v0.toml:66   threads = 1
configs/instrument_staged_snk_v0.toml:67   tie_break = "lexicographic"
```

and its solver block, which is the reason a census seat cannot be one of these
unchanged, `configs/instrument_staged_snk_v0.toml:74-78`:

```
configs/instrument_staged_snk_v0.toml:75   # WP-1.8b (design docs/experiments/wp18b_design.md §5): the solver on the
configs/instrument_staged_snk_v0.toml:76   # search path, gate OFF in every committed config until an SPRT says
configs/instrument_staged_snk_v0.toml:77   # otherwise. The knobs mirror configs/solver_v0.toml's committed values.
configs/instrument_staged_snk_v0.toml:78   on_search_path = false
```

## P4.4 COST SHAPE, as a SHAPE — what is already recorded, at what budget

**Reported, not extrapolated.** Games-per-hour is the pilot's job (D-500's class).

**(a) The committed instrument seat, `configs/instrument_v0.toml`, at `nodes 50000`,
24 positions x 5 reps** — `artifacts/wp19b_bench_landing_v2.txt`, whose candidate
binary sha is `e0eb1b196d0c384d57aa272f29815fa619025245b8a6a40a3e1de1d76f6ff453`:

```
band early: nps baseline median 366471.1 (IQR 445.6), candidate median 434737.9 (IQR 627.8)
band early: time-to-depth-2 baseline median 104.0 ms (IQR 1.0), candidate median 84.0 ms (IQR 0.0)
band late: nps baseline median 315867.8 (IQR 796.1), candidate median 383433.2 (IQR 2058.3)
band late: time-to-depth-2 baseline median 151.0 ms (IQR 1.0), candidate median 120.0 ms (IQR 1.0)
```

So the shipped seat: **~434,738 nps (early band) and ~383,433 nps (late band)**, at
**`nodes 50000`**, single-threaded.

**(b) The same binary sha, gate OFF, per-band medians over 5 reps** —
`artifacts/stage3_premise_nps_derivation_v1.txt:2, 4, 6` (config
`configs/bench_wp18c_solver_off.toml`, `budget nodes 50000 reps 5`, per
`artifacts/stage3_premise_nps_off_positions_v1.txt` lines 2-5):

```
artifacts/stage3_premise_nps_derivation_v1.txt:2   CORPUS band 15   OFF rows   60 nodes   3010560 search   3010560 solver         0 time_ms    6849  median nps    439819 IQR  0.51% of median
artifacts/stage3_premise_nps_derivation_v1.txt:4   CORPUS band 35   OFF rows   60 nodes   2509570 search   2509570 solver         0 time_ms    6672  median nps    375684 IQR  0.83% of median
artifacts/stage3_premise_nps_derivation_v1.txt:6   TRIGGER-RICH     OFF rows  100 nodes   3159070 search   3159070 solver         0 time_ms   10633  median nps    297184 IQR  0.49% of median
```

**(c) Per-position node and time totals at the same budget**, which is the shape a
per-position label costs — `artifacts/stage3_premise_nps_derivation_v1.txt:32-34`:

```
artifacts/stage3_premise_nps_derivation_v1.txt:32   CORPUS band 15   OFF search   50176.0   ON search   2846.9 + solver   49137.1 =   51984.0
artifacts/stage3_premise_nps_derivation_v1.txt:33   CORPUS band 35   OFF search   41826.2   ON search   3424.7 + solver   39867.1 =   43291.8
artifacts/stage3_premise_nps_derivation_v1.txt:34   TRIGGER-RICH     OFF search   31590.7   ON search   5159.7 + solver   27504.7 =   32664.3
```

Note the OFF seat spends **50,176** nodes for a 50,000 budget (the check interval's
overshoot), and reaches depth 2-3 — visible per record line in
`artifacts/stage3_premise_nps_off_positions_v1.txt` (e.g. `entry 0 … depth_turns 3
seldepth 4 nodes 50176 nps 450425 time 111`).

**(d) The gate-ON seat's cost, for scale, same budget, same binary** —
`artifacts/stage3_premise_nps_derivation_v1.txt:3, 5, 7`:

```
artifacts/stage3_premise_nps_derivation_v1.txt:3   CORPUS band 15   ON  rows   60 nodes   3119040 search    170815 solver   2948225 time_ms  161217  median nps     19319 IQR  0.49% of median
artifacts/stage3_premise_nps_derivation_v1.txt:5   CORPUS band 35   ON  rows   60 nodes   2597505 search    205480 solver   2392025 time_ms  290592  median nps      8953 IQR  0.68% of median
artifacts/stage3_premise_nps_derivation_v1.txt:7   TRIGGER-RICH     ON  rows  100 nodes   3266435 search    515970 solver   2750465 time_ms  498594  median nps      6557 IQR  0.36% of median
```

**That is a ~23x to ~45x slowdown for a census-bearing seat at the same node budget**
— read from the median-nps columns of the same file, at `cap 2048`. It is not an
estimate; it is what the two seats of one bench recorded.

**What no artifact records: the cost of a WHOLE GAME.** Every figure above is
per-POSITION at a fixed budget, from a bench fixture. Nothing on disk measures turns
per self-play game at `nodes 50000` under `book_v2` openings. The nearest existing
anchor is a turn cap, not a measurement — `configs/arena_wp15d_cap_vs_staged.toml:11`:
`turn_cap = 40`, whose own status is *"An evaluation horizon, never a game rule"*
(`crates/pistol-arena/src/config.rs:66-69`).

**P4 VERDICT: HOLDS WITH A GAP.** Score production, units, sign convention and point
of view are all pinned; `SearchInfo` carries everything a label needs; the committed
seat's nps is recorded at `nodes 50000` with sub-1% IQR. The gap is `Provenance`,
which is not on the wire, and which under a node budget matters only for the
`SolverProof` case that no committed config can currently reach.

---

# P5 — THE ARENA: **HOLDS FOR THE SEAM, FAILS FOR THE RECORD**

The single most important finding in this memo: **the arena ALREADY drives engines as
black-box subprocesses over the line protocol.** WP-2.0's hardest-sounding constraint
is met by existing, CI-gated, shipped code. What the arena does not do is emit a
per-position record, and it has no seed.

## P5.1 It drives players as CHILD PROCESSES over the protocol — requirement (c), met

```
crates/pistol-arena/src/channel.rs:55       /// Start `binary --config config` and begin reading its answers.
crates/pistol-arena/src/channel.rs:56       pub fn start(label: &str, binary: &Path, config: &Path) -> Result<Channel, ArenaError> {
crates/pistol-arena/src/channel.rs:57           let mut child = Command::new(binary)
crates/pistol-arena/src/channel.rs:58               .arg("--config")
crates/pistol-arena/src/channel.rs:59               .arg(config)
crates/pistol-arena/src/channel.rs:60               .stdin(Stdio::piped())
crates/pistol-arena/src/channel.rs:61               .stdout(Stdio::piped())
```

The two-line exchange, `crates/pistol-arena/src/exchange.rs:37-39` and `:58-63`:

```
crates/pistol-arena/src/exchange.rs:37       let position = position_line(moves);
crates/pistol-arena/src/exchange.rs:38       for line in [position.as_str(), rules.go_line] {
crates/pistol-arena/src/exchange.rs:59               if let Some(rest) =
crates/pistol-arena/src/exchange.rs:60                   line.strip_prefix(&format!("{} ", pistol_cli::report::BESTMOVE_PREFIX))
```

and the position line, `crates/pistol-arena/src/exchange.rs:153-155`:

```
crates/pistol-arena/src/exchange.rs:153   /// `position start moves …` — the whole game so far (docs/decisions.md D-6).
crates/pistol-arena/src/exchange.rs:154   pub fn position_line(moves: &[Turn]) -> String {
crates/pistol-arena/src/exchange.rs:155       let mut line = format!("{} start moves", pistol_cli::protocol::POSITION);
```

**The seam is not merely followed, it is PINNED BY A TEST in another crate** —
`crates/pistol-cli/tests/workspace_shape_tests.rs:118-139`:

```
crates/pistol-cli/tests/workspace_shape_tests.rs:119   fn pistol_arena_manifest_names_only_core_engine_and_cli() {
crates/pistol-cli/tests/workspace_shape_tests.rs:137       "the arena talks to engines through the protocol and to the rules through pistol-core; a search or eval dependency here would be reaching past the seam"
```

confirmed by the manifest, `crates/pistol-arena/Cargo.toml:23-29` — `pistol-core`,
`pistol-engine`, `pistol-cli`, serde/toml, and NOTHING else. The `pistol-engine`
dependency is used for exactly two symbols, neither a search entry point:
`crates/pistol-arena/src/openings.rs:6` (`use pistol_engine::PositionSpec;`) and
`crates/pistol-arena/src/bin/stub_engine.rs:6`.

pistol-core is the referee, in-process, as rule 2 requires —
`crates/pistol-arena/src/game.rs:88-89`:

```
crates/pistol-arena/src/game.rs:88           // pistol-core is the referee and the only judge of legality (rule 2).
crates/pistol-arena/src/game.rs:89           let outcome = match state.make_turn(turn) {
```

**This is the answer to P1's black-box question and P2's conflict alike: the shape
already exists, is CI-gated (`tools/ci.sh:157-158`, `gate 15/19: arena self-match
smoke`), and has 21 integration test files behind it.**

## P5.2 It already plays from a book, paired

```
crates/pistol-arena/src/openings.rs:57   pub fn load(path: &Path, take: usize, skip: usize, turn_cap: u32) -> Result<Openings, ArenaError> {
crates/pistol-arena/src/openings.rs:61       let found = pistol_cli::sha256::sha256_hex(&bytes[body_offset..]);
crates/pistol-arena/src/openings.rs:83       refuse_symmetry_duplicates(path, &parsed)?;
```

game seeding, `crates/pistol-arena/src/game.rs:30-31`:

```
crates/pistol-arena/src/game.rs:30       let mut state = replayed(opening);
crates/pistol-arena/src/game.rs:31       let mut moves = opening.moves.clone();
```

and the pairing, `crates/pistol-arena/src/schedule.rs:131-134`:

```
crates/pistol-arena/src/schedule.rs:131       let opening = &openings.taken[index / 2];
crates/pistol-arena/src/schedule.rs:132       // Even index: engine A takes the first seat. Odd: engine B does. So the
crates/pistol-arena/src/schedule.rs:133       // report's order is opening index, then side assignment, by construction.
crates/pistol-arena/src/schedule.rs:134       let a_is_p1 = index.is_multiple_of(2);
```

## P5.3 It emits per-GAME records only, and it THROWS AWAY the score

The record, `crates/pistol-arena/src/record.rs:101-122`:

```
crates/pistol-arena/src/record.rs:102   pub struct GameRecord {
crates/pistol-arena/src/record.rs:104       pub index: usize,
crates/pistol-arena/src/record.rs:106       pub opening: usize,
crates/pistol-arena/src/record.rs:108       pub a_is_p1: bool,
crates/pistol-arena/src/record.rs:110       pub result: GameResult,
crates/pistol-arena/src/record.rs:112       pub end: End,
crates/pistol-arena/src/record.rs:119       pub moves: Vec<Turn>,
crates/pistol-arena/src/record.rs:121       pub compute: [Compute; 2],
```

and compute is a whole-game AGGREGATE, `crates/pistol-arena/src/record.rs:71-89`:

```
crates/pistol-arena/src/record.rs:71   pub struct Compute {
crates/pistol-arena/src/record.rs:73       pub nodes: u64,
crates/pistol-arena/src/record.rs:75       pub time_ms: u64,
crates/pistol-arena/src/record.rs:77       pub max_depth: u32,
crates/pistol-arena/src/record.rs:79       pub searches: u32,
crates/pistol-arena/src/record.rs:83       /// Fold one `info totals` line's numbers in.
crates/pistol-arena/src/record.rs:84       pub fn add(&mut self, nodes: u64, time_ms: u64, depth_turns: u32) {
crates/pistol-arena/src/record.rs:85           self.nodes = self.nodes.saturating_add(nodes);
crates/pistol-arena/src/record.rs:87           self.max_depth = self.max_depth.max(depth_turns);
```

**THE DECISIVE LINE — the arena reads THREE fields off `info totals` and discards the
score and the pv**, `crates/pistol-arena/src/exchange.rs:169-189` (verified
independently by this session):

```
crates/pistol-arena/src/exchange.rs:169   fn totals_of(line: &str) -> Option<(u64, u64, u32)> {
crates/pistol-arena/src/exchange.rs:184       Some((
crates/pistol-arena/src/exchange.rs:185           value("nodes")?.parse().ok()?,
crates/pistol-arena/src/exchange.rs:186           value(pistol_cli::report::TIME_FIELD)?.parse().ok()?,
crates/pistol-arena/src/exchange.rs:187           value("depth_turns")?.parse().ok()?,
crates/pistol-arena/src/exchange.rs:188       ))
```

The `score` and `pv` are on that very line (`crates/pistol-cli/src/report.rs:83-96`)
and are simply not read. **So the label WP-2.0 wants is already on the wire the arena
is already reading — it is dropped one function short.**

The writer is one `game` line and one `moves` line per game,
`crates/pistol-arena/src/conclusion.rs:37-57`:

```
crates/pistol-arena/src/conclusion.rs:39               "game {} opening {} p1 {p1} p2 {p2} result {} end {end} forfeit_by {by} reason \
crates/pistol-arena/src/conclusion.rs:40                {reason} turns {} dup_of {dup} nodes_a {} nodes_b {} depth_a {} depth_b {} \
crates/pistol-arena/src/conclusion.rs:41                llr_game {} llr_pair {}",
crates/pistol-arena/src/conclusion.rs:53           let mut moves = format!("moves {}", record.index);
```

Dedupe and SPRT machinery exist and are reusable —
`crates/pistol-arena/src/dedupe.rs:12-30`, `crates/pistol-arena/src/sprt.rs:169-183`,
`crates/pistol-arena/src/score.rs:6-33` — but they are a VERDICT apparatus, and
WP-2.0 makes no strength claim.

## P5.4 Determinism: structural, and there is NO seed

**MEASURED**: `/usr/bin/grep -rn -i 'seed\|rng\|rand\|shuffl' crates/pistol-arena/src/`
returns **nothing**. Determinism is instead a pure function of the book window and
index parity, closed over by a digest —
`crates/pistol-arena/src/report.rs:29-40`:

```
crates/pistol-arena/src/report.rs:29   /// The fields that define the EXPERIMENT, hashed into the verdict block.
crates/pistol-arena/src/report.rs:31   /// Not the config document's own digest, and the difference is the point. The
crates/pistol-arena/src/report.rs:32   /// document also carries `n_workers` and `hang_timeout_ms`, which are run
crates/pistol-arena/src/report.rs:33   /// mechanics rather than experiment parameters
```

with the worker-invariance contract at
`crates/pistol-arena/src/report.rs:230-231`: *"The part of a report two worker counts
must agree on, byte for byte."*

**That is WP-2.0's `(book range, config, engine SHA)` already implemented — minus the
seed, which the arena has never needed because it has no stochastic element.** Seeds
exist only in the book GENERATOR:
`crates/pistol-cli/src/random_openings/config.rs:73-75` (`pub seed: u64`),
`crates/pistol-cli/src/random_openings/rng.rs:37-43` (`SplitMix64::from_seed`),
`configs/random_openings_v2.toml:63` (`seed = 20260830`).

The arena's engine identity is content-verified on every respawn —
`crates/pistol-arena/src/identity.rs:99` (`verify_respawn`),
`crates/pistol-arena/src/seats.rs:46`, and the config's own
`binary_sha256` (`crates/pistol-arena/src/config.rs:190`) — which is requirement 4's
"engine SHA", already enforced rather than merely recorded.

## P5.5 Reuse, extend, or sit beside — the facts, not a recommendation

| WP-2.0 requirement | arena today | evidence |
|---|---|---|
| black-box over line protocol | **DONE** | `channel.rs:57-63`, `exchange.rs:37-63`, pinned by `workspace_shape_tests.rs:119-137` |
| games from `book_v2` openings, paired | **DONE for a book**; no config names v2 | `openings.rs:57-114`, `schedule.rs:131-134`; §P3.4 |
| same config both sides (self-play) | **EXPRESSIBLE, never done** | `config.rs:168-193` — `engine_a`/`engine_b` are two independent sections; nothing forbids identical values |
| node budget, instrument mode | **DONE** | `config.rs:91-112`; movetime refused by name at `validate.rs:39-45`; mode pinned at `handshake.rs:5` (`REQUIRED_MODE`) |
| ONE RECORD PER POSITION with a label | **ABSENT** | `record.rs:101-122` is per-game; `exchange.rs:184-188` drops score and pv |
| deep-search label at a SECOND budget | **ABSENT** | `config.rs:36` has ONE `budget: BudgetSection`; `Rules.go_line` is one string, `game.rs:10` |
| census logging per game | **ABSENT and unreachable** | §P2.1 |
| seed | **ABSENT** | §P5.4 |
| corpus/manifest sink | **ABSENT** | `RunSection` has six keys, none an output path (`config.rs:48-83`); output is `arena --out <path>`, `bin/arena.rs:20` |

The crate states its own boundary at `crates/pistol-arena/src/lib.rs:34-39`:

```
crates/pistol-arena/src/lib.rs:34   //! # What this crate is not
crates/pistol-arena/src/lib.rs:36   //! It is not the Stage-5 harness: no book generation, no BayesElo, no
crates/pistol-arena/src/lib.rs:37   //! pentanomial *manager* beyond paired bookkeeping. And it holds no code
crates/pistol-arena/src/lib.rs:38   //! specific to any external engine — both sides are the pistol CLI speaking the
crates/pistol-arena/src/lib.rs:39   //! line protocol, and an external opponent is the bridge's job.
```

and its purpose at `crates/pistol-arena/src/lib.rs:3-6`: *"Hard Rule 6 makes SPRT over
paired balanced openings the judge of every search and eval change, so this crate is
the judge every later work package is tried by."*

**P5 VERDICT: HOLDS FOR THE SEAM, FAILS FOR THE RECORD.** The protocol driving, the
book windowing, the process isolation, the engine-identity verification and the
structural determinism are all present, tested and CI-gated. The per-position record,
the second (label) budget, the census, the seed and the corpus sink are all absent.

---

# WHAT THE DESIGN MUST DECIDE BECAUSE OF THESE PREMISES

Enumerated. No recommendations — these are the decisions the premises force, each one
a place where more than one option is viable and where CLAUDE.md's Process section
therefore wants an OPTION MATRIX before a choice.

1. **Where the census comes from, given it is not on the wire (P2.1).** The
   black-box constraint and requirement 3 are in direct conflict at HEAD. At least
   four shapes exist and none is obviously right: put the census on the wire (an
   engine change the package puts out of scope); add a config key (violates *"no
   committed config can ask"*, `search.rs:203-204`, and the reason that sentence
   exists); run a SEPARATE census pass over the emitted positions with a linking
   instrument (the `trigger_census` shape, at the ~23-45x cost of §P4.4(d)); or
   re-scope requirement 3. **The package cannot be designed until this is settled,
   because it decides whether the pipeline is one program or two.**

2. **How a census row acquires position identity (P2.4).** D-537 counts *"WIN-PROVING
   FIRINGS ON DISJOINT POSITIONS"* and the row carries none. Adding
   `GameState::key()` to `TriggerObservation` costs the struct its `Copy`
   (`census.rs:13`) if a move list is carried instead of a key; carrying only a
   128-bit key gives disjointness but not reproducibility of the position.

3. **Whether the pipeline extends `pistol-arena` or sits beside it (P5.5).** The
   arena already meets the seam requirement and already windows a book; it does not
   emit per-position records, has one budget, and its purpose statement
   (`lib.rs:3-6`) is SPRT adjudication. Extending it means adding a non-verdict
   output path to the crate that is *"the judge every later work package is tried
   by"*.

4. **How the GAME budget and the LABEL budget are both expressed.** `ArenaConfig` has
   exactly one `budget` (`config.rs:36`) and `Rules.go_line` is a single string
   (`game.rs:10`). Two registered budgets need either a schema change (and an
   `ARENA_SCHEMA_VERSION` bump, which `config.rs:7-10` shows is the established
   move) or a second document.

5. **Where the seed goes, and what it seeds (P5.4).** The arena is deterministic
   without one because it has no stochastic element. Requirement 4 names a seed; the
   design must say what is random — and if nothing is, whether the seed is a
   registered constant that appears in the manifest or is dropped.

6. **Which `book_v2` range WP-2.0 consumes, and whether it may (P3.2).** The ledger is
   empty, but 4500 was sized as `P + 500` for two OTHER claimants (D-518), and the
   ledger's rule requires the row to land *in the same commit as the arena config*
   (`book_v2_ledger.md:16-19`). A self-play corpus wants far more openings than an
   SPRT slice.

7. **How the pipeline detects game over (P1.2).** The protocol will not say; a driver
   that plays into a win gets an `error` from `set_position`
   (`position.rs:68-73`). The arena's answer is to link `pistol-core` as referee
   (`game.rs:88-89`) — the design must say it takes that answer, because rule 2
   forbids any other.

8. **Process isolation across games, given D-7 (P2.3).** D-527 cost a matrix revision
   because one `Searcher` spanned 24 games. The pipeline plays many games in
   sequence. `newgame` (P1.5), one process per game (`bench_block.sh:28-31`), and the
   arena's per-game respawn (`seats.rs:27-47`) are three different answers with three
   different costs.

9. **What the label's `depth` and `nodes` MEAN, given `Provenance` is off the wire
   (P4.2).** Under a node budget a `SolverProof` label's depth is a proof depth and
   its nodes are solver nodes; the schema must either record provenance (needing it on
   the wire) or the seat must guarantee the gate is off (which every committed config
   already does, `instrument_staged_snk_v0.toml:78`).

10. **What the score field is CALLED in the schema (P4.1).** `cp` is not centipawns
    (`report.rs:149-152`), the sign is root-side-to-move (`info.rs:153-154`), and
    `mate T` counts both sides' turns (`report.rs:147-149`). A schema that names any
    of these wrongly makes every label in the corpus wrong in a way no loader test
    catches.

11. **Whether one record per position means one per PLY or one per TURN.** The rules
    unit is the turn (rule 3), `depth_turns` is in turns, and `PositionSpec::Start`
    *"always names a position at a turn boundary"* (`position.rs:9-10`) — but
    `PositionSpec::Set` can express a mid-turn position (`position.rs:37-46`, D-6),
    and rule 4 means a game can end on a first stone. This is D-477's own unit
    question and the axis the record is indexed on.

12. **What the corpus manifest digests, given the book's whole-file parse (P3.5).**
    The arena's `experiment_sha256` (`report.rs:29-40`) is the existing precedent and
    deliberately excludes run mechanics; a corpus manifest has to decide the same
    boundary for a pipeline whose output is data rather than a verdict.

---

# THE PREMISE MOST LIKELY TO BE WRONG

**My own adversarial read of my own memo: it is P1, and specifically my "HOLDS".**

Not because any quotation in §P1 is wrong — each is at the line that consumes the
unit. Because **"a driver can extract a deep-search label using only the line
protocol" is a claim about a SEAM, and I verified it one `go` at a time, never over a
GAME.** Every artifact I read is a one-position bench:
`tools/bench_block.sh:247` sends exactly `newgame / position / go / quit` and exits.
The arena plays games, but it asks for a move — it never asks a second time at a
second budget, and it discards the very fields the label needs
(`exchange.rs:184-188`). **So the composite operation WP-2.0 actually needs — play a
turn at the game budget, then re-ask the SAME position at the label budget, then
advance — has no precedent anywhere in this tree, and I proved its parts, not the
whole.**

Three specific ways P1 could turn out weaker than "HOLDS":

- **The double-`go` interaction with the transposition table.** A label `go` at
  `nodes 50000` immediately after a game `go` at a smaller budget on the SAME position
  runs on a table the first search just warmed. That is not a determinism breach —
  it is reproducible — but it means the label is NOT the label that budget would
  produce cold, and D-527 is this project's own record of exactly that confusion
  costing a matrix revision. Nothing I quoted rules it in or out; `Searcher::clear()`
  (`search.rs:229-239`) is per-GAME, and the only per-position isolation that exists
  is a fresh process (`bench_block.sh:28-31`). **I did not verify this and could not
  without a run.**

- **Cost.** If the answer to that is "one process per label", then §P4.4's ~434k nps
  is the wrong shape entirely, because process spawn plus a 256 MB table allocation
  (`tt_bytes = 268435456`, `instrument_staged_snk_v0.toml:27`) is paid per POSITION
  rather than per game. I reported nps and refused to extrapolate; that refusal is
  correct under D-500's class, but it also means **this memo does not know whether
  the pipeline is cheap.**

- **`Provenance`.** I graded it "a narrow gap" on the grounds that the two salvage
  provenances are wall-clock-only. That is correctly quoted
  (`info.rs:251-254`), but it makes the label's soundness depend on a CONFIG
  property (gate off) rather than on anything the wire says — and a config is exactly
  the thing a future SPRT is licensed to change.

Second most likely to be wrong, and I want it on the record: **my P2 verdict of
"FAILS" may be too strong in one direction and too weak in another.** Too strong,
because "reachable only by linking `pistol-search`" describes the CURRENT tree, and a
separate census pass over the emitted positions is a perfectly ordinary answer that
keeps the pipeline black-box — the conflict may be dissolvable rather than blocking,
and calling it FAILS may read as a verdict on the package when it is a verdict on
requirement 3 as literally written.

It is not, however, too weak on the evidence, and that is worth saying because my
first draft of it was. I had written the absence up from a keyword grep for "census",
which proves absence only of that word; I then enumerated the engine config's entire
key set instead (§P2.1(d)) — 6 sections, 23 top-level fields, 12 nested
candidate-policy fields, under `deny_unknown_fields` — and there is no census key
under any name. **That check cost seconds and I should have run it before writing the
verdict rather than after; D-291 names an estimate that could have been measured in
seconds as a finding, and this was one against this memo.**
