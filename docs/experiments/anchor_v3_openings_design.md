# Anchor v3 — a registered opening book for the local match platform. DESIGN, revision 2.

> **ONE LINE.** Anchors v1 and v2 did not play a weak opening. **They played no
> opening at all** — game rule 3 makes turn 1 a single stone at the origin WLOG,
> so the "opening" every anchor to date started from is the RULE, identical in
> all 40 games. Anchor v2 measured what that costs: **DISTINCT GAMES 2 of 40**,
> on both seats. This design gives the matchserver a registered opening book so
> the next anchor's denominator is its N.

**REVISION 2, AND WHY.** Revision 1 returned **FAIL** from REVIEW-design — 3
BLOCKING, 7 MAJOR, 6 minor
(`docs/experiments/anchor_v3_openings_design_REVIEW.md`, revision
`963c8fddc2c2b4dc62de5ff9829ab8a527977457`). The premise survived attack: the
matchserver really does lack opening support, a matchserver change really is the
route, §1's sealbot claim verifies against the code, and the turn-cap arithmetic
is correct. What failed was the document's grip on its own instruments. Every
finding is disposed of below, each at the section that owns it. Revision 1 is
kept as `anchor_v3_openings_design_rev1_SUPERSEDED.md`.

---

## 0. THE PREMISE, RESTATED — WHAT AN ANCHOR WITHOUT OPENINGS MEASURES

**The origin is not a choice.** Game rule 3: *"Turn 1 = ONE stone (origin
WLOG)."* The matchserver plays that stone itself
(`referee.rs:17`, `const OPENING: Coord = Coord::new(0, 0)`) and both engines are
first asked at turn 2, from a board holding one stone that neither of them chose.

**AND THE CONSEQUENCE IS MEASURED, NOT FEARED.** Anchor v2, seat 1, 40 games at
`go movetime 500`: **DISTINCT GAMES 2 of 40** (`sealbot_anchor_v2_prereg.md`
§10). Seat 2 the same. That document's own reading: *"THE INTERVAL'S NOMINAL N IS
40 AND ITS REAL ONE IS 2 … a `movetime` budget did not buy diversity."* Two
distinct stone sequences, each replayed twenty times, because the movetime
ceiling lands answers at the budget and the completed-depth answer is stable — so
the seat is effectively deterministic from a fixed opening even under a clock.

**SO THE OPENINGS ARE NOT A REFINEMENT. They are the difference between an
interval over 40 games and an interval over 2.** A book opening is five stones
deep (`param k_stones 5`): turn 1's forced single plus two pair turns, which is
the shallowest depth at which either side has made a choice at all.

---

## 1. WHAT EXISTS, READ FROM THE CODE

| fact | where |
|---|---|
| the opening is one stone at the origin, hard-coded | `referee.rs:17` |
| the referee plays it itself and seeds `plies` with it | `referee.rs:106-112` |
| seats alternate by game parity | `main.rs:75`, `let a_is_p1 = game % 2 == 1` |
| the transcript records the opening as a fixed sentence | `transcript.rs:25` |
| **sealbot** is handed `setup` = the FIRST ply, `moves` = every later ply in play order | `sealbot_client.rs:69-83` |
| **pistol** gets `position_line`, which refuses an EVEN-length ply list and re-chunks `plies[1..]` through `Turn::pair`, canonicalising | `pistol_client.rs:157-174` |
| `pistol_client::play_order` exists precisely because canonical order is not play order | `pistol_client.rs:12-20`, `:178-197`; README *"Why the pistol client recovers play order"* |
| the config is explicit and complete, `deny_unknown_fields`, `SCHEMA_VERSION = 1` | `config.rs:14-32` |
| the suite drives the SHIPPED `run_match.sh` with stub engines; **nine configs**, all `schema_version = 1` | `tools/sealbot/tests/run_tests.sh:90-140` |
| CI gate 16 of 19 runs that suite | `tools/ci.sh:170-171` |

**TWO FACTS DECIDE THE SHAPE, ONE PER SEAT.**

