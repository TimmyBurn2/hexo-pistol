# tools/sealbot — the local HeXO match platform and the sealbot adapter

An ANCHOR harness: it stands up a local server that follows the official HeXO
game rules (hexo.did.science) and refereees matches between two engine
processes, so pistol can be compared against external engines (first: sealbot)
without the remote. It is not SPRT, not paired, not an Elo instrument — the
report it writes says "anchor", and standing judgment stays: bot far below
strong humans until measured against them (docs/research/sealbot_notes.md,
D-197).

Three artefacts live here:

1. `matchserver/` — the local match server (Rust). The referee and driver.
2. `sealbot_shim.py` — the sealbot-side engine shim (JSON lines).
3. `pistol_hexo_adapter.py` — the pistol-side hexo-bridge engine adapter, for
   the day the remote platform is reachable again. NOT exercised by the local
   match; kept because the platform contract (D-211's mapping) is stable.

Everything machine-specific — paths, budgets for a particular run, output
stems — lives in a gitignored `local/*.toml` (root `/local/`), with
`local/sealbot.example.toml` committed as the shape.

## Why Rust, and why the referee is pistol-core

The referee must enforce the official rules — hex lattice, 6-in-a-row with
overlines winning, 1-then-2 stones, sudden death on the completing stone,
and the radius-8 placement region the platform enforces (D-101: the operator
confirmed radius 8 against the htttx spec and the HeXO server). Those rules
already exist, once, in pistol-core — CLAUDE.md rule 2 forbids a second
implementation, and a Python referee would be exactly that. So the referee is
`pistol-core::GameState` called in-process, the same object the arena and the
engine's own replay path use, and no rule is restated here.

Performance: the referee is in-process (thousands of `place` calls per game
cost microseconds); engines run as subprocesses and are the only real compute.
The arena already proves this shape fast at thousands of games. A Python
server would need either subprocess round-trips per rules check or a second
rules implementation — both worse than the problem they solve.

## The platform's game rules, as the server enforces them

Every line comes from the official platform's behaviour as pinned by the
bridge docs and D-101; each is enforced by pistol-core, never by this crate:

1. Board: unbounded hexagonal lattice, axial (q, r).
2. Win: 6 or more own stones contiguous on one axis; overlines win.
3. Turn 1 places ONE stone. The platform's standard opening auto-plays it at
   the origin (the htttx `setup` packet delivers one cross at (0,0) owned by
   p1); engines are first asked at turn 2. The server does the same.
4. Every later turn places TWO stones. A stone that completes a line ends the
   game the instant it is placed (rule 4 sudden death): the turn's remaining
   stone, if the engine submitted one, is recorded but NOT applied.
5. Placement is legal only within hex-distance 8 of an existing stone (union
   of radius-8 balls) — a platform rule (D-101), enforced as written.
6. No captures; no rule-level draws. A configured turn cap is an evaluation
   horizon, reported as "capped", never a win for either side.

### How a submitted move is judged

Stones are applied one at a time, in the order the engine submitted them:

- `place` refuses (occupied cell, outside the legal region, game decided) →
  ILLEGAL MOVE; the opponent wins, as the platform's `finishReason:
  illegal-move` does. The named refusal is recorded.
- a stone that completes 6 ends the game instantly (see rule 4 above).
- a submitted turn that stops short of the stones owed without winning →
  ILLEGAL (incomplete turn); forfeit to the opponent.
- engine crash, protocol break, or exceeding the per-turn wall cap → forfeit
  to the opponent, reason recorded distinctly from illegal-move.

### Why the pistol client recovers play order

pistol's `bestmove` spells a pair canonically (smaller cell first), but which
stone goes down FIRST is a legality question pistol-core answers when the turn
is made (D-6: a pair is legal iff some ordering is; D-52 constructs pairs
legal in only one order). The client replays the game on its own shadow
`GameState`, calls `make_turn` there, and submits the stones in the order
pistol-core actually plays them — so the referee's strict submitted-order
semantics never misjudge a legal pair.

## Shape

```
matchserver/src/
  main.rs          CLI: one config path, run the match, write the report
  config.rs        the TOML schema (deny_unknown_fields; no code-side defaults)
  referee.rs       the game loop over pistol-core; the rules above live here
  client.rs        EngineClient trait + subprocess plumbing (timeouts, capture)
  pistol_client.rs the pistol line-protocol driver (instrument mode: go nodes)
  sealbot_client.rs the JSON-lines driver for sealbot_shim.py
  report.rs        Wilson 95% interval + report rendering
  transcript.rs    JSONL transcript + report writers
matchserver/src/bin/
  replay_check.rs  the second instrument: replay transcripts, verify the record
```

The extension seam is `EngineClient`: a new engine is a new client module plus
a config `kind`; the referee never changes. Both engine kinds are configured
as a `command` argv plus kind-specific budget fields, so stub engines for
tests use the same driver code the real engines use.

## Determinism

The pistol side runs instrument mode (`go nodes <registered>`, single thread,
stable tie-break) — the same instrument the arena's strength claims use. The
transcript records every stone and every engine reply, so a game is replayable
from its own transcript. sealbot is time-budgeted and therefore not
run-to-run deterministic; it is the UNVERIFIED side of an anchor, and the
report says so.

## The engine pin, and why the binary is built at e2280ca

The anchor measures the WP-1.7-closure engine (docs/decisions.md D-433),
whose committed binaries are pinned at revision `e2280ca`. Between `e2280ca`
and current `dev` HEAD, no crate the `pistol` binary builds from changed
(`git diff --stat e2280ca..<head> -- crates/pistol-core crates/pistol-eval
crates/pistol-search crates/pistol-engine crates/pistol-cli` is empty), yet a
release build at HEAD has a DIFFERENT sha256 — the only lockfile change
(pistol-solver gaining serde) shifts the build-graph fingerprint and with it
the symbol-metadata hashes, so byte-identity of the source is not
byte-identity of the binary. The defensible engine is therefore not
"HEAD, same sources" but the binary whose digest REPRODUCES the ADR pin:

```
git worktree add --detach <dir> e2280ca
cargo build --release --locked -p pistol-cli
sha256sum <dir>/target/release/pistol   # must equal 665d2815… (D-433's pin)
```

Measured on this machine: the `e2280ca` build reproduces
`665d2815ddba28e7889ebea661a10b15352036ab46bfc6f1758d72813cad4184` in two
different directories; the HEAD build is `2fc9cebe…` in both of ITS
directories. The match config points at the pinned worktree's binary (paths
may be absolute; only `output_dir` is held repo-relative).

## The second instrument

`replay_check.rs` re-reads what the match WROTE and replays it stone by stone
through pistol-core, confirming each transcript's turns, wins and end states
against the rules. It shares pistol-core with the referee deliberately — the
rules are not the stage under doubt; the RECORD is. It checks what no in-run
path can: that the bytes on disk are the game that was played. Its negative
control (a tampered winner must fail) is part of the test suite.

## Running

```
tools/sealbot/run_match.sh local/<config>.toml
```

builds the matchserver, runs the match, and prints the report path and the
sha256 digests of everything written. Transcript and report land in
`artifacts/`, content-named, never committed (CLAUDE.md rule 8); their
digests are what the ADR anchors.

Tests: `tools/sealbot/tests/run_tests.sh` drives the SHIPPED script with two
stub engines through a scripted game with hand-computed outcomes (a
first-stone win and an illegal-move forfeit), plus the replay checker over
both records and a tampered-record negative control — because any tools/
artefact that produces a recorded number carries a test driving it
(tools/SHELL_CHECKLIST.md item 10).

`pistol_hexo_adapter.py` is smoke-tested by hand against the real binary
(positive: a two-stone answer under `go movetime`; negative: a free_setup
board and a non-decomposing stone stream are both refused loudly). It is NOT
exercised by the anchor match, which runs against the local server.
