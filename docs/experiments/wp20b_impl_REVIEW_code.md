# WP-2.0b — REVIEW-impl (CODE). Fresh context, not the implementer.

**NAMED REVISION:** `f7606cc7809cc19e9b481a57af7f0a662cddd831` — a `git stash create`
object over the uncommitted work on `dev` (HEAD `a6777f4`).

**DOES IT STILL MATCH THE WORKTREE? YES.** Taken independently:
`git add -A && git stash create` returned `0a845736f80c68bac8ca544bcc96e0ccdaab2cbf`,
and `git diff f7606cc 0a84573 --stat` is **empty**. The revision under review is the
tree on disk.

**VERDICT: PASS.** No blocking defect. The identity is correct at both firing sites,
the side-to-move argument §2 rests on is sound and enforced in code, the token is
byte-identical OFF under a rule I re-derived myself, the arming rule holds on the error
path, and every mutant the design registered dies. **Two MAJOR findings**, of which
**M1 is the one to weigh before WP-2.1**: the census's whole in-tree row population —
the population the design's own §2 says the fold exists for — has no test that reads a
row's `key`, and I have a surviving mutant that proves it.

**SCOPE NOTE.** The revision is not confined to WP-2.0b (see m6). This report judges
the WP-2.0b diff only: `crates/pistol-{core,search,engine,cli,arena}` plus the
artifacts and documents WP-2.0b's §9 registers.

**WHAT I RAN.** A detached worktree at `f7606cc` under `/home/tom/pistol-runs/` with
its own `CARGO_TARGET_DIR` (never the live tree, never `/tmp`):
`cargo clippy --workspace --all-targets -- -D clippy::all` (clean);
`canonical_key_tests` 7/7, `census_identity_tests` 4/4, `census_protocol_tests` 13/13,
`report_tests` 11/11, `census_capture_tests` 7/7, `capture_tests` 39/39,
`trigger_census_cover_tests` 3/3, and in the live tree `workspace_shape_tests` 4/4 and
`file_justification_gate_tests` 9/9. All green. The worktree was removed after the run;
it held no `artifacts/` or `sessions/` of its own, and every reproducer below recreates
it in one command.

---

## THE FOUR ANSWERS THE DISPATCH ASKED FOR