- **sealbot needs no change.** `sealbot_client::request` splits an arbitrary ply
  list into `setup` (the first) and `moves` (the rest), and `sealbot_shim.py:56-63`
  replays both through the SAME `game.make_move` loop — so a five-stone opening
  reaching it as `setup=[[0,0]]`, `moves=[4 stones]` is applied as five
  sequential moves. Verified against the code by REVIEW-design (R1).
- **pistol needs no change either**, and revision 1 never said so (minor 3).
  `position_line` refuses an even-length ply list — a five-stone opening is odd,
  and D-175 refuses an even `k_stones` at generation for the same reason — and
  re-chunks through `Turn::pair`, which canonicalises. **The pistol seat therefore
  receives exactly the book's own line, whatever play order the referee chose.**

**NEITHER IS A CLAIM ABOUT SEALBOT ITSELF.** `HexGame.make_move` is an external
tree not in this repository, the shim ignores its return value, and a stone
`HexGame` refused would be silently dropped. §6 limb 3 attributes that, and it is
falsifiable: the referee already converts an illegal sealbot reply into a named
forfeit (`referee.rs:229-232`).

---

## 2. THE OPENING SOURCE — AN EXPLICIT, CLOSED CHOICE

Hard rule 1 forbids a code-side default, so the opening source is a required
block with a closed `kind`, and **`schema_version` becomes 2**:

```toml
[openings]
kind = "platform_standard"      # the origin cross: what every anchor before v3 ran
```

```toml
[openings]
kind = "book"
file = "crates/pistol-cli/tests/fixtures/random_openings_v1.txt"
skip = 0
take = 50
body_sha256 = "7b1b3a99…"       # the file's own in-band `# body_sha256` line
file_sha256 = "895a05ed…"       # the whole file
```

**BOTH DIGESTS, AND REVIEW-design's R6 IS WHY.** The in-band `# body_sha256`
(D-147/D-148) is what `pistol-arena::openings` verifies and what proves the body
is the fixture's; the whole-file digest is what ties the run to THESE bytes at
THIS path, since `file` may name any path and the committed test that pins
`895a05ed…` pins it at a path the matchserver never reads. A hand-edited copy
with no `# body_sha256` line passes a whole-file check alone, and a body-only
check passes a file whose header was rewritten. **Neither digest alone is the
check.**

### 2.1 THE READER'S HOME IS A MATRIX DECISION, NOT THIS DOCUMENT'S

`crates/pistol-arena/src/openings.rs::load` already is this reader, with **eight**
refusals, and revision 1 reproduced two of them and never named the alternative —
REVIEW-design's MAJOR 4, under CLAUDE.md's own OPTION MATRIX rule. The decision
is taken in `docs/experiments/matrix_anchor_openings_reader.md`, which enumerates
the eight from the code, and it is **attacked by a fresh-context DECISION-RED-TEAM
before selection**. This section states only what the matrix's §1 establishes and
what binds whichever option wins:

**THE MATRIX IS SETTLED AT REVISION 3 AFTER THREE RED-TEAM ROUNDS, AND ITS
FINDINGS BIND THIS SECTION.** Three of them change what IMPL owes:

- **There are SEVENTEEN refusal sites in `openings.rs`, not eight**, plus two
  scope rules (S1, comment stripping under D-143; S2, whole-file validation
  before the window is cut under D-202). §2.2's table below is the checklist a
  REVIEW-impl answers row by row; the matrix's §1 is where it is derived.
- **All seventeen are reachable without `pistol-engine` and without
  `pistol-cli`.** The selected option is **O2** — a minimal reader inside the
  matchserver — because O1 costs **+24 681 first-party lines (4 542 -> 29 223,
  MEASURED)** and none of the seventeen needs an engine, an eval or a search to
  decide whether an opening is admissible.
