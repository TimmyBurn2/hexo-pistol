# REVIEW-design — `anchor_v3_openings_design.md` revision 1

**Revision read**: `963c8fddc2c2b4dc62de5ff9829ab8a527977457` (a `git stash create`
object). **Does it match HEAD?** No — HEAD is `d83ac01` and this revision is HEAD
plus the working tree's 13 uncommitted paths, of which
`docs/experiments/anchor_v3_openings_design.md` is one (added). The tree did NOT
move under this review: `git diff --stat 963c8fdd $(git stash create)` printed
nothing, so every file below was read from the working tree at the named revision.

**Fresh context; I did not write the design.** No `cargo`, no `tools/ci.sh`, no
bench, no match was run — a wall-clock instrument is live on this box. Every claim
below is from reading source, and the two claims that could only be settled by
running something are marked UNVERIFIED LIMB in place.

**Files read in full**: `CLAUDE.md`; `docs/experiments/anchor_v3_openings_design.md`;
`docs/process.md`; `tools/SHELL_CHECKLIST.md`; `tools/sealbot/README.md`;
`tools/sealbot/run_match.sh`; `tools/sealbot/tests/run_tests.sh`;
`tools/sealbot/sealbot_shim.py`; `tools/sealbot/pistol_hexo_adapter.py`;
`tools/sealbot/matchserver/src/{main,config,referee,transcript,sealbot_client,pistol_client,report,client,budget}.rs`;
`tools/sealbot/matchserver/src/bin/replay_check.rs`;
`tools/sealbot/matchserver/Cargo.toml`; `crates/pistol-arena/src/openings.rs`;
`docs/experiments/sealbot_anchor_v2_prereg.md`;
`docs/experiments/sealbot_anchor_prereg.md`.
**Read in part**: `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` (header
+ body shape + line count); `crates/pistol-cli/tests/random_openings_document_tests.rs`
(lines 1–80); `crates/pistol-cli/src/random_openings/mod.rs` (lines 1–80);
`crates/pistol-core/src/{play,turn}.rs` (the turn/pair paths);
`crates/pistol-engine/src/position.rs` (`replay`); `crates/pistol-engine/Cargo.toml`;
`crates/pistol-arena/src/config.rs`; `tools/ci.sh` (gate 16);
`docs/decisions.md` (D-6, D-22, D-52, D-95, D-101, D-151, D-175, D-197, D-423,
D-424, D-438, D-505, D-534, D-568); `docs/experiments/overnight2_ledger.md` §2b.

**Citation audit — the design's §1 table and its D-numbers.** Every file:line in
§1 checks out: `referee.rs:17` is the `const OPENING`, `referee.rs:106-112` is the
place + `plies` seed, `main.rs:75` is `let a_is_p1 = game % 2 == 1`,
`transcript.rs:25` is the fixed `"opening"` sentence, `sealbot_client.rs:69-80` is
`request`'s split, `config.rs:14-32` is `SCHEMA_VERSION`/`deny_unknown_fields`/the
struct, `ci.sh:170-171` is gate 16/19. D-534 (725 ms median overshoot; the
precondition on arming the solver in a committed play config), D-197 (sealbot a
milestone, not a human-strength proxy), D-151 and D-175 (no balance filter, and
the arithmetic that keeps `random_openings_v1.txt` a fixture) all say what the
design says they say. **D-568 is not cited anywhere in the design** — see MAJOR 3.

---

# FINDINGS

## BLOCKING 1 — §6's dry-run limb 4 registers a criterion the shipped binary cannot satisfy, and a prior review already struck this exact criterion

