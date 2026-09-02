# Anchor v3 — a registered opening book for the local match platform. DESIGN, revision 1.

> **ONE LINE.** The overnight arc II dispatch's §2 asks for an anchor over **50
> registered openings from `book_v1`, each played both colours**. The
> matchserver plays exactly one opening — a single stone at the origin, a
> `const` in `referee.rs` — so the anchor cannot be run until the platform can
> be handed an opening. This is the design for that, and for nothing else.

**WHY THIS IS A PACKAGE AND NOT A LINE IN THE ANCHOR'S PRE-REGISTRATION.** The
dispatch budgets §2 at *"quiet box, ~1-2 h"*. That is the RUN's cost and it is
right; it is not the cost of the capability the run needs, which does not exist.
Recorded here rather than discovered at launch, and it changes nothing about
what §2 asks for.

---

## 1. WHAT EXISTS, READ FROM THE CODE

| fact | where |
|---|---|
| the opening is one stone at the origin, hard-coded | `tools/sealbot/matchserver/src/referee.rs:17`, `const OPENING: Coord = Coord::new(0, 0)` |
| the referee plays it itself and seeds `plies` with it | `referee.rs:106-112` |
| seats alternate by game parity | `main.rs:75`, `let a_is_p1 = game % 2 == 1` |
| the transcript records the opening as a fixed sentence | `transcript.rs:25` |
| sealbot is handed `setup` = the FIRST ply, `moves` = every later ply in play order | `sealbot_client.rs:69-80` |
| the config is explicit and complete, `deny_unknown_fields`, `SCHEMA_VERSION = 1` | `config.rs:14-32` |
| the suite drives the SHIPPED `run_match.sh` with stub engines | `tools/sealbot/tests/run_tests.sh` |
| CI gate 16 of 19 runs that suite | `tools/ci.sh:170-171` |

**THE ONE FACT THAT DECIDES THE SHAPE**: `sealbot_client::request` already
splits an arbitrary ply list into `setup` (the first) and `moves` (the rest). A
five-stone opening therefore needs **no change to the shim's contract** — sealbot
sees the opening's later stones as ordinary play, which is what they are. That is
a reading of the code and **NOT a claim about sealbot's behaviour**: whether
sealbot answers sensibly from a book position is UNVERIFIED and is what the dry
run in §6 exists to attribute.

---

## 2. THE OPENING SOURCE IS AN EXPLICIT, CLOSED CHOICE

Hard rule 1 forbids a code-side default for any tunable, so the opening source is
not an optional key whose absence means the origin. It is a required block with a
closed `kind`:

```toml
[openings]
# The platform's own setup: p1's turn-1 stone at the origin, exactly as the
# htttx `setup` packet delivers it. What every anchor before v3 ran.
kind = "platform_standard"
```

```toml
[openings]
# A slice of a committed opening book, in the `position` verb's own encoding.
kind = "book"
file = "crates/pistol-cli/tests/fixtures/random_openings_v1.txt"
skip = 0
take = 50
sha256 = "<the book's digest>"
```

**`schema_version` becomes 2.** Every existing match config gains the block; the
committed `local/sealbot.example.toml` shows both spellings. A version bump
rather than an optional key, because a document that runs a different match
depending on a key it does not have is the silent-fallback hard rule 3 forbids.

**`sha256` IS REQUIRED ON THE BOOK FORM AND IS CHECKED BEFORE GAME ONE.** The
book is an input to a recorded number; an input named without its digest is an
input that can change under a run (`docs/process.md`, instrument governing
revision). A mismatch is a refusal, never a warning.

**REFUSALS, each by name and each before any process is spawned:**

| condition | why it is a refusal and not a repair |
|---|---|
| `kind = "book"` and `games != 2 * take` | the anchor's whole shape is each opening played BOTH colours; a games count that is not twice the take plays some opening once, and which one depends on arithmetic nobody wrote down |
| `skip + take` exceeds the book's line count | a short read is a silently smaller sample |
| `take < 1` | a book form that takes nothing is the platform form spelled obscurely |
| the digest disagrees | above |
| any opening line does not parse, or any stone is refused by `pistol-core` | rule 2: the rules live in one place, and an opening the rules reject is not an opening |
| `kind = "platform_standard"` with any of `file`/`skip`/`take`/`sha256` present | a document naming values nothing reads |

---

## 3. WHAT THE REFEREE DOES

`run_game` takes the opening as a parameter — a `&[Coord]` in play order, the
stones the server plays before either engine is asked — and applies them exactly
as it applies engine stones today:

- `state.to_move()` is read BEFORE each `place`, so `plies` carries the true
  mover of every opening stone. The current code hard-codes `Player::P1` because
  there is only ever one opening stone; with an opening of five that would be
  wrong for four of them, and the sealbot request is built from `plies`.