- **THE DIGEST IS VERIFIED BY `run_match.sh`, NOT INSIDE THE BINARY.** The
  matchserver has **no library target** and **no gate runs its Rust tests** —
  gate 3 is `cargo test --workspace` and this is a detached workspace; gate 16
  drives `run_match.sh`, which runs `cargo build` and never `cargo test`. A
  digest vendored into the binary would arrive with its FIPS pin unrunnable, so
  `run_match.sh` checks both digests with `sha256sum` **before it launches the
  binary**, where gate 16 exercises it and the refusal lands before any process.

**AND THE SAME STRUCTURAL FACT DECIDES HOW THIS PACKAGE IS TESTED AT ALL.** With
no lib target and no `cargo test` gate, **every obligation below is discharged
through `tools/sealbot/tests/run_tests.sh`**, which drives the SHIPPED
`run_match.sh` — which is what `docs/process.md`'s coverage rule asks for anyway.
The suite gains **one deliberately malformed book fixture per row of §2.2**, and
asserts the matchserver refuses each by name and at the right exit class. The
claim is one-directional and therefore true: *everything the arena refuses, the
matchserver refuses*. The reverse is false by design — §2.2 requires this reader
to refuse strictly more — and is not asserted.

### 2.2 THE EIGHT, AS THE CHECKLIST A REVIEW-impl ANSWERS ROW BY ROW

This table replaces revision 1's two-row refusal list. **A missing row is a
finding rather than a judgement call**, which is the whole point of writing it
out — revision 1's failure was that nobody could tell an omission from an
oversight.

| # | refusal | why it is a refusal and not a repair |
|---|---|---|
| R1 | the in-band `# body_sha256` disagrees with the body, or `file_sha256` disagrees with the file | §2 |
| R2 | a blank line or a comment inside the body | a body that is not all openings is a file whose count is not its take |
| R3 | a line that is not `start moves …` | the encoding is the `position` verb's tail and nothing else |
| R4 | an opening the RULES refuse | rule 2: the rules live in one place |
| R5 | **the opening does not leave the game UNDECIDED and AT A TURN BOUNDARY** | §2.3 |
| R6 | two openings equal up to a lattice symmetry | D-137, and rule 6's distinct-n: a symmetric pair doubles reported n |
| R7 | a file mixing turn counts | the turn cap arithmetic in §3 has one opening length in it |
| R8 | `skip + take > total`, or `take < 1` | a short read is a silently smaller sample |
| R9 | **`turn_cap` does not exceed the opening's turn count** | §2.4 |
| R10 | `games != 2 * take` on the book form | §4 |
| R11 | `kind = "platform_standard"` with any book key present | a document naming values nothing reads |

### 2.3 R5 REPLACES A REFUSAL REVISION 1 NAMED AND NOTHING PERFORMED

Revision 1 said *"an opening that already contains a win is refused at config
load"* and specified no mechanism that could do it. **A winning stone is not
refused**: `GameState::place` returns `Ok(PlyOutcome::Win { .. })`. The check is
therefore explicit — after replaying the opening, `state.outcome()` must not be
`Win` and `state.phase()` must be at a turn boundary — and it is made **at config
load**, before any process is spawned.

**AND THE MATE-IN-ONE CITATION IS DROPPED.** Revision 1 justified the refusal
with `book_v1`'s header arithmetic, which establishes that no position holds a
*mate in one* — a different property from *already contains a win*. It happens to
imply it at `k=5` (the largest holding is three), but citing a claim that does
not do the work is exactly the defect this package's sibling arc kept producing.

### 2.4 R9 REPLACES A GUARD THAT BECOMES FALSE

`config.rs:85-87` refuses `turn_cap < 2` saying *"the engines are first asked at
turn 2"*. Under a five-stone opening the first ask is turn **4**, so the guard is
too weak and its stated reason is wrong. R9 refuses a cap that does not exceed
the opening's turns, and the message names the opening's turns rather than the
literal 2.

**THE FAILURE THIS PREVENTS IS NOT A BAD MEASUREMENT, IT IS AN UNREADABLE ONE.**
With `turn_cap = 3` and a book opening, `run_game`'s loop breaks on its first
iteration, 100 games are written in which no engine was ever asked anything, and
`replay_check` then fails with `"no turns recorded"` — so the registered
agreement criterion reports NOT MET on what was a config typo. That is
`tools/SHELL_CHECKLIST.md` item 12's distinction lost: *the answer is no* versus
*no answer was taken*.