**CLAIM.** §6 limb 4 reads *"the go line read back from the record is `go movetime
<ms>`, so the seat is in play mode."* No record the platform writes carries the go
line when the engine is the real `pistol` binary. The transcript records only
`raw = "bestmove <token>"` (`pistol_client.rs:262`, `transcript.rs:203`); the
report carries no budget-verb field (`report.rs:220-242`); and the shipped binary
writes nothing to stderr — the only `eprintln!` reachable from `pistol` is the
top-level refusal at `crates/pistol-cli/src/bin/pistol.rs:63`. The `saw go` lines
the suite reads back at `tests/run_tests.sh:355-362` are printed by
`tools/sealbot/tests/stub_pistol.py`, not by the engine.

This is not a new discovery. `docs/experiments/sealbot_anchor_v2_prereg.md` §5,
criterion F, records the strike verbatim: *"A second revision registered the seat's
own stderr `go` line, **which the STUB prints and the shipped binary does not**;
that is struck too, and this is what replaces both."* What replaced it is criterion
D (the mode pin fired from both directions against the real binary) and criterion F
(per-answer node counts vary and none of them is a node budget).

**SITES.** design §6 limb 4; `docs/experiments/sealbot_anchor_v2_prereg.md` §5
criterion F; `tools/sealbot/matchserver/src/pistol_client.rs:232` (`go` is sent and
never echoed), `:262` (`raw`); `tools/sealbot/matchserver/src/report.rs:220-242`;
`crates/pistol-cli/src/bin/pistol.rs:63`.

**FAILURE SCENARIO.** The dry run is executed against the real engines. The
operator opens `artifacts/<dryrun>/g001_engine_a.stderr` looking for the go line
and finds it empty. Limb 4 can be marked neither MET nor NOT MET. Either it is
quietly dropped — `docs/process.md`: *"Recording without a criterion is a dry run
nothing can fail"* — or the package stalls on a plumbing question that was answered
and written down one anchor ago.

**WHAT WOULD CLOSE IT.** Delete limb 4 and register in its place the two criteria
that survived that review for this defect class: the mode pin firing from both
sides against the real binary (a `movetime` seat at an instrument config and a
`nodes` seat at a play config, each a named handshake forfeit —
`pistol_client.rs:113-139`), and per-answer node counts that vary rather than
landing at a node budget. Both are already exercised end to end by the shipped
suite (`tests/run_tests.sh` m8/m9) and both were MET on the real binary in v2 §5.

---

## BLOCKING 2 — the opening is specified as `&[Coord]` "in play order" with no rule for deriving that order, and §3 pre-commits the referee to a panic when the derivation is wrong

**CLAIM.** The book's line is a full `position` tail —
`start moves 0,0 -4,3/-1,-1 0,-4/1,3` (`random_openings_v1.txt:62`) — and the
`a/b` pair token is CANONICAL, smaller cell first. pistol-core says in as many
words that this is **not** a play order: `Turn::first()`'s doc at
`crates/pistol-core/src/turn.rs:129-130` reads *"The cell that comes first in
canonical order — which is not a claim about play order"*, `Turn::from_str`
(`turn.rs:236-240`) refuses an uncanonical spelling, and D-6/D-52 construct pairs
legal in only one order. `GameState::make_pair` (`crates/pistol-core/src/play.rs:103-140`)
is the only thing that resolves the order: it tries canonical, then reversed.

The whole reason `pistol_client::play_order` exists (`pistol_client.rs:12-20`,
`:178-197`) and the reason `tools/sealbot/README.md` carries a section titled *"Why
the pistol client recovers play order"* is this exact hazard, against a referee that
*"applies submitted stones strictly in submitted order"*.

§3 says the referee takes *"a `&[Coord]` in play order"* and applies the stones
*"exactly as it applies engine stones today"* — i.e. `place` in submitted order —
and never says how the `&[Coord]` is derived from `a/b`. It then pre-commits: *"A
`place` that refuses is a **panic naming the opening and the stone**."*

**SITES.** design §3 bullets 1–2; `crates/pistol-core/src/turn.rs:129-130`,
`:236-240`; `crates/pistol-core/src/play.rs:103-140`;
`tools/sealbot/matchserver/src/pistol_client.rs:178-197`;
`tools/sealbot/matchserver/src/referee.rs:283-311`;
`crates/pistol-arena/src/openings.rs:249-257` (`replayed`, the correct pattern:
`make_turn` per opening turn).

**FAILURE SCENARIO.** An implementer decodes `0,-4/1,3` to `[Coord(0,-4), Coord(1,3)]`
because that is the order the file writes. On any book containing a D-52 pair — one
whose canonical cell is outside the union of radius-8 balls until the other cell
opens it — `place` refuses the first stone and the matchserver **panics mid-match**,
after some games have been written, on a legal opening. The same defect fires
earlier as a false refusal: §2's *"any stone is refused by `pistol-core`"* check
would reject a perfectly legal book line at config load.

**Reachability on the registered book, stated honestly**: `random_openings_v1.txt`
generates every stone inside `max_radius 5` **of the origin** (`:31`), the origin
stone is placed first, and `LEGAL_RADIUS` is 8 — so every later cell is legal by
radius whatever the order, the generator dedupes cells, and D-175's arithmetic
rules out a first-stone win. **On `book_v1` the naive decode is correct by
accident.** The design states none of that arithmetic, so a REVIEW-impl has nothing
to check the decode against, and `kind = "book"` accepts any file.

**WHAT WOULD CLOSE IT.** Specify the opening as `&[Turn]` applied through
`GameState::make_turn`, recording into `plies` the order pistol-core actually
played (`undo`-and-reverse, exactly as `pistol_client::play_order` already does) —
which is also what `pistol-arena::openings::replayed` does. Or state the decode
explicitly and record the arithmetic that makes canonical order safe for this book.
Either way, the pre-committed panic should become a config-load refusal, since the
design's own §2 claims the check happens there.

---

## BLOCKING 3 — §5's `replay_check` remedy removes the opening from the second instrument's coverage, and nothing else checks it either

**CLAIM.** The design's diagnosis is right and I verified it: `replay_one` places
`Coord::new(0,0)` unconditionally (`bin/replay_check.rs:92-96`) and `replay_turn`
then refuses on `turn != state.turn()` (`:184-189`). A v3 transcript's first turn
record says turn 4 while the replay's state is at turn 2, so **every v3 transcript
fails today**. The design names this as *"the limb most likely to be missed"*.

The remedy — *"It must read the opening from the transcript rather than assume the
origin"* — is where it stops, and that is where the instrument breaks. `replay_check`'s
own header says *"The stage under doubt is the RECORD"* (`:4-6`), and the README says
it *"checks what no in-run path can: that the bytes on disk are the game that was
played"*. Under the remedy as written, the opening is taken **from** the record, so
the opening is no longer under check. And `replay_check` takes exactly one argument,
an artifacts directory (`:36`); it globs `gNNN.jsonl` (`:44-59`) and never opens
`report.json`, so it has no access to the book path, the digest, the skip or the
take against which a recorded opening could be checked.

Nothing else covers it either. §6 limb 2 reads the same transcripts. §4's test is a
unit test over the pairing function (*"one test over all `2 * take` games asserting
each opening appears exactly twice"*), not over the run's record.

**SITES.** design §5 bullet 3 and §6 limb 2; `bin/replay_check.rs:36`, `:44-59`,
`:92-96`, `:184-189`; `main.rs:97-111` (report assembly, the only place the book
identity would live); `docs/process.md`, "Criterion and defect class" (*"An
externally derived referent, a value computed by something that does not share the
suspect input, is the operationalisation that reliably achieves this and is what a
reviewer looks for first"*).

**FAILURE SCENARIO.** An off-by-one in the window indexing (minor 1's shape, or a
`(g-1)/2` typed as `g/2`) makes every game play book line 0. All 100 transcripts
record opening 0. `replay_check` reads the opening from each transcript, replays it
cleanly, and prints `100 transcript(s) replayed to their recorded outcomes`, exit 0
— the design's own agreement criterion, MET. `distinct_games` still reads near 100
because both seats are under a clock. The report echoes `file`, `sha256`, `skip = 0`,
`take = 50` **from the config**, not from what was played. The ADR line then records
an anchor over "50 registered openings, each played both colours" that was one
opening played 100 times — the precise failure D-438 §10.1 had to recover by hand,
reintroduced with a fresh instrument standing behind it.

**WHAT WOULD CLOSE IT.** Register that a second reader cross-checks each
transcript's recorded opening against the book the report names, at the book line
the pairing rule fixes for that game number — the externally derived referent
`docs/process.md` asks for. That needs the report to carry the book identity in a
machine-readable field (which §5 already asks for, unspecified — see MAJOR 7) and
`replay_check` to read it, or a second small checker that does. It also needs §6's
dry run to state which of the four transcripts it checked this way.

---

## MAJOR 1 — `distinct_games` does not mean what §5 says it means, and under v3 it loses the ability to detect what it was added for

**CLAIM.** §5 says *"`distinct_games` unchanged in meaning. **Distinct games are now
expected to be many**"*. Two things are wrong with that.

(a) **The meaning changes.** The key is built from `summary.turns` only
(`report.rs:137-143`), and the opening never becomes a `TurnRecord` — the referee
seeds `plies` at `referee.rs:112` and pushes no record. Under v3 the key therefore
covers only the engine-played **suffix**, so `report.rs:135-136`'s own comment
("the stones of the game in submitted order, which is what makes two games the SAME
game") stops being true of the number it labels.

(b) **The number goes vacuous, and §5 reads that as the good news.** Under v2 the
number was diagnostic: `2 of 40` said both engines were deterministic from the
fixed opening, and D-438 plus v2 §10.1 built their honest reading on it. Under v3,
50 distinct openings force 50 distinct engine suffixes by construction, so the
report will read ≈`100 of 100` **whatever the engines do**. It can no longer
distinguish "the clock bought diversity" from "the openings differed", which is the
only question it was added to answer.

**SITES.** design §5 bullet 2; `report.rs:83-91` (the doc comment stating what the
number is for), `:131-143`, `:191`, `:292-295`; `referee.rs:112`; v2 prereg §2 and
§10.1; D-438.

**FAILURE SCENARIO.** The v3 report prints `Distinct games: 100 of 100 played.` The
ADR reads it as v2's number read: the interval's nominal N is now real. It is not —
the number is now guaranteed by the config's `take = 50` and says nothing about
either engine.

**WHAT WOULD CLOSE IT.** Strike "unchanged in meaning". Either fold the opening
stones into the key and say so, or — better, because it preserves the diagnostic —
register a v3-specific number: distinct engine suffixes **within each opening's
colour-pair**, which is what "did the clock buy diversity" now means, and which a
deterministic seat would make 1 for every pair.

---

## MAJOR 2 — the interval is taken over paired games as though they were independent, and the §2 definition v3 inherits says the opposite in as many words

**CLAIM.** `report.rs:188` computes `wilson_95(a_wins_decided, decided)` over all
decided games and `:253` prints the header `ANCHOR match (not SPRT, **not paired**,
not an Elo claim)`. §4 of the design says the v3 shape is *"the paired-balanced
shape CLAUDE.md rule 6 names"*. The definitions this anchor inherits are the v1
prereg's §2 — v2 §2 says *"Unchanged from `docs/experiments/sealbot_anchor_prereg.md`
§2, which owns these definitions"* — and that §2 justifies the game-level Wilson
with: *"the seats are reported separately **precisely because nothing here pairs
them**"*. v3 pairs them and registers no pair-level number, and does not flag that
the inherited definition now contradicts the run.

**SITES.** design §4 and §5 bullet 2; `report.rs:188`, `:253`, `:285-295`;
`docs/experiments/sealbot_anchor_prereg.md` §2 ("Interval"); v2 prereg §2 and §10.1.

**FAILURE SCENARIO.** v2's seat 1 measured 20–20 and its own §10.1 says the split
*"is the seat alternation and is not a measure of either engine"* — pistol converted
its p1 seat every time, sealbot converted its p1 seat every time. If P1 still
converts most book openings, v3 returns roughly 50–50 with a Wilson of about
[0.40, 0.60] *"over 100 decided games"*. A reader takes that as an even match on 100
independent samples. It is 50 opening-pairs whose per-pair score is close to
deterministic; the independent unit is 50, not 100, and the direction of the
error depends on the sign of the within-pair correlation — which nothing in the
report exposes.

**WHAT WOULD CLOSE IT.** Say what the interval's unit is. Register the per-pair
score distribution (how many pairs went 2–0 / 1–1 / 0–2) as the number pairing
actually buys — it is one fold over `summaries` and it is the thing that separates
"the engines are even" from "the first player wins". And either amend
`report.rs:253`'s "not paired" or state in §5 why it stands (it is defensible as a
claim about the *statistic*; it is not defensible unexplained beside §4).

---

## MAJOR 3 — the design draws a slice from a book D-505 forbids new pre-registrations from slicing, and cites neither D-505 nor the D-568 limb that licenses it

**CLAIM.** `crates/pistol-cli/src/random_openings/mod.rs:33-35` marks V1
*"RETIRED FOR GOVERNED USE (docs/decisions.md D-505)"*. D-505 (`docs/decisions.md:1078`):
*"`random_openings_v1.txt` holds 2000 openings and **every slice of it is consumed**
… so it is RETIRED FOR GOVERNED USE … **no new pre-registration may draw a slice
from it**."* The licence exists — D-568 (`:1204`) limb 5: *"**ANCHORS MAY USE
`book_v1`**, which is retired for SPRT and licensed here for anchor runs, because
an anchor makes no strength claim and so cannot launder a used opening set into
one."* The design cites D-534, D-197, D-151 and D-175, and neither D-505 nor D-568.

**SITES.** design §2's `kind = "book"` block and §7; `docs/decisions.md:1078`
(D-505), `:1204` (D-568 limb 5); `crates/pistol-cli/src/random_openings/mod.rs:33-35`.

**FAILURE SCENARIO.** The package's central choice reads as a breach on its own
face. A REVIEW-impl reviewer greps `random_openings_v1.txt`, lands on the RETIRED
comment in the generator, and either stops the package or spends the round
rediscovering D-568. Worse downstream: the anchor's report and ADR name a retired
book with no pointer to the line that permits it, and a later reader cannot tell
whether the run was licensed or slipped through.

**WHAT WOULD CLOSE IT.** Cite D-568 limb 5 beside the `kind = "book"` block, and
state the reason the licence gives — the slice is already spent for SPRT, which is
exactly why an anchor may use it — so a reader does not have to reconstruct it.

---

## MAJOR 4 — the reader this design specifies already exists, with strictly stronger semantics, and the design never names the alternative

**CLAIM.** `crates/pistol-arena/src/openings.rs::load(path, take, skip, turn_cap)`
is this reader. It verifies the fixture's in-band `# body_sha256`
(`:60-68`, D-147/D-148); refuses a blank line or a comment inside the body
(`:156-170`); refuses a line that is not `start moves …` (`:180-187`); replays every
opening through the rules so an illegal or already-decided opening is a rules
refusal (`:190-191`); refuses two openings equal up to a lattice symmetry
(`:200-222`, D-137, CLAUDE.md rule 6's distinct-n); refuses a file that mixes turn
counts (`:225-245`); refuses `skip + take > total` (`:98-108`); and refuses a
`turn_cap` that does not leave room for an engine move (`:86-95`). The design's §2
table reproduces two of those eight and substitutes a whole-file `sha256` in the
config for the in-band digest.

The reuse is genuinely not free, and I want that on the record: the matchserver is
a **detached** cargo workspace (`tools/sealbot/matchserver/Cargo.toml`'s empty
`[workspace]` table, with the comment saying so) depending only on `pistol-core`,
`serde`, `serde_json` and `toml`; `pistol-arena::openings` needs `pistol-arena` →
`pistol-engine` → `pistol-eval` + `pistol-search` + `pistol-solver`. That is a real
reason to reimplement. **The finding is that the design does not name the option at
all**, so nothing records which of those eight refusals were dropped deliberately
and which were simply not seen — and three of them are missing from §2 (turn_cap,
symmetry duplicates, uniform turn count), one of which is MAJOR 5 below.

CLAUDE.md's Process section: *"A named decision with more than one viable option is
settled by an OPTION MATRIX … attacked by a fresh-context DECISION-RED-TEAM
subagent BEFORE selection; an option adopted without a matrix … is the same breach
as silent architecture drift."* Reuse-vs-reimplement of a rules-adjacent reader is
such a decision.

**SITES.** design §2 (whole section); `crates/pistol-arena/src/openings.rs:44-121`,
`:156-245`; `tools/sealbot/matchserver/Cargo.toml`; `crates/pistol-engine/Cargo.toml`.

**FAILURE SCENARIO.** A REVIEW-impl reviewer cannot tell whether the missing
symmetry-duplicate refusal is a considered omission (book_v1 is already deduped by
generation — `# derived symmetry_collisions 0`) or an oversight, so it is either
waved through or re-litigated at IMPL, and a successor pointing `file` at any other
book inherits an unguarded 1-1 pair that doubles reported n.

**WHAT WOULD CLOSE IT.** An OPTION MATRIX line, or at minimum a §2 paragraph naming
`pistol-arena::openings::load`, why the matchserver declines the dependency, and
which of its eight refusals are reproduced and which are deliberately not.

---

## MAJOR 5 — a MISSING REFUSAL: the turn cap must leave room for engine turns, and `config.rs`'s existing cap guard becomes false under v3

**CLAIM.** `config.rs:85-87` refuses `turn_cap < 2` with the message *"turn_cap must
be at least 2 (the engines are first asked at turn 2)"*. Under a 5-stone book
opening the first ask is turn 4 — the design says so itself in §3 — so that guard
is both too weak and, in its stated reason, wrong. §2's refusal table does not add
the missing one. `pistol-arena` already has it, with the reason spelled out
(`openings.rs:86-95`: *"a cap of {turn_cap} ends each game before either engine has
moved; the cap counts from the start of the game"*).

**SITES.** design §2 refusal table and §3's turn-cap paragraph;
`tools/sealbot/matchserver/src/config.rs:85-87`;
`tools/sealbot/matchserver/src/referee.rs:161-164`;
`tools/sealbot/matchserver/src/bin/replay_check.rs:147-149`;
`crates/pistol-arena/src/openings.rs:86-95`.

**FAILURE SCENARIO.** A `local/*.toml` carries `turn_cap = 3` with `kind = "book"`
(a typo, or a dry-run cap copied down from v2's `turn_cap = 20` without noticing the
opening now costs three turns). Every refusal the design lists passes. `run_game`'s
loop breaks on its first iteration at `referee.rs:162` with `Capped { turn: 3 }`,
`turns` is empty, and 100 games are written in which no engine was ever asked
anything. Then the second instrument fails in the wrong vocabulary:
`replay_one` returns `Err("no turns recorded")` (`replay_check.rs:147-149`) → exit
1 → the registered agreement criterion is NOT MET, and the registered consequence
("that run is not a measurement") fires on what was a config typo, not a record
defect. `docs/SHELL_CHECKLIST.md` item 12's distinction — the answer is no versus
no answer was taken — is exactly what is lost.

**WHAT WOULD CLOSE IT.** Add the refusal to §2: `turn_cap` must exceed the opening's
turn count, refused before any process is spawned, and restate `config.rs:86`'s
message so it names the opening's turns rather than the literal 2.

---

## MAJOR 6 — §3's "an opening that already contains a win is refused at config load" is refused by nothing the design specifies, and its stated justification is about a different property

**CLAIM.** Two defects in one sentence.

(a) The check named in §2 is *"any opening line does not parse, or any stone is
refused by `pistol-core`"*. A **winning** stone is not refused: `GameState::place`
returns `Ok(PlyOutcome::Win { .. })` (`referee.rs:301`, `play.rs`'s outcome path).
So the refusal the design names cannot catch the case §3 says it catches. The
mechanism that does catch it is `PositionSpec::replay`
(`crates/pistol-engine/src/position.rs:68-73`), which refuses a won position by
name — and which the matchserver cannot call without MAJOR 4's dependency.

(b) The justification is the wrong property: *"`book_v1`'s own header records that
no position in it holds a **mate in one**"*. Mate-in-one is "the mover can win
immediately"; "already contains a win" is "a ≥6 run is on the board". The header's
arithmetic (`random_openings_v1.txt:19-28`, D-175) establishes the former. It
happens to imply the latter at k=5 (the largest holding is three), but the design
cites the claim that does not do the work.

**SITES.** design §2 refusal table row 5 and §3 bullet 4;
`tools/sealbot/matchserver/src/referee.rs:293-308`;
`crates/pistol-engine/src/position.rs:68-73`;
`crates/pistol-cli/tests/fixtures/random_openings_v1.txt:19-28`.

**FAILURE SCENARIO.** A successor points `file` at any other book — the licence
D-568 gives is for `book_v1`, but the config's `kind = "book"` is not restricted to
it — one of whose lines ends in a completed six. Every refusal passes at config
load. The referee applies the opening; the last `place` returns `Win`; the design's
§3 checks only `PlyOutcome::TurnComplete`, so it panics with the "did not complete
its turn" message, which names the wrong fault. If the check is loosened to accept
`Win`, the game is over before either engine is asked and the report records a win
by an engine that never moved.

**WHAT WOULD CLOSE IT.** Add an explicit refusal: the opening must leave the game
UNDECIDED and at a turn boundary, checked at config load by inspecting
`state.outcome()` and `state.phase()` after the replay — and drop the mate-in-one
citation, which supports a different sentence.

---

## MAJOR 7 — §5 fixes neither the transcript's opening encoding nor the report's opening fields, and the choice has a dependency consequence and a measurement consequence

**CLAIM.** §5 asks for *"the opening's own move list and its book line index"* in
place of `transcript.rs:25`'s sentence, and *"the book's path, digest, `skip` and
`take`"* in the report. No key names, no encoding, no example. A REVIEW-impl
reviewer told to check the code against this document cannot: there is nothing to
check against. Two consequences make it load-bearing rather than cosmetic.

(a) **Dependency.** `replay_check` depends on `pistol-core` only. It can consume
`[[q,r], …]` in play order (via `place`) or `start moves …` turn tokens (via
`Turn::from_str` + `make_turn`, both in `pistol-core`), but **not** a `PositionSpec`
(`crates/pistol-engine/src/position.rs`) without the matchserver taking a
`pistol-engine` dependency. The design's §2 calls the book's encoding *"the
`position` verb's own encoding"* without noticing that the type that parses it
lives in a crate the harness does not depend on.

(b) **Measurement.** The design does not say whether the opening stones become
`TurnRecord`s. They must not: `report.rs:144-150` builds engine A's per-answer wall
column by filtering `(turn.mover == Player::P1) == summary.a_is_p1`, so opening
turns recorded as turns would enter that column with `wall_ms = 0` and drag the
median and maximum — the one column a `movetime` anchor exists to produce (D-534,
D-95, v2 §10.1). §5 implies `game_start` only; it does not say it.

**SITES.** design §5; `tools/sealbot/matchserver/src/transcript.rs:22-27`;
`tools/sealbot/matchserver/src/report.rs:144-150`, `:220-242`;
`tools/sealbot/matchserver/src/bin/replay_check.rs:99-116`;
`tools/sealbot/matchserver/Cargo.toml`; `crates/pistol-engine/src/position.rs`.

**FAILURE SCENARIO.** (b) is the one that produces a wrong recorded number: an
implementation that records the opening as turns publishes an
`answer_wall_ms_median` computed over 100 zero-millisecond entries mixed with the
real answers. At 57 engine turns per game the opening is a small fraction, but the
median is the lower middle of a sorted vector (`report.rs:106-111`) and the column
is the anchor's headline overshoot measurement.

**WHAT WOULD CLOSE IT.** Fix the JSON keys and the encoding in §5 (I would say:
`"opening_moves": [[q,r], …]` in play order plus `"opening_line": <absolute book
line>`, which keeps `replay_check` on `pistol-core` alone), and add one sentence
saying the opening produces no `TurnRecord`.

---

# minor

**minor 1 — "opening index `skip + (g - 1) / 2`" is D-202's named consumer trap.**
`crates/pistol-arena/src/openings.rs:16-20` records it in as many words: a taken
window is indexed `0..take` and the absolute book position is `openings_skip +
index`, *"a consumer trap worth naming"*. The design's phrase reads naturally as a
Vec index into a `take`-long window. With the registered `skip = 0` the two
coincide, so this cannot bite the registered anchor; a successor anchor at
`skip = 50` indexes past the end. One word ("book line") fixes it.

**minor 2 — §3's stated reason for reading `state.to_move()` is false.** *"the
sealbot request is built from `plies`"* — `sealbot_client::request` maps only the
`Coord` out of each ply and drops the `Player` (`sealbot_client.rs:78-83`), and
`pistol_client::position_line` does the same (`pistol_client.rs:164`). **Nothing
reads the mover component of `plies` today.** The change is right; the justification
names a consumer that does not exist, and a reviewer sent to check it will find
nothing to check.

**minor 3 — §1's decisive fact is stated for the sealbot seat only.** The pistol
seat's corresponding fact is never stated and it is the answer to "does the
`position` verb's encoding match the book's line encoding": `position_line`
(`pistol_client.rs:157-174`) refuses an even-length ply list — a 5-stone opening is
odd, and D-175 refuses an even `k_stones` at generation for the same reason — and
re-chunks `plies[1..]` through `Turn::pair`, which canonicalises. So the pistol seat
receives **exactly the book's own line** whatever play order the referee chose,
and needs no change. That belongs in §1's table beside the sealbot row.

**minor 4 — §8 understates the suite change.** *"a new match in
`tools/sealbot/tests/run_tests.sh` with a book of two stub openings"* is not the
whole obligation: all nine existing configs are generated with `schema_version = 1`
and no `[openings]` block (`tests/run_tests.sh:90-140`), so the bump to 2 makes
every one of them refuse, and the assertion block (m1's turn-7 first-stone win, the
`movers == ["p2","p1", …]` check at `:285-287`, m3's "turn 2, p2's" at `:315-316`)
must be re-affirmed under `kind = "platform_standard"`.

**minor 5 — an unmarked estimate where a measurement exists (D-291).** *"The
dispatch budgets §2 at 'quiet box, ~1-2 h'. That is the RUN's cost and **it is
right**"* asserts a cost without marking it ESTIMATED, and the referent is
committed: v2 §10's table records **40 games at `movetime 500` with a match wall of
2 m 49 s**. 100 games of the same seat, starting three turns deep, is derivable
from those bytes and is nowhere near 1–2 h. The number is the prereg's to own, but
the design endorses it.

**minor 6 — §8's mutation obligation is stated as prose, which D-568 makes standing
law against.** D-568's fifth paragraph: *"a mutation set is specified against CALL
SITES ENUMERATED BY A `git grep` RECEIPT recorded in the mutation document, never
against prose, and a comment asserting a property about call sites is not evidence
of that property."* §8 names *"the pairing function, the digest check, the
refusals"*. The sites do not exist yet, so the receipt is owed at IMPL — but the
design should say so rather than leave the prose form standing.

---

# ATTEMPTED AND REJECTED

**R1 — "The sealbot shim will reject or misread a non-origin, multi-stone setup."**
REJECTED as a claim about this repository's code. `sealbot_shim.py:56-63` replays
`setup` and `moves` through the **same** `game.make_move` loop, so the
setup/moves split is immaterial to the shim: a 5-stone opening reaching it as
`setup = [[0,0]]`, `moves = [4 stones]` is applied exactly as five sequential
moves. **The design's §1 claim verifies against the code.** The residue is real and
the design owns it correctly: sealbot's own `HexGame.make_move` is an external tree
not in this repository (UNVERIFIED LIMB — I did not run it, and could not have
without loading the CPU), and the shim ignores its return value, so a stone
`HexGame` refused would be silently dropped and sealbot would answer from a
different position. §6 limb 3 exists to attribute exactly that and §6's STOP rule
is the right consequence. One line I would add rather than raise as a finding: the
referee already converts an illegal sealbot reply into a named forfeit
(`referee.rs:229-232`), so limb 3 is falsifiable as written.

**R2 — "`pistol-arena` is the cheaper route; run the anchor there."** REJECTED. The
arena seats engines through a `[budget]`-tagged pistol spec
(`crates/pistol-arena/src/config.rs:85-92`) and has no external-engine kind and no
JSON-lines client. Seating sealbot there is a larger change than the matchserver's,
and it would move the referee off the platform's judging policy (`referee.rs`'s
forfeits mirroring the server's `finishReason: illegal-move`, README "The platform's
game rules"). **The design's premise — that the matchserver is the thing to change —
survives attack.** What does not survive is the failure to name `pistol-arena`'s
*reader* (MAJOR 4), which is a different question from its *runner*.

**R3 — "The turn-cap arithmetic in §3 is wrong."** REJECTED — VERIFIED CORRECT.
`run_game` breaks at `state.turn() > turn_cap` (`referee.rs:161-164`), so the asked
turns are `first_ask ..= turn_cap`: 2..=60 is 59 asks under the platform opening,
and a 5-stone book opening is turn 1 (one stone) + turn 2 (two) + turn 3 (two) —
the book's own `# derived turn_structure p1@origin,p2,p2,p1,p1` — so the first ask
is turn 4 and 4..=60 is 57. **Both of §3's numbers check out.** A fact §3 should
also state and does not: the mover at turn 4 is P2, the same parity as the platform
opening's turn 2, so the seats rule and the `a_is_p1` expression are unaffected.

**R4 — "`games != 2 * take` is the wrong guard; an odd `games` should be allowed."**
REJECTED. `main.rs:74-75` runs `1..=games` with `a_is_p1 = game % 2 == 1`, so an odd
`games` leaves the last opening played once in one colour — exactly the shape rule
6's paired balanced openings forbid. The guard is correct and it refuses every odd
`games` for free (an odd number is never `2 * take`). The `platform_standard` form
keeps the pre-existing odd-`games` seat imbalance, which is out of this package's
scope and unchanged.

**R5 — "`take = 50` from `book_v1` is a short read the design's refusals miss."**
REJECTED. `skip + take > total` is listed and is the right guard; the file holds
2000 body lines (2061 total, header ends at `:61`), so `0 + 50` is comfortably
inside. Note in passing that D-175's own text says the book holds **500** openings
while D-505 and the file say 2000 — a stale ADR number, not a defect in this design.

**R6 — "The `sha256` requirement is ceremony, because the book is a tracked fixture
already pinned by a test."** REJECTED as stated, and re-raised in a narrower form as
MAJOR 6's sibling. `crates/pistol-cli/tests/random_openings_document_tests.rs:13-14`
pins `895a05ed…` for the file **at a path the matchserver never reads**, and the
config's `file` key can name any path, so a digest in the config is the only thing
tying the run to those bytes. The residual finding is not that the digest is
ceremony but that the design does not say **which** digest: the file carries an
in-band `# body_sha256 7b1b3a99…` at `:61` (D-147/D-148, the digest
`pistol-arena::openings` verifies) as well as the whole-file `895a05ed…`. The design
picked the one that does not also prove the file is a fixture — a hand-edited copy
with no `# body_sha256` line passes the design's check. `<the book's digest>` should
be resolved to one of them, and the in-band line should be verified as well.

**R7 — "The aggregate W/L will be meaningless because the first player wins."**
REJECTED. `report.rs:122-125`, `:202-210` and `:262-279` already split every tally
by seat, and v2 §10.1 read it exactly that way. The seat split is on the report's
face and the design does not need to add it. (The *interval* over the aggregate is
still MAJOR 2, which is a different complaint.)

**R8 — "`transcript.rs:25` is cosmetic; §5's first bullet is prose-polish."**
REJECTED. It is the only place the opening reaches the record, and `replay_check`
has no other source for it. §5 is right to call the current sentence a false record
under v3 — my complaint is that the remedy stops one step short (BLOCKING 3) and
leaves the encoding unfixed (MAJOR 7).

**R9 — "Pulling `pistol-engine` into the matchserver would trip the solver-link
gate."** REJECTED. `tools/solver_link_check.sh` enumerates *the workspace's*
binaries (`:122-127`, `:196-199`), and the matchserver is a detached workspace, so a
`pistol-solver` transitive dependency there would not be examined. Not a reason
against MAJOR 4's alternative — the dependency weight is.

---

# VERDICT

**FAIL — 3 BLOCKING, 7 MAJOR, 6 minor.**

The premise survives (the matchserver really does lack opening support; a
matchserver change really is the route), §1's decisive sealbot claim verifies
against the code, and §3's turn-cap arithmetic is correct. What fails is the
document's grip on its own instruments: a dry-run criterion the shipped binary
cannot satisfy and that a prior review already struck (B1), an opening decode whose
correctness is unspecified and pre-committed to a panic (B2), and a second
instrument whose repair removes the openings from its own coverage while nothing
else covers them (B3).