- A `place` that refuses is a **panic naming the opening and the stone**, not a
  forfeit: a forfeit attributes to an engine a fault that is the harness's, and
  the config check in §2 means an unparseable opening never reaches here.
- The last opening stone must complete its turn (`PlyOutcome::TurnComplete`). An
  opening that leaves a turn half-played would have the first engine asked for
  the wrong number of stones. Checked, and a panic if not.
- An opening that already contains a win is refused at config load, not here:
  `book_v1`'s own header records that no position in it holds a mate in one, and
  a book that did would make a game the engines never played.

**THE TURN CAP COUNTS TURNS AND THE OPENING SPENDS SOME.** `book_v1`'s openings
are five stones — turn 1's single plus two pair turns — so an engine's first ask
is turn 4 rather than turn 2, and a `turn_cap` of 60 buys 57 engine turns rather
than 59. This is registered in the anchor's pre-registration as a number, not
left for a reader to derive.

---

## 4. THE PAIRING, WRITTEN DOWN

Game `g` (1-based) plays opening index `skip + (g - 1) / 2`, with
`a_is_p1 = g % 2 == 1`. So openings advance every two games and the colours swap
within each pair — the paired-balanced shape CLAUDE.md rule 6 names, which the
platform-standard form cannot have because it has one opening.

**This is the only thing about seat alternation that changes**: `a_is_p1` keeps
its present expression, and the opening index is the new one. A successor
changing either must change both, so they are computed in one function with one
test over all `2 * take` games asserting each opening appears exactly twice, once
per colour.

---

## 5. WHAT THE RECORD MUST CARRY

- **the transcript**: the opening's own move list and its book line index, in
  place of `transcript.rs:25`'s fixed sentence. A transcript that says
  *"server: p1 turn 1 at 0,0"* for a five-stone opening is a false record, and
  `replay_check` — the second instrument — must be able to replay from it.
- **the report**: the book's path, digest, `skip` and `take`; and
  `distinct_games` unchanged in meaning. **Distinct games are now expected to be
  many**, where anchor v2's fixed opening made two the honest denominator.
- **`replay_check`**: it replays transcripts against `pistol-core`. It must read
  the opening from the transcript rather than assume the origin, or every v3
  transcript fails to replay. **This is the limb most likely to be missed**, and
  it is named here so a reviewer can check it was not.

---

## 6. THE DRY RUN, AND WHAT IT ESTABLISHES

Input of the same kind, never the registered workload: **four games, two
openings, both colours**, at a reduced budget, against the real sealbot. It
establishes ATTRIBUTION and nothing else:

1. every game's transcript replays under `replay_check`, exit 0;
2. the four transcripts show **two distinct openings, each twice, colours
   swapped**;
3. sealbot answered from a book position at all — its first reply is a legal
   turn, not a refusal or a hang. **This is the unverified limb**: the platform
   has never handed sealbot a non-origin setup, and a shim that cannot take one
   is a finding rather than a surprise at game 1 of 100.
4. the go line read back from the record is `go movetime <ms>`, so the seat is
   in play mode.

A dry-run failure at limb 3 STOPS the anchor and is reported as a platform
finding; it is not worked around by changing the opening source.

---

## 7. WHAT THIS DESIGN DOES NOT DO

1. **No solver seat.** D-534's 725 ms median movetime overshoot blocks any
   play-config arming of the solver, and this package does not touch it.
2. **No new engine kind, no protocol change.** The shim's contract is unchanged;
   §1 records why.
3. **No strength claim, ever.** Sealbot is UNVERIFIED (D-197). The word is
   ANCHOR, and the pre-registration owns that clause rather than this design.
4. **No opening BALANCE filter.** `book_v1` has none by its own header's
   arithmetic, and adding one here would make the openings a judgement rather
   than a fixture (D-151, D-175).

---

## 8. THE OBLIGATIONS THIS PACKAGE OWES

| obligation | discharged by |
|---|---|
| REVIEW-design, fresh context, attacks the premise | a subagent, before IMPL |
| REVIEW-impl, fresh context, not the implementer | a subagent, against this document |
| RED-TEAM on the data path (adversarial openings: empty line, short line, an illegal stone, a stone outside radius 8, a duplicate stone, a line whose last turn is half-played) | a subagent |
| the suite drives the SHIPPED `run_match.sh` (`tools/SHELL_CHECKLIST.md` item 10) | a new match in `tools/sealbot/tests/run_tests.sh` with a book of two stub openings |
| CI gate 16 green | `tools/ci.sh` |
| mutation receipts at the sites this package adds | the pairing function, the digest check, the refusals |
| an ADR line | at closure (hard rule 10) |