---

## 3. WHAT THE REFEREE DOES — AND THE OPENING IS `&[Turn]`, NOT `&[Coord]`

**REVISION 1'S `&[Coord]` "IN PLAY ORDER" WAS THE REVIEW'S BLOCKING 2 AND IT IS
GONE.** The book's `a/b` pair token is CANONICAL order, and `pistol-core` says so
in as many words: `Turn::first()`'s doc at `turn.rs:129-130` — *"The cell that
comes first in canonical order — which is not a claim about play order"*. D-52
constructs pairs legal in only one order. Revision 1 gave no rule for deriving
the order and then pre-committed the referee to a `panic!` when it was wrong.

**THE RULE, and it borrows the two implementations that already solve this:**

1. The opening is a `&[Turn]`, parsed by `Turn::from_str` (which refuses an
   uncanonical spelling).
2. Each turn is applied with **`GameState::make_turn`**, which tries the
   canonical order and then the reverse (`play.rs:103-140`, D-51/D-52) — so a
   pair legal in only one order is accepted whichever way the file writes it.
3. `plies` is then read off **`state.played()`**, which is the order pistol-core
   actually played. This is exactly what `pistol_client::play_order` does and
   what `pistol-arena::openings::replayed` does.

**THE PANIC BECOMES A CONFIG-LOAD REFUSAL** (R4), because §2 already says the
check happens there and a panic mid-match — after some games are written — is the
wrong shape for a fault in a document.

**ON `book_v1` THE NAIVE DECODE WOULD HAVE BEEN CORRECT BY ACCIDENT**, and the
arithmetic is recorded here because revision 1 relied on it without knowing:
every stone is generated inside `max_radius 5` of the origin (`:31`), the origin
stone is placed first, and `LEGAL_RADIUS` is 8 — so every later cell is legal by
radius whatever the order. **`kind = "book"` accepts any file**, so the decode is
specified rather than left to that.

**THE MOVER IS READ FROM `state.to_move()` BEFORE EACH PLACE** — but not for the
reason revision 1 gave. `sealbot_client::request` maps only the `Coord` out of
each ply and `pistol_client::position_line` does the same: **nothing reads the
mover component of `plies` today** (minor 2). It is recorded correctly because a
`plies` list whose movers are wrong is a false record, and because `state.played()`
supplies it for free.

**THE TURN CAP.** A five-stone opening is turn 1 (one stone) + turns 2 and 3 (two
each) — the book's own `# derived turn_structure p1@origin,p2,p2,p1,p1` — so the
first ask is turn **4**, and `4..=60` is **57 engine turns** against the platform
opening's `2..=60` = 59. Both numbers verified by REVIEW-design (R3). **And the
mover at turn 4 is P2, the same parity as the platform opening's turn 2**, so the
seats rule and the `a_is_p1` expression are unaffected.

---

## 4. THE PAIRING

Game `g` (1-based) plays **book line** `skip + (g - 1) / 2`, with
`a_is_p1 = g % 2 == 1`. Openings advance every two games, colours swap within
each pair: the paired-balanced shape CLAUDE.md rule 6 names, which the
platform-standard form cannot have because it has one opening.

**"BOOK LINE", NOT "INDEX"** (minor 1). `pistol-arena::openings.rs:16-20` records
the trap by name: a taken window is indexed `0..take` and the absolute book
position is `openings_skip + index`. With the registered `skip = 0` the two
coincide; a successor anchor at `skip = 50` indexes past the end.

**R10's GUARD IS `games == 2 * take`**, which refuses every odd `games` for free.
An odd `games` leaves the last opening played once in one colour — the shape rule
6 forbids. (The `platform_standard` form keeps the pre-existing odd-`games` seat
imbalance; out of scope and unchanged.)

One function computes both the opening index and `a_is_p1`, with one test over
all `2 * take` games asserting each book line appears exactly twice, once per
colour.