**1. Does the identity do what D-537's denominator needs? YES.** The fold's input is
`state.board().stones()` at both sites — `pvs.rs:636-639` inside the existing
`self.census.is_some().then(..)` closure, over the `state` from
`self.position.staged_context()` (the node's own state), and `search.rs:764-766` over
`position.staged_context()` after `reset_to(state)` (the root's own state). Both are
the position AT the decision. `key_pos` is `state.key()` from the same borrow at both
sites — the right state.

**§2's side-to-move argument is sound, and it is enforced rather than assumed.**
`GameState::key`'s own doc already carries the same reasoning
(`state.rs:126-133`: *"for an ongoing game the stone count fixes the turn, the phase and
the mover together"*), and the one route by which a caller could hand the engine a
stone set with a contradicting mover is closed by name:
`crates/pistol-engine/src/position.rs:172-180` refuses a `Set` document whose declared
`to_move`/`phase` disagree with what replaying the stone lists produces
(*"those stones leave X to move at phase N, and the document says …"*). So two firings
cannot share a key at two genuinely different decision points. I verified the refusal
exists at source rather than trusting the design's citation.

**2. Is the token OFF byte-identical? YES, and I re-derived it.** I recomputed the
receipt's rule myself over all eight records —
`sed -n '1,/^# timing/p' <record> | grep -v '^revision \|^binary_sha256 ' | sha256sum` —
and got `81e37d42…` for all four `gate_v0` records (pre and post, runs 1 and 2) and
`c7f155e8…` for all four `instrument_v0` records, matching the registered referents
exactly. **The extraction rule excludes only `revision` and `binary_sha256` and stops
at `# timing`**; I read a whole record to check what that leaves in, and it leaves in
the ten `engine_id` handshake lines, the corpus/openings digests, the budget, and all
24 `position … nodes … depth_turns … seldepth … hashfull … score … bestmove` lines plus
the three ladders. The excluded block is host, cores, tree-dirty and per-position
`time_ms`/`nps` — machine facts. **The rule does not exclude anything that should have
varied.** The `run 1 == run 2` check on both binaries is real and I reproduced it.

The one census-OFF output byte that **does** move is the FOURTH-word refusal
(`budget_token.rs:64-72`): `go nodes 100 a b` used to name `a`, and now names `b` with
different text. That is registered by design §3 (*"A FOURTH word is refused naming the
FOURTH word"*), is outside the standing position set, and `/usr/bin/grep -rn "takes one
budget"` finds no pre-existing test or tool pinning the old wording. Not a finding.

**3. The arming rule (invariant 8): HOLDS, including the error path.**
`instance.rs:104-129` runs `collect → search → take → stop` with `take` and `stop` on
the `Err` path as well (`answer` is bound, not `?`-propagated). I traced every early
exit of `Searcher::search` for a `take_trigger_census` panic: `check_root(…)?`
(`search.rs:274`) and the solver-proof `return Ok(…)` (`search.rs:353`) both precede
`run.census = self.census.take()` (`search.rs:416`), so `self.census` is still `Some`
at both; there is no `return` between `:416` and `self.census = run.census.take()`
(`:563`). **`take_trigger_census` cannot panic on any reachable path**, and rows cannot
cross a `go`: `collect_trigger_census` re-arms with a fresh `Vec` and resets
`census_folds`, and `stop_trigger_census` disarms. A census `go` that returns `Err`
disarms and returns the error; a budget refused by `Budget::resolve`/`stop_for` never
arms in the first place.

**4/5/6/7.** Every test is on the seat §8 assigns it and no ON-seated test reads an
empty row set — I confirmed the ARMED run produces a row (below). The arena sink is
closed and the closure is driven through the **binary** (`Command::new(ARENA)` with
`--census`, `census_capture_tests.rs:41-58`) against a stub that writes real
`census_line` bytes; I re-ran M14's shape by hand and the file is written. Rule 3: the
`--census` claim is taken before any game, and a colliding census file gives back the
`--out` claim and exits 2 — **reproduced**, see R2. Findings below.

---

## FINDINGS

### M1 (MAJOR) — no test anywhere reads an IN-TREE census row's `key`, and the seated protocol suite never produces one

**Claim.** `crates/pistol-cli/tests/census_protocol_tests.rs`'s single shared armed run
fires **exactly once**, at the **root**. Every seated test therefore reads one row with
`turns_from_root 0`, produced by `search.rs:764-766`. The other firing site —
`crates/pistol-search/src/pvs.rs:636-639`, which is where the census's production
population comes from — has its row's `key` read by **nothing**: the search-level suite
counts fold ENTRIES and asserts a row with `turns_from_root > 0` exists, but never looks
at what that row carries. This is M10's shape a second time: a workload that does not
reach the code a defect would live in.

**Sites.** `crates/pistol-search/src/pvs.rs:636-639`;
`crates/pistol-cli/tests/census_protocol_tests.rs:24` (`BUDGET = "go depth_turns 2"`),
`:113-141` (test 1, which pins the key VALUE only for the row whose
`turns_from_root == 0` and otherwise asserts 32 lower-case hex digits);
`crates/pistol-search/tests/census_identity_tests.rs:69-131`.

**Failure scenario.** The in-tree site emits matrix **option A** — `state.key()`, i.e.
`key_pos` — as the row's `key`. Option A is the identity F2 declares *"FAILS"* §8
compliance because it folds no symmetry, and the resulting mis-count runs in the
over-counting direction F2 names as *"the failure in the direction the rule was written
to prevent"*. Every row of a production sweep except one per search would carry it, and
D-537's disjointness denominator would be computed over the wrong equivalence — the
single thing this package exists to make countable. **No registered test fails.**

**MINIMAL REPRODUCER — RUN, not argued.**

```
# (a) the workload fact
git worktree add --detach /home/tom/pistol-runs/wp20b-review f7606cc
cd /home/tom/pistol-runs/wp20b-review
CARGO_TARGET_DIR=/home/tom/pistol-runs/wp20b-review-target cargo build --release --locked
printf 'position start moves 0,0 -1,1/1,0 0,1/0,2 -1,0/1,-1 0,-1/1,-2 0,-2/0,3 -1,-1/1,1 -1,2/-1,3\ngo depth_turns 2 census\nquit\n' \
  | /home/tom/pistol-runs/wp20b-review-target/release/pistol --config configs/gate_staged_solver_v0.toml
```

returns exactly **one** `info census` line, and it reads `turns_from_root 0`
(`solver_firings 1` on the totals line confirms it).

```
# (b) the surviving mutant, in crates/pistol-search/src/pvs.rs, inside the census closure:
-            let stones: Vec<(pistol_core::Coord, pistol_core::Player)> =
+            let _stones: Vec<(pistol_core::Coord, pistol_core::Player)> =
                 state.board().stones().collect();
-            let key = pistol_core::canonical_key(&stones);
+            let key = state.key();               // MUTANT M-A: matrix option A in-tree
             let key_pos = state.key();
```

Result, all in the worktree with its own target dir:

| suite | against the mutant |
|---|---|
| `pistol-core --test canonical_key_tests` | 7 passed, 0 failed |
| `pistol-search --test census_identity_tests` | 4 passed, 0 failed |
| `pistol-search --test trigger_census_cover_tests` | 3 passed, 0 failed (388 s) |
| `pistol-cli --test census_protocol_tests` | 13 passed, 0 failed |
| `pistol-cli --test report_tests` | 11 passed, 0 failed |
| `pistol-arena --test census_capture_tests` | 7 passed, 0 failed |

**45 tests, 0 failures. The mutant is alive.** (`/usr/bin/grep -rln
"take_trigger_census\|TriggerObservation" crates/*/tests` names exactly the three search-
and cli-side suites above plus the arena's, so this is the whole population that could
have seen it.)

**Why the registered set did not catch it.** `artifacts/wp20b_mutants_v2.txt`'s M2 —
*"the in-tree key read from the PLAY ORDER, not the position"* — died at
`a_census_row_carries_the_canonical_key_of_the_position_it_fired_at`, a test that reads
only the ROOT row, which means M2 as applied also touched `search.rs`'s root site. The
receipt records mutant NAMES and results but not the patches, so this could not be
checked from the receipt; it is visible only from the workload fact in (a). Nothing in
the twenty mutants is confined to `pvs.rs`'s row CONTENT.

**Not a wrong answer today** — the shipped code is correct at both sites, and every
mutant the design registered dies at its registered test. What is unmet is the coverage
D-553's law is about, at the one call site whose output the sweep will be counted over.
A one-line falsifier of the class exists (`assert_ne!(row.key, row.key_pos)` on the rows
`census_identity_tests` already collects, plus an in-tree row whose `key` differs from
the root's), but the remedy is the implementer's to choose.

### M2 (MAJOR) — the arena's census sink is O(whole artifact) in memory, three times over

**Claim.** A `--census` capture accumulates every row of the run in
`crates/pistol-arena/src/passes.rs:53` (`let mut rows: Vec<String>`), then
`census_file::render` copies them all into `Fixture::body`
(`crates/pistol-arena/src/census_file.rs:43-45`), then `Fixture::render`
(`crates/pistol-cli/src/corpus/emit.rs:93-99`) allocates a third `String` of
header+body before a single byte is written. Nothing streams.

**Failure scenario.** The design's own §2 sizes the sweep's census at **~3.0·10⁷ rows,
~291 B each, ~8.7 GB** (ESTIMATED, and it is the design's estimate, not mine). At that
scale the sink needs roughly **3× the artifact** resident — the `Vec<String>`, the
`body`, and the rendered copy — before `write_all`. A capture is per report, so a
tranche is a fraction of the sweep; even a tenth is ~2.5 GB resident on a workstation
whose whole point is a single-box budget. The failure is an allocation failure at the
END of a run that, by D-563, costs hours, and it destroys the run's rows.

**Reproducer (bounded, by inspection rather than by burning the RAM).** The three
allocations are the three lines cited; `Fixture` holds `header: String, body: String`
(`emit.rs:13-16`) and `render` is `String::with_capacity(header.len() + body.len() + 80)`
(`emit.rs:94`). I did not run an 8 GB capture — the seat that could produce rows cannot
be reached from any committed arena config (F3), which is also why this is latent rather
than blocking. It is recorded because the sink is exactly what WP-2.1 is being asked to
depend on, and `passes.rs:36-40`'s own comment — *"a census file discovered to be
unwritable after sixty hours of asking is sixty hours of rows nothing can hold"* — makes
the argument for the finding better than I can.

### m1 (MINOR) — the `CensusUnsupported` refusal goes on the wire with eighteen spaces in it

`crates/pistol-engine/src/error.rs:173-176`. Reproduced:

```
$ printf 'behave honest\n' > stub.toml
$ printf 'position start\ngo nodes 100 census\nquit\n' | ./arena-stub-engine --config stub.toml
error CensusUnsupported: `arena-stub-engine` cannot produce a trigger census, and answering with no rows would                  read as a search whose trigger never fired
```

A line-continuation was dropped from the string literal. Rule 3 wants a NAMED error and
gets one, but this is the text a driver shows an operator, and no test pins it —
`census_capture_tests.rs:168` asserts only that stderr contains `CensusUnsupported`.

### m2 (MINOR) — stdout claims a capture and a manifest row that a census failure then deletes

`crates/pistol-arena/src/passes.rs:60-69` prints the capture's `manifest_row` and
`arena: capture written to …` **before** the census file is written (`:71-86`). If the
census write or its `manifest_row` fails, `bin/arena.rs:125-135` abandons **both** files
and exits 2 — so a scraped stdout carries a `capture_manifest … body_sha256 …` row for
bytes that no longer exist. The run does fail loudly, which is why this is minor, but
the manifest row is the thing rule 8 and D-469 lean on.

### m3 (MINOR) — `CENSUS_FORMAT_VERSION` is a promise nothing binds

`crates/pistol-arena/src/census_file.rs:6-10` says *"A consumer refuses any other
version"*, but there is no consumer, and the file's identity is
`capture_sha256(experiment, label_go, capture_file::CAPTURE_FORMAT_VERSION)`
(`:22-26`, `:65-69`) — the CENSUS version is written as a `# param` and is **not** in
the digest. A census v2 would therefore produce a file with the **same**
`capture_sha256` as v1. `capture_tests::a_capture_identity_moves_when_the_format_version_moves`
pins exactly that property for the capture; the census has no counterpart, and by
construction cannot pass one.

### m4 (MINOR) — census rows bypass the `no_tab` guard every other captured field passes

`crates/pistol-arena/src/capture.rs:319` runs `no_tab(&record)` over the position,
totals and bestmove of every ask (`:334-345`); `Step::Census(row) => census.push(row)`
(`:268-271`) runs no check at all. Low impact — the census grammar is whitespace-
delimited, so a tab splits like a space rather than shifting a TSV column — but the
asymmetry is undocumented and a foreign engine's `info census …` line reaches the
committed-manifest-indexed artifact unvalidated.

### m5 (MINOR) — the perf guard's instrument survives only on a RAM-backed tmpfs

`artifacts/wp20b_perf_RECEIPT.txt` names `…/scratchpad/wp20b_perf_guard.sh`
(sha256 `9ec50053…`) and `…/scratchpad/wp20b_perf_report.py` (sha256 `0a6a0f87…`).
Both files still exist and **both digests verify** — I checked, and I read the script
against design §9's block: same seat, same fixture, the two `%%`/`grep` filters intact,
`newgame` before every position, plus the third arm and the four counters `wp20b_impl.md`
§3 registers. So the number is sound today. But `docs/process.md`'s instrument rule makes
the script part of the registration, CLAUDE.md records that this machine's `/tmp` is a
24 GiB tmpfs, and MEMORY's own lesson is that scratchpad-only artefacts do not survive.
Export both to `artifacts/` with the receipt's digests before closure.

Same class, second item: the mutation worktree `/home/tom/pistol-runs/wp20bmut`
(detached at `dfba9d7`) is still checked out. `git worktree list` shows it; CLAUDE.md's
Process forbids removing it before its gitignored `artifacts/` and `sessions/` are
exported with a digest receipt. `artifacts/wp20b_mutants_v{1,2}.txt` are in the main
tree, so the receipts survive; whether anything else in that worktree does has not been
checked here, and closure should check before removing it. I left it alone.

### m6 (MINOR) — the reviewed revision carries work that is not WP-2.0b

`tools/wp21_tranche_config.py`, `crates/pistol-arena/tests/wp21_tranche_config_tests.rs`,
`docs/experiments/wp21_prereg.md`, `docs/experiments/overnight2_ledger.md` and
`docs/book_v2_ledger.md` are in the same uncommitted tree. They are outside this
review's subject and **were not reviewed** — in particular `tools/wp21_tranche_config.py`
has had no `tools/SHELL_CHECKLIST.md` review, which `docs/process.md`'s coverage rule
requires of a `tools/` artefact that produces a recorded number. CLAUDE.md's Closure
section wants one feature per commit; naming this so the WP-2.0b commit does not absorb
them.

### m7 (MINOR) — a `--census` capture on a gate-off seat writes `rows 0` at exit 0, untested

Every committed arena config points both seats at `configs/instrument_v0.toml` (F3), so
the realistic first production `--census` run writes a census file whose body is empty
and whose header says `# derived rows 0`, at exit 0, having spent the whole capture.
That is a legitimate answer (the trigger never fired) and the design does not ask for a
refusal — but it is F3's failure mode wearing a receipt, invariant 7's *"or the run
fails loudly"* does not reach it, and `census_capture_tests` has no case for it. The
`rows 0` line is the only signal; WP-2.1 should read it before believing a tranche.

### m8 (MINOR) — two documentation obligations still open

`docs/experiments/wp20b_impl.md` jumps from **§4 to §6** — there is no §5, and a reader
sent to *"§4 departure 2"* will hunt for a section that does not exist. Separately,
design §10.3 books an ADR line for D-537's counting rule *"at this package's closure"*;
`docs/decisions.md` gains D-565 and D-566 in this diff, neither of which is it, and no
line records the census artifact class, the `Step::Census` seam change or the
`--census` flag (hard rule 10). Both are closure obligations, not implementation
defects; recorded so closure does not pass over them.

---

## REJECTED

### R1 (REJECTED) — "the byte-identity receipt's post-change binary is stale"

**The suspicion, which was worth having.** `find crates -name '*.rs' -newer
target/release/pistol` returns `crates/pistol-cli/src/protocol.rs`, whose mtime
(`00:07:25`) is **twelve minutes after** the binary the receipt names was built
(`23:55:01`) — and `protocol.rs` is the file carrying the census emission, the most
byte-identity-relevant file in the diff.

**Reproducer, and why it rejects the finding.** A fresh from-scratch release build of
`f7606cc` in a detached worktree with an empty `CARGO_TARGET_DIR`
(`/home/tom/pistol-runs/build.log` shows every dependency compiling, so it is not an
incremental reuse) produced

```
15c94598569fb04a385a0bea3a9c04d58bf52a49065f12b57092fa11eed3b7e3  .../wp20b-review-target/release/pistol
15c94598569fb04a385a0bea3a9c04d58bf52a49065f12b57092fa11eed3b7e3  target/release/pistol
```

— byte-identical, and equal to the receipt's post-change binary. The binary the
byte-identity and perf receipts were taken with **is** a build of the revision under
review; the mtime is a touch, not a content change. Recorded rather than dropped because
the next reviewer will meet the same mtime.

### R2 (REJECTED) — "`bin/arena.rs`'s `census_path` error arm leaks the claimed `--out`"

**The suspicion.** `bin/arena.rs:95` is
`outpath::census_path(&out_path).map_err(|e| e.to_string())?` — a `?` with **no**
`abandon(&out_path)`, unlike the `claim` failure arm three lines below it, which does
give the `--out` claim back. A caller reaching it would leave a zero-byte claimed
report behind and block the retry.

**Reproducer attempted, and it cannot be reached.** `census_path` fails only when
`Path::file_stem` is `None` — the final component being `.`, `..`, or empty — and
`outpath::claim` refuses every such `--out` first:

```
$ arena --capture missing.txt --out $S/.  --label-nodes 5000 --census
arena: Io: claiming …/.  … File exists (os error 17)
$ arena --capture missing.txt --out $S/.. --label-nodes 5000 --census
arena: Io: claiming …/.. … File exists (os error 17)
$ arena --capture missing.txt --out $S/   --label-nodes 5000 --census
arena: Io: claiming …/ … Is a directory (os error 21)
```

No `--out` reaches `census_path` with a `None` stem. The arm is unreachable and the
asymmetry is cosmetic. **The claim-collision path that IS reachable behaves correctly**
and was run: with `cap.census.txt` pre-existing, `arena --capture … --out cap.txt
--label-nodes 5000 --census` refuses by name, exits 2, leaves the pre-existing census
file untouched, and `cap.txt` is **not** left behind — the `--out` claim was given back.

---

## COUNT BY CLASS

| class | n |
|---|---|
| **BLOCKING** | **0** |
| MAJOR | 2 (M1, M2) |
| MINOR | 8 (m1–m8) |
| REJECTED | 2 (R1, R2) |

The one finding that changes what anyone may conclude is **M1**: it is the second
instance of the vacuity class M10 was, it is at the site the census's production
population comes from, and it has a live mutant.