---

## 5. WHAT THE RECORD MUST CARRY — WITH KEYS AND AN ENCODING

Revision 1 asked for *"the opening's own move list and its book line index"* with
no keys, no encoding and no example, so a REVIEW-impl reviewer had nothing to
check the code against (MAJOR 7). Fixed here.

### 5.1 The transcript

`transcript.rs:25`'s fixed sentence is replaced by two keys:

```json
"opening_moves": [[0,0], [-4,3], [-1,-1], [0,-4], [1,3]],
"opening_line": 62
```

`opening_moves` is **play order** — what `state.played()` returned — as `[q,r]`
pairs. `opening_line` is the **absolute** book line.

**THE ENCODING IS CHOSEN FOR A DEPENDENCY REASON AND NOT FOR TASTE.**
`replay_check` depends on `pistol-core` only. It can consume `[[q,r], …]` through
`place`, or `start moves …` tokens through `Turn::from_str` + `make_turn` — but
**not** a `PositionSpec`, which lives in `pistol-engine`. Revision 1 called the
book's encoding *"the `position` verb's own encoding"* without noticing that the
type which parses it is in a crate the harness does not depend on.

### 5.2 THE OPENING PRODUCES NO `TurnRecord`, AND THAT IS A MEASUREMENT CLAUSE

`report.rs:144-150` builds engine A's per-answer wall column by filtering
`(turn.mover == Player::P1) == summary.a_is_p1`. **Opening stones recorded as
turns would enter that column with `wall_ms = 0`** and drag the median and the
maximum — the one column a `movetime` anchor exists to produce (D-534, D-95, v2
§10.1). The opening is `game_start` state, never a turn.

### 5.3 `distinct_games` GOES VACUOUS AND IS REPLACED RATHER THAN RE-LABELLED

`distinct_games` keys on engine turns only, so revision 1's *"unchanged in
meaning"* was false (MAJOR 1): under v3 every game starts from a different
opening, so the count is **100 of 100 by construction** and carries no
information. **The diagnostic v2 relied on it for does not disappear; it moves.**

The report gains **`distinct_openings_played`**, computed from the transcripts'
own `opening_line` values, and reported as *"50 of 50, each played twice"*.
`distinct_games` stays on the report with a stated expectation of 100, so a value
BELOW 100 is the finding it always was — two games from different openings that
produced identical stone sequences.

### 5.4 `replay_check` MUST BIND THE OPENING TO THE BOOK, WHICH IS BLOCKING 3

Revision 1's remedy — *"read the opening from the transcript"* — **removes the
opening from the second instrument's coverage**: `replay_check` takes only an
artifacts directory and never reads the config or the report, so nothing would
check that game `g`'s opening is book line `skip + (g-1)/2`. **A window
off-by-one gives 100 games on one opening, exit 0, and a report echoing `skip`
and `take` from the config.**

`replay_check` therefore gains a second argument — the book path with its two
digests — and asserts, per game: the transcript's `opening_line` is
`skip + (g-1)/2`; the book's line at that number parses to turns whose replay
yields exactly `opening_moves`; and across the run, each book line in the window
appears exactly twice with `a_is_p1` differing. **All three are `pistol-core`
work.** `skip` and `take` reach it from the report's instrument block, not from
the config it is checking.

---

## 6. THE DRY RUN

Input of the same kind, never the registered workload: **four games, two
openings, both colours**, at a reduced budget, against the real sealbot.

1. every game's transcript replays under `replay_check` **with the book
   argument**, exit 0 — which is §5.4's binding, exercised;
2. the four transcripts show **two distinct book lines, each twice, colours
   swapped**;
3. **sealbot answered from a book position at all** — its first reply is a legal
   turn, not a refusal or a hang. **The unverified limb**: the platform has never
   handed sealbot a non-origin setup, and a shim that cannot take one is a
   finding rather than a surprise at game 1 of 100. Falsifiable as written: the
   referee converts an illegal sealbot reply into a named forfeit;
4. **the seat is in play mode**, established by the MODE PIN and by node counts
   that vary across answers.

**LIMB 4 REPLACES A CRITERION THE SHIPPED BINARY CANNOT SATISFY** (BLOCKING 1).
Revision 1 registered *"the go line read back from the record is `go movetime
<ms>`"*. **No transcript, report or engine-stderr field carries the go line**;
only the stub prints `saw go`. The v2 pre-registration had already struck this
exact criterion and replaced it with the mode pin plus varying node counts —
which is what limb 4 now is.

A dry-run failure at limb 3 STOPS the anchor and is reported as a platform
finding; it is not worked around by changing the opening source.

---

## 7. WHAT THIS DESIGN DOES NOT DO

1. **No solver seat.** D-534's 725 ms median movetime overshoot blocks any
   play-config arming of the solver. v2 seat 2 measured a **1225 ms median** and
   an **1866 ms max** against a 500 ms budget; nothing here touches it.
2. **No new engine kind, no protocol change.** §1 records why, per seat.
3. **No strength claim, ever.** Sealbot is UNVERIFIED (D-197); the word is
   ANCHOR. **The INTERVAL is the pre-registration's clause and not this
   document's**, and it inherits v2 §2's definition — which justifies a
   game-level Wilson *"precisely because nothing here pairs them"*. **Under v3
   the games ARE paired**, two per opening with colours swapped, so
   `report.rs:253` printing "not paired" becomes false and the interval over the
   aggregate is no longer over independent trials. The pre-registration owns the
   fix; this design records that v3 creates the problem (MAJOR 2).
4. **No opening BALANCE filter.** `book_v1` has none by its own header's
   arithmetic; adding one would make the openings a judgement rather than a
   fixture (D-151, D-175).
5. **It slices `book_v1`, and the licence is named** (MAJOR 3). D-505 says no new
   pre-registration may slice it; **D-568's fifth limb licenses anchors to use
   `book_v1`** — retired for SPRT, and an anchor makes no strength claim, so it
   cannot launder a used opening set into one. Both are cited because revision 1
   cited neither.

---

## 8. THE OBLIGATIONS THIS PACKAGE OWES

| obligation | discharged by |
|---|---|
| the OPTION MATRIX for §2.1, attacked by a fresh-context DECISION-RED-TEAM **before selection** | `docs/experiments/matrix_anchor_openings_reader.md` |
| REVIEW-design | **round 1 FAIL** (3B/7M/6m); this revision answers it, and revision 2 owes its own |
| REVIEW-impl, fresh context, not the implementer | a subagent, against this document |
| RED-TEAM on the opening data path (empty line, short line, illegal stone, a stone outside radius 8, a duplicate stone, a half-played last turn, a D-52 reversed pair, a winning opening, a symmetric duplicate) | a subagent |
| the suite drives the SHIPPED `run_match.sh` — **and all NINE existing configs gain `[openings] kind = "platform_standard"` under `schema_version = 2`**, with m1's turn-7 first-stone win, the `movers == ["p2","p1", …]` check and m3's "turn 2, p2's" re-affirmed unchanged (minor 4) | `tools/sealbot/tests/run_tests.sh` |
| CI gate 16 green | `tools/ci.sh` |
| **a mutation set specified against call sites enumerated by a `git grep` receipt recorded in the mutation document** — the sites do not exist yet, so the receipt is owed AT IMPL and this line is not it (minor 6, D-568) | the IMPL package |
| an ADR line | at closure (hard rule 10) |

**THE RUN'S COST IS ESTIMATED FROM A MEASUREMENT AND REVISION 1 ENDORSED A GUESS**
(minor 5, D-291). The dispatch budgets §2 at *"quiet box, ~1-2 h"* and revision 1
said *"it is right"*. **v2 §10 measured 40 games at `movetime 500` with a match
wall of 2 m 49 s on seat 1.** Scaling by games alone gives **~7 minutes**
ESTIMATED for 100; book openings start three turns in, which shortens games
further, and diverse openings may lengthen them. **The 1-2 h figure is the
capability's cost, not the run's**, and the pre-registration owns the number.
