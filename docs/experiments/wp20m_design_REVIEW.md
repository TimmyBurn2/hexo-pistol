# REVIEW-design — `docs/experiments/wp20m_design.md`

## Header

- **Named revision adjudicated:** `5064b0563775c013bc0c797ee8092c3b891625f9`
  (`docs(wp20m): the mechanism design carries no meaning …`).
- **Matches HEAD:** **yes.** `git rev-parse HEAD` →
  `5064b0563775c013bc0c797ee8092c3b891625f9`, branch `dev`.
- **Tree state:** `git status --porcelain` → **empty**. Clean.
- **What I ran:** `git`, `/usr/bin/grep`, `git grep`, `sed`, `wc`, `cat`. Every
  recorded grep is `/usr/bin/grep` or `git grep` (D-265).
- **What I refused to run:** `cargo` in any form and `tools/ci.sh`, per the
  dispatch. Where a claim needs a run I name it rather than assert it.
- **Read as binding:** CLAUDE.md, `docs/process.md`, `wp20_DESIGN_STOP_SPLIT.md`,
  `wp20_design.md`, both prior reviews, `matrix_wp20_shape_selection.md`,
  `wp20_dispatches.md`, `wp20_premise_memo.md`, D-6/80/88/200/291/424/477/481/
  483/500/511/527/535–544.
- **Code read at this revision:** `pistol-arena` `channel.rs`, `exchange.rs`,
  `seats.rs`, `replay.rs`, `transcript.rs`, `report.rs`, `schedule.rs`,
  `validate.rs`, `config.rs`, `bin/arena.rs`, `bin/stub_engine.rs`;
  `pistol-cli` `protocol.rs`, `report.rs`, `corpus/emit.rs`; `pistol-engine`
  `position.rs`, `position_token.rs`, `instance.rs`, `engine.rs`;
  `tools/ci.sh`, `tools/determinism.sh`, `tools/file_justification_check.sh`,
  root `Cargo.toml`.

---

## VERDICT: **FAIL**

3 BLOCKING, 11 MAJOR, 6 MINOR.

Two of the three BLOCKING findings are not schema quibbles imported from the
dead package. **One of them is a contradiction between two of this document's
own seven invariants**, and it is falsified by an instrument this repository
already ships and already runs as CI gate 9. The other is the dispatch's
question 1 answered against the design.

---

## Question 1, answered directly: **IS THE CAPTURE MEANING-FREE?**

## **NO.**

The premise — *"a capture that adds no meaning cannot be wrong about meaning"*
(`wp20m_design.md:11`) — is true as a sentence and false as a description of
this design. The design makes **at least six** decisions that are decisions
about meaning, and it declares none of them:

| # | the decision | where it is smuggled | why it is meaning |
|---|---|---|---|
| 1 | one record per **turn**, not per **ply** | `:48-50` asserts it as a fact about the domain | premise memo decision 11 (`wp20_premise_memo.md:1520-1526`) names it as open; D-477 exists because an unstated unit axis cost two red-team rounds |
| 2 | whether the **pre-turn-1** position is a to-move position | nowhere | `position_line` **cannot spell it** (see BLOCKING 2b) |
| 3 | whether the **terminal position of a won game** is captured | nowhere | the engine **refuses** it by name (BLOCKING 2c) — and skipping it is "an exclusion by outcome", which INVARIANT 4 forbids |
| 4 | which fields of the totals line are **content** and which are **volatile** | forced by INVARIANT 3 against INVARIANT 5 | `nps`/`time` are wall-clock; deciding they are "the same answer" is exactly a claim about what a field means (BLOCKING 1) |
| 5 | **which engine** produces a label | `:32` "on one channel", never says which | review-1 MAJOR 5, never applied (MAJOR 2) |
| 6 | what "**the position**" column *is* | `:33-34` | a move-list prefix, a `position` line, or an index pair are three different columns (MAJOR 4) |

Decisions 2, 3, 5 and 6 are not interpretation questions that WP-2.0-S could
take later. **They are capture questions**, and they are questions this design
had to answer and did not.

**The split's own premise does not survive its own record.** `D-544` and
`wp20_DESIGN_STOP_SPLIT.md:53-55` both state: *"Every failure in both reviews
was an INTERPRETATION question and none was a CAPTURE question."* That is
false on the reviews' own text. Review 1's **MAJOR 5** (*"the design never says
which seat answers a label ask, and never refuses a report whose two seats are
different engines"*, `wp20_design_REVIEW.md:457`) is a capture question; its
rev2 disposition is **NOT APPLIED** (`wp20_design_REVIEW_rev2.md:260-265`); and
it is **still not applied here** — `/usr/bin/grep -n "seat"
docs/experiments/wp20m_design.md` returns exactly one line, `:89`, about table
size. The same holds for review-1 MAJOR 7 (budget kind), MINOR 6 (pass-2
failure modes), MINOR 7 (CLI grammar) and MINOR 9 (ply/turn) — five capture
findings, all NOT APPLIED at rev2, all still open at this revision.

So the split line was drawn on a claim that is not true, and **the findings it
left on the mechanism's side of the line travelled into this package
unfixed**. That is the structural finding behind most of what follows.

---

## Requirement coverage — the dispatch (`wp20_dispatches.md:79-86`)

| req | text | disposition |
|---|---|---|
| **1** | plays self-play games; GAME and LABEL budgets separate registered values | **DELIVERED HERE** as a mechanism (§1, pass 1 unmodified). Value correctly deferred to the pilot prereg (D-483). **But the label budget's KIND is constrained nowhere** — MAJOR 8 |
| **2** | one record per position: canonical move list, **side to move**, label (score+move+depth+nodes), **game outcome**; a **documented, versioned schema with a loader test** | **SPLIT, AND IT LEAKS.** The label bytes are delivered here. *Side to move* is derivable from the move list. **Game outcome FALLS BETWEEN** — MAJOR 10. **The versioned schema and loader test FALL BETWEEN** — MAJOR 4 |
| **3** | census logging | correctly **out of both packages** (D-539 → WP-2.0b). Clean |
| **4** | deterministic end-to-end; a re-run receipt proves byte-identical output | **CLAIMED HERE (INVARIANT 5) AND FALSE** — BLOCKING 1 |
| **5** | ledgers: book ranges consumed, **corpus manifest with digests**, census corpus manifest | **FALLS BETWEEN BOTH PACKAGES.** `/usr/bin/grep -ci "ledger\|manifest" docs/experiments/wp20m_design.md` → **0** and **0**; `wp20_DESIGN_STOP_SPLIT.md` §3's "what is owed to WP-2.0-S" does not name it either — MAJOR 9 |

**Falls between, named:** requirement 5 entirely; requirement 2's *game
outcome* and its *versioned schema + loader test*.

---

# FINDINGS

## BLOCKING

### BLOCKING 1 — INVARIANT 5 contradicts INVARIANT 3, and CI gate 9 is the proof

`wp20m_design.md:116-118` (INVARIANT 3) — *"The captured totals and bestmove
lines are **byte-identical** to what the engine wrote … No field is reordered,
renamed, dropped or combined."*

`wp20m_design.md:121-122` (INVARIANT 5) — *"A re-run of pass 2 over one report
at one label budget produces a **byte-identical capture file**."*

**These cannot both hold.** The totals line carries two wall-clock fields:

```
crates/pistol-cli/src/report.rs:83-84
"{INFO_PREFIX}{marker} depth_turns {} seldepth {} nodes {}{solver_field} {NPS_FIELD} {} \
 {TIME_FIELD} {} hashfull {} score {} pv",
```

sourced from `SearchInfo::nps` and `SearchInfo::time_ms`
(`crates/pistol-search/src/info.rs:147,149`), whose own doc calls `nps` *"a
measurement of the machine, not of the search"* (`pistol-cli/src/report.rs:15`).

This project has already ruled on those two fields, and mechanized the ruling:

```
tools/determinism.sh:153-154
# `nps` and `time` are the only fields two runs may disagree about.
normalize() { sed -E 's/ nps [0-9]+ time [0-9]+//'; }
```

`tools/determinism.sh` is CI **gate 9/19** (`tools/ci.sh:104-105`) and is
CLAUDE.md's own hard-rule-4 gate. It closes with *"no difference outside
nps/time"* (`determinism.sh:289`). **The one instrument this repository trusts
to say two runs agree deletes exactly the bytes INVARIANT 3 requires the
capture to keep.**

**SECOND LIMB, and it is the worse one: the registered test passes vacuously.**
§7 registers `a_rerun_over_one_report_is_byte_identical`. Every test in
`pistol-arena` drives `bin/stub_engine.rs`, and the stub hardcodes its totals:

```
crates/pistol-arena/src/bin/stub_engine.rs:126-127
nps: 1,
time_ms: 0,
```

So the registered test is **green under the only engine the arena test suite
has** while the invariant is false under the binary the pilot will actually
run. `docs/process.md`'s "Criterion and defect class" names this exactly: *"A
criterion that is a property the named defect class PRESERVES … passes
vacuously and is not a criterion."* This is the D-527 class again — a check
that passed on the bands a defect cannot move.

**FIX (and it forces a meaning decision, which is the point).** The design must
choose and state one of:
(a) INVARIANT 5 is weakened to *byte-identical after `determinism.sh`'s own
normalisation*, the normalisation named as a mechanism in the design, and the
test re-registered as `a_rerun_over_one_report_is_byte_identical_outside_nps_and_time`
**driving the real `pistol` binary**, not the stub; or
(b) the capture writes the totals line with `nps`/`time` normalised out — which
drops two fields and so **contradicts INVARIANT 3**, and must say so.
Either way, this package has decided that `nps` and `time` do not carry answer
content. That is a meaning decision, and it belongs on the document's face.

---

### BLOCKING 2 — "every to-move position" is undefined, and all three of its boundaries break against the code

INVARIANT 4 (`:119-120`) — *"**Every to-move position of every game is
captured**, with no exclusion by book, forfeit, outcome or turn."*
§2 (`:48-50`) — *"**Every to-move position is a PREFIX of a recorded move
list**, which is what `position_line` sends — so pass 2 truncates and does not
reconstruct."*

The second sentence is presented as a fact about the domain. It is a
**restatement of one of two live readings**, and it is false under the other.

**(a) Per PLY or per TURN — memo decision 11, never taken.**

```
docs/experiments/wp20_premise_memo.md:1520-1526
11. **Whether one record per position means one per PLY or one per TURN.** …
    `PositionSpec::Start` *"always names a position at a turn boundary"*
    (`position.rs:9-10`) — but `PositionSpec::Set` can express a mid-turn
    position (`position.rs:37-46`, D-6) … This is D-477's own unit question
    and the axis the record is indexed on.
```

Verified at this revision: `crates/pistol-engine/src/position.rs:8-14` —
`Start` *"always names a position at a turn boundary"*; `:37-46` — `Set`
carries `to_move` **and `phase`**, and `phase` is how a mid-turn position is
stated. After the first stone of a two-stone turn the same player **is to
move**; that is a to-move position by any reading of the words, the engine can
hold it, and `exchange::position_line` (`exchange.rs:154-161`) **cannot
express it**. So under the per-ply reading INVARIANT 4 is simply false, and
under the per-turn reading it is true only because the design silently narrowed
"to-move position". Review-1 MINOR 9 asked for one sentence citing the
consumption site; rev2 disposition **NOT APPLIED**
(`wp20_design_REVIEW_rev2.md:331`); still not applied here.

**(b) The empty prefix is not expressible by the sender the design names.**
Every game has a to-move position before turn 1 (P1 to move, rule 3). The
design says pass 2 sends what `position_line` sends. At zero moves:

```
crates/pistol-arena/src/exchange.rs:154-161
let mut line = format!("{} start moves", pistol_cli::protocol::POSITION);
```

→ the literal `position start moves`, which the engine **refuses**:

```
crates/pistol-engine/src/position_token.rs:87-89
Some((&MOVES_KEYWORD, [])) => {
    return Err(format!("`{MOVES_KEYWORD}` with no turns after it"));
}
```

The correct spelling of the initial position is bare `position start`
(`position_token.rs:37`). This never fires on the SPRT path because
`replay::walk` only asks at `at >= opening_turns` (`replay.rs:138`) and the book
supplies the prefix — but INVARIANT 4 removes exactly that guard by capturing
book turns. So either the design's "truncates and does not reconstruct" claim is
false at k=0, or the initial position is excluded — an exclusion INVARIANT 4
forbids. **The design says neither.**

**(c) The terminal position of a won game — a `go` on a decided position, which
the dispatch asked about explicitly.**

`transcript::replays` **permits** the last recorded turn to win; it refuses only
turns *after* one (`transcript.rs:369-376`). So the full move list of a won game
reaches a decided position, and:

```
crates/pistol-engine/src/position.rs:68-73
if let Outcome::Win { winner, turn } = state.outcome() {
    return Err(EngineError::illegal_position(format!(
        "{winner} completed a line on turn {turn}: a won position is terminal, so \
         there is no move to ask this engine for (rule 4)"
    )));
}
```

**Answer to the dispatch's question 3: yes, a naive enumeration of prefixes
`0..=len` asks a `go` on a decided position, and the engine answers `error
IllegalPosition: …` and never a `bestmove`.** §1's *"read to `bestmove`"*
(`:33`) has no arm for that (MAJOR 1), so pass 2 would block on
`Channel::receive` until `hang_timeout_ms` and then raise `ArenaError::Hung`
(`channel.rs:166-171`) — **a wall-clock-shaped failure on a deterministic
input**, on the main line of every won game in the corpus.

And the correct behaviour — skip it — is *"an exclusion by … outcome"*, which
INVARIANT 4's own words forbid. The invariant is self-contradictory as written.

**FIX.** One section defining a to-move position, with D-477's rule obeyed: the
unit quoted at the line where it is **consumed** (`replay.rs:137-138` for the
turn axis, `position.rs:9-10` for why `Start` cannot carry a ply). Then state
the prefix range explicitly — for game `g` with `len` recorded turns, prefixes
`k ∈ [0, len]`, **less `k = len` when the last recorded turn wins** — say that
`k = 0` is spelled bare `position start` and not through `position_line`, and
re-word INVARIANT 4 so its "no exclusion" clause does not contradict the one
exclusion the rules force. Register
`the_terminal_position_of_a_won_game_is_not_asked_for` and
`the_initial_position_of_every_game_is_captured`.

---

### BLOCKING 3 — the document's central claim is stated as a general law and used as a licence, and it is false in this package

`:7-13` and `:11` — *"**So this package interprets nothing** … **A capture that
adds no meaning cannot be wrong about meaning**"*; `:157-158` — *"this package
is **deliberately unable to get them wrong**."*

BLOCKING 1 and BLOCKING 2 are both counter-examples, and they are not marginal:
one is a contradiction between two invariants, the other is on the main line of
every game. The claim is doing real work in this document — it is the stated
reason the design is short, the reason §8's deferral list is treated as
exhaustive, and the reason no option matrix was taken for any of the six
decisions tabulated above. **A licence that is false is worse than an absent
one**, because it suppresses the attack rather than answering it.

This is not a prose complaint reachable by D-424's overrule: D-424 reaches
*"prose that constrains nothing"*, and this prose constrains what the document
had to decide. Nor is it a correctness-free finding — BLOCKING 1 and 2 each name
a way the package produces a wrong answer (a capture that is not reproducible;
a pass that hangs), and D-424's last clause says a finding of that shape *"is
never overruled, only fixed"*.

**FIX.** Replace the universal claim with the true and still-sufficient one:
*this package makes no decision about what the score, the node counts or the
provenance MEAN; it does make, and states here, the decisions about WHICH
positions are asked, WHICH engine answers, WHAT counts as the same capture, and
WHAT happens when an ask fails.* Then take those four decisions on the face of
the document. The package survives; only the slogan does not.

---

## MAJOR

### MAJOR 1 — pass 2's failure modes are undesigned, and BLOCKING 2 puts them on the main line

`:32-33` — *"send `newgame`, send the position, send a `go` at the label budget,
and **read to `bestmove`**."* There is no arm for anything else, and the channel
has three other outcomes plus the engine's own refusal line:

- `Received::Closed` (`channel.rs:31-33`) — the engine exited;
- `Received::Overlong` (`channel.rs:28-30`) — over `MAX_LINE_BYTES`, and
  **deliberately not a line**;
- `ArenaError::Hung` (`channel.rs:166-171`) — the watchdog;
- `error <NamedError>: <why>` (`pistol-cli/src/report.rs:134-137`), which the
  engine writes and stays alive (D-5) — so `bestmove` **never arrives**.

`exchange::ask` classifies all of these as **forfeits** (`exchange.rs:46-86`),
and "forfeit" means nothing in a pass that plays no game. Hard rule 3 requires a
named refusal. Review-1 MINOR 6 asked for one paragraph; rev2 **NOT APPLIED**
(`wp20_design_REVIEW_rev2.md:328`); not applied here — `/usr/bin/grep -c -i
abort docs/experiments/wp20m_design.md` → **0**.

**FIX.** One paragraph: a label ask that does not reach `bestmove` **aborts the
pass by name** with the position it happened on, and no capture file survives a
partial pass (`replay::run`'s own reasoning at `replay.rs:16-19` — *"a criterion
over SOME of a report's games is a criterion over a sample nobody registered"* —
applies verbatim). Register `a_label_ask_that_does_not_answer_aborts_by_name`.

### MAJOR 2 — which seat answers a label ask, and no refusal of a two-engine report

Review-1 MAJOR 5, **NOT APPLIED** twice and now three times. §1 says *"on one
channel"* (`:32`) and never which. Nothing makes a source report self-play from
pass 2's side:

- `crates/pistol-arena/src/validate.rs:242-250` refuses only **identical
  labels**;
- `crates/pistol-arena/src/transcript.rs:189-194` likewise refuses only
  identical labels.

So an ordinary A-vs-B SPRT report reads cleanly, and pass 2 would produce a
corpus whose labels come from one of two different teachers — while
`capture_sha256` (§3) attests **both**. Rule 3.

**FIX.** State that pass 2 asks seat 0 (or the seat to move via
`seat_of`, `replay.rs:139-140` — but say which), and add an invariant: **a
source report whose two seats do not attest the same `binary_sha256`,
`config_sha256` and `weights_sha256` is refused by name.** All three are already
in `EngineIdentity` (`transcript.rs:246-251`), so the check is free. Register
`a_report_whose_two_seats_are_different_engines_is_refused`.

### MAJOR 3 — pass 2 never verifies the engine it spawns, while its identity attests one

`capture_sha256` closes over *"both engine identities"* (`:67`) **taken from the
report**. Nothing in the design re-verifies that the binary at the recorded path
still is that engine. The arena's own replay path does exactly this, before any
game, and says why:

```
crates/pistol-arena/src/replay.rs:216-221 (and the doc comment at :204-215)
pub fn verify_engines(transcript: &Transcript) -> Result<(), ArenaError> {
    …  "not 'the engine has not changed since this pass started' but
        'the engine is the one the report attests'" … (D-252)
```

Without it, a decoy at the recorded path produces a capture file whose identity
digest attests an engine that never answered a single position. That is the
D-252/D-283 class, in the one place where the output is a corpus rather than a
verdict.

**FIX.** One sentence in §1 and one invariant: pass 2 calls
`replay::verify_engines` before spawning, and refuses by name on drift. Register
`a_capture_refuses_a_report_whose_engines_have_changed`.

### MAJOR 4 — the capture file's own shape is undesigned, while INVARIANT 5 pins it byte-for-byte

§1 (`:33-34`) — *"Write **one line per position** holding the position, the
verbatim `info totals` line, the verbatim `bestmove` line, and the game and turn
indices."*

Four fields on one line, **two of which are whole verbatim lines**, and the
totals line ends with a **variable-length `pv`** (`pistol-cli/src/report.rs:
93-96`). No delimiter, no field order, no quoting rule, no sink named. This
crate has already been bitten by exactly this: `transcript.rs:124-131` refuses a
path containing whitespace because *"the format is whitespace-delimited and does
not quote"*.

Three further gaps in the same field:
- **No sink.** The matrix's zero-seams claim rests on
  `pistol_cli::corpus::emit::Fixture` (`matrix_wp20_shape_selection.md:58`,
  D-542); the design names no sink at all.
- **No `source_sha256` in the file.** §3 correctly excludes it from the
  *identity*, but the design never says the file records it as **provenance** —
  which is precisely why `Transcript` carries it (`transcript.rs:48-50`: *"so a
  consumer can bind the replay it produces to the report it was taken from"*).
- **No versioned schema, no loader test.** §3 names *"this package's format
  version"* as a digest input; it appears nowhere in the file and no test pins
  the file's shape. Dispatch requirement 2 asks for *"a documented, versioned
  schema with a loader test"* (`wp20_dispatches.md:81-82`).

"WP-2.0-S decides the columns" cannot cover this: INVARIANT 5 makes the file's
bytes a *property this package asserts*, so this package fixes the file's shape
whether the document says so or not.

**FIX.** Name the sink, the field order, the delimiter, the `source_sha256`
provenance field and the format-version field, and register one loader test.

### MAJOR 5 — `capture_sha256` omits the instrument and duplicates what it already covers

Verified `experiment_digest` (`crates/pistol-arena/src/report.rs:41-76`). It
closes over: `openings_body_sha256`, `openings_take`, `openings_skip`,
`turn_cap`, budget kind+value, the four SPRT parameters, and **for each engine:
label, `binary_sha256`, `config_sha256`, `weights_sha256`** (`:69-73`).

Two consequences for §3's proposed inputs:

1. **"Both engine identities" is already inside `experiment_sha256`.** Adding it
   a second time is D-423's own class applied to a digest, and the design gives
   the second spelling no canonical form — while §3's stated ground for using
   `experiment_digest` is *"rather than inventing a second"* (`:72-73`).
2. **The instrument is missing.** `arena_version` is written into the report
   (`report.rs:130`) but is **not** in `experiment_digest`; and §3 does not
   propose it. So the only thing standing between a change in pass-2's own
   behaviour and a colliding capture identity is a **hand-maintained format
   version**. `docs/process.md`'s "Instrument governing revision" is the rule
   this misses: *"an artefact that produces a registered number … is named in
   the pre-registration WITH ITS REVISION."*

**FIX.** Drop the duplicated engine identities (or say why a second spelling is
wanted) and add `arena_version` (`env!("CARGO_PKG_VERSION")`) beside the format
version. Register `a_capture_identity_moves_when_the_arena_version_moves`.

**What §3 gets right, recorded because I attacked it and failed:** excluding
`source_sha256` is correct, and the sampling exclusion **is** consistent with
INVARIANT 4 (nothing is sampled, no seed is taken) — see "the strongest attack
that did not land".

### MAJOR 6 — the `totals_of` mutant names a mutation this package does not make, so the test cannot die

§7's mutant table (`:150`): *"a `totals_of` lookup made load-bearing" → dies by
`raising_totals_of_leaves_the_sprt_report_byte_identical`.*

At this revision **all three lookups are already load-bearing** — every one
carries `?`:

```
crates/pistol-arena/src/exchange.rs:184-188
Some((
    value("nodes")?.parse().ok()?,
    value(pistol_cli::report::TIME_FIELD)?.parse().ok()?,
    value("depth_turns")?.parse().ok()?,
))
```

and this package **widens nothing** — §5 keeps only the visibility half of
branch B. So the named mutation is a no-op on the code this package ships, and
the row is dead. Review-1 MINOR 5 already found the registered test is not the
one that proves output-neutrality and named the one that is
(`a_totals_line_missing_the_new_fields_still_bills_compute`); rev2 disposition
**NOT APPLIED** (`wp20_design_REVIEW_rev2.md:327`); not applied here.

**Related, and it should be one ADR line rather than a finding:** D-542 records
branch B as *"raised to `pub(crate)` **and widened** so `score` and `pv` come out
of the one parser … the two new ones non-fatal `Option`s"*
(`matrix_wp20_shape_selection.md:45-48`). This design lands the visibility and
defers the widening. It **says** it is doing that (`:104-107`), so it is not
silent drift — but D-542's own text now describes something no package is
building, and hard rule 10 wants the amendment.

**FIX.** Replace the mutant with one that exists: `INVARIANT 6`'s real content
is that no *behaviour* changed, and the mutation that would kill it is *the
visibility change accompanied by a body change*. Register the `totals_of` unit
test as the killer.

### MAJOR 7 — pass 2 builds a second recogniser of the totals marker while landing a widening with no consumer

§5 (`:100-102`): *"`exchange::totals_of` rises to `pub(crate)`. **Pass 2 does not
use it.** It reads the totals line verbatim off the channel."*

But pass 2 **must still tell `info totals …` from `info …`** to know which line
to capture — the totals marker exists for exactly that reason
(`pistol-cli/src/report.rs:20-29`, D-80). So this package ships **both** a
widening with no consumer **and** a second site recognising the marker, in the
same crate — a small copy of the duplication row (b) was killed for
(`matrix_wp20_shape_selection.md:80-83`).

`totals_of(&line).is_some()` is that recogniser, costs nothing, keeps one
reader, and gives the widening the consumer §5 says it does not have.

**FIX.** One sentence: pass 2 identifies the totals line **through
`totals_of`** and captures the line's own bytes; it discards `totals_of`'s
parsed value, which is what "does not use it" was reaching for.

### MAJOR 8 — the label budget's KIND is constrained nowhere

Review-1 MAJOR 7, **NOT APPLIED** twice, still open. §8 defers *"the label
budget's VALUE"* (`:153`) — correct under D-483 — but the **kind** is not the
value, and it is a mechanism.

Every other budget path in this crate refuses `movetime` by name:
`validate.rs:42` (`ArenaError::MovetimeBudgetRefused`, *"the one refusal this
crate exists to make loudly"*), `transcript.rs:164-170` (a movetime **source**
report is refused), `config.rs:120-134` (`go_line()` returns `None` for
`MovetimeMs`), `bin/arena.rs:49-51`. Pass 2's label budget is the new mode's
**only** budget input and nothing refuses it — a `movetime` label budget puts
wall-clock inside every captured line and destroys INVARIANT 5 outright.

**FIX.** One sentence: the label budget is one of the two instrument budgets,
its `go` line built by `BudgetSection::go_line()` (`config.rs:120`) rather than
formatted a third time. Register `a_movetime_label_budget_is_refused_by_name`.

### MAJOR 9 — dispatch requirement 5 (ledgers and the corpus manifest) is in NEITHER package

`/usr/bin/grep -c -i "ledger" docs/experiments/wp20m_design.md` → **0**;
`manifest` → **0**. `wp20_DESIGN_STOP_SPLIT.md` §3 lists what is owed to
WP-2.0-S and does not name it either. Requirement 5 asks for *"book ranges
consumed (shared with the SPRT ledger), corpus manifest with digests"*
(`wp20_dispatches.md:85-86`), and hard rule 8 makes the manifest the only thing
that indexes an uncommitted corpus. **It has fallen out of the arc.**

**FIX.** One line in §8 assigning it — to WP-2.0-S, or to the pilot's
pre-registration — so a successor can find it.

### MAJOR 10 — game outcome falls between the two packages and becomes unrecoverable

Requirement 2 names *"game outcome"* as part of a record. The capture records
none, and — by MAJOR 4 — records no pointer to its source report either. So
WP-2.0-S, working *"from bytes this package did not reshape"* (`:12-13`), cannot
recover the outcome at all: the `forfeit` flag and the result live only in the
source report (`transcript.rs:20`, `read_games` at `:307`), which the capture
does not name.

**FIX.** Either the capture line carries the source `experiment_sha256` **and**
`source_sha256` so the report is findable, or §8 states that WP-2.0-S consumes
capture-plus-report as a pair. Both are one sentence; neither is present.

### MAJOR 11 — `bin/arena.rs` is 283 lines against a hard 300-line gate

`wc -l crates/pistol-arena/src/bin/arena.rs` → **283**.
`tools/file_justification_check.sh` is CI **gate 17/19** (`tools/ci.sh:176-177`)
and its cap is *"300 lines, which is what rule 9's '~300-line soft cap' is worth
as a number a script can compare against"* (`file_justification_check.sh:21-23`).

A third mode arm needs a `USAGE` paragraph (the existing two run
`:23-47`), a `Mode` variant, a dispatch pattern and a `capture_pass` function.
That crosses the cap, and the gate then demands a
`docs/rule9_justifications.md` entry — one that may not state a count. The
design says nothing about where the code lives beyond *"a third arm"* (`:23`),
and review-1 MINOR 7 (*"the new mode's CLI grammar is not named"*) is **NOT
APPLIED** twice and still open.

**FIX.** Write the usage line and the flag grammar, name the module that holds
pass 2 (`crates/pistol-arena/src/capture.rs` or similar), and say whether
`bin/arena.rs` takes a rule-9 entry or whether the `USAGE` text moves.

---

## MINOR

**MINOR 1 — INVARIANT 3's "less the trailing newline" understates the channel.**
`channel.rs:105-106` does `String::from_utf8_lossy` (U+FFFD substitution) and
then `trim_end_matches(['\n', '\r'])` — a trailing **run** of both characters,
not "the trailing newline". Both are unreachable for `pistol`'s own ASCII,
single-`\n` output, so **INVARIANT 3 is achievable** — but it is achievable
because of a property of the *engine*, not of the *channel*, and an invariant
about bytes should say which.

**MINOR 2 — `MAX_LINE_BYTES` does not truncate, so INVARIANT 3 is not threatened
by it.** Recorded because the dispatch asked. `channel.rs:96-104`: over the cap
with no newline the reader sends `FromEngine::Overlong` and **stops**; it never
hands back a shortened `Line`. The totals line is bounded by the `pv` and comes
nowhere near 1 MiB. The residual is MAJOR 1's, not INVARIANT 3's.

**MINOR 3 — the artefact was renamed with no ADR line.** D-544 records
*"the `label_sha256`"* as the verified item; this design calls it
`capture_sha256` (`:65`). Hard rule 10 wants the amendment, not a silent
rename.

**MINOR 4 — two invariants have no test, and three tests pin no invariant.**
INVARIANT 2 (*"pass 2 never plays a move"*) and INVARIANT 7 (*"pass 1 is
unmodified"*) appear in no row of §7. Conversely
`a_capture_run_is_identified_by_its_experiment_and_its_budget`,
`two_reports_of_one_experiment_share_a_capture_identity` and
`the_capture_refuses_a_report_it_cannot_read_by_name` pin §3 prose and a refusal
that **no invariant states**. INVARIANT 3 covers the totals line *and* the
bestmove line; only the totals line has a test.

**MINOR 5 — §2's `Transcript` enumeration reads as complete and is not.**
`:45-47` omits `hang_timeout_ms`, `budget_nodes`, `forfeit`, `a_is_p1`, `index`
and per-game `nodes` (`transcript.rs:12-53`). Pass 2 needs `hang_timeout_ms` for
its own watchdog and `index` for the game column it promises.

**MINOR 6 — pass 1's config is a commit on a gate path and is not named.**
*"both engine sections naming the same committed config"* (`:26-27`) still
requires a **new arena experiment config** in `configs/`, which CI gate 6
(`tools/ci.sh:92-93`) validates. §8 does not list it among what this package does
not decide, so it reads as though pass 1 costs nothing to land.

---

## D-483 compliance

**PASS**, and it is checkable in one command:

```
$ /usr/bin/grep -oE "[0-9]+" docs/experiments/wp20m_design.md | LC_ALL=C sort -u | tr '\n' ' '
0 1 2 256 3 4 483 5 500 527 540 544 6 7 8
```

Every numeral in the document is a section number, an invariant number, a
package number (`2.0`), a D-key (483, 500, 527, 540, 544) or the `256` of
`sha256`. **No measured number, no bracket, no threshold, no budget value, no
count.** The label budget's
value and the `book_v2` range are explicitly deferred to the pilot prereg
(`:153-154`), which is exactly D-483's shape. This is the one standing law the
document obeys cleanly, and it is worth saying plainly.

---

## Could an implementer build from this without deciding something the design should have decided?

## **No.**

An implementer must decide, before writing a line:

1. whether a record is a **ply** or a **turn** (BLOCKING 2a);
2. whether the **initial** position is captured, and how it is spelled
   (BLOCKING 2b);
3. what to do at the **terminal** position of a won game (BLOCKING 2c);
4. what happens when a label ask **fails** (MAJOR 1);
5. **which seat** answers (MAJOR 2);
6. whether the engines are **verified** (MAJOR 3);
7. the capture file's **field order, delimiter and sink** (MAJOR 4);
8. whether a `movetime` label budget is **refused** (MAJOR 8);
9. the mode's **CLI grammar** (MAJOR 11).

Decisions 1, 2, 3, 5 and 7 change what the corpus IS. Decision 4 changes whether
a failed pass is detectable. And decision 3 is not a preference — the engine
refuses the position, so an implementer who reads INVARIANT 4 literally writes a
pass that hangs on the first won game.

---

## The strongest attack that did not land

**I tried to break `capture_sha256`'s central claim — that two reports of one
experiment yield one capture identity — by showing that two runs of one
experiment can hold DIFFERENT GAMES.** If they can, then
`two_reports_of_one_experiment_share_a_capture_identity` pins a defect: two
different captures under one identity, and §3's whole ground for preferring
`experiment_sha256` over `source_sha256` (*"a reason that changes no answer"*,
`:70-71`) collapses.

There were three plausible routes and **all three are closed**:

1. **The SPRT early stop.** `schedule::run` stops at the first boundary
   crossing, and the crossing is data-dependent — so a differently-scheduled run
   might stop elsewhere. It cannot: the crossing is computed over the
   **contiguous** prefix (`schedule.rs:76-77`, `contiguous` at `:114-120`) and
   the limit is taken as a **minimum** (`:80-82`), then `kept` truncates the
   contiguous prefix to it (`:92-98`). Games are deterministic at a nodes budget
   in instrument mode (hard rule 4), so the kept set is a function of the
   experiment alone. This is what
   `two_worker_run_report_identical_to_single_worker`
   (`crates/pistol-arena/tests/run_tests.rs:130`) exists to pin, and
   `report.rs:29-40` names it as the reason `n_workers` is outside the digest.
2. **A partial report.** An abandoned run writes `ABORTED_KIND`
   (`report.rs:14`, `render` at `:108-110`), and `transcript::read` **refuses**
   it at the first token: *"it carries no verdict and its games are explicitly
   not a sample"* (`transcript.rs:142-149`). A partial report cannot reach pass
   2 at all.
3. **`discarded_in_flight`.** Schedule-dependent, and correctly confined to the
   timing block (`report.rs:185-197`, `schedule.rs:16-18`) — it names games that
   were dropped, not games that were kept.

So the design's §3 rationale survives, and the exclusion of sampling from the
digest is genuinely consistent with INVARIANT 4: nothing is sampled and no seed
is taken. **§3 is the strongest part of this document**, and its residual
weakness is only the omitted instrument version (MAJOR 5), not its premise.

Two smaller attacks also failed and are recorded so nobody re-runs them:

- **Coldness across many games on one channel** (the dispatch's question 6).
  Nothing assumes a channel is per-game. `Channel` (`channel.rs:46-52`) holds no
  game state; `seats::with_seats` (`seats.rs:22-59`) spawns, handshakes,
  verifies identity, sends **one** `newgame` and hands the channels to an
  arbitrary `drive` closure — it is `replay::one_game` (`replay.rs:106-108`)
  that chooses to call it per game, not `with_seats`. And the engine's
  `new_game` is order-free and idempotent — `self.state = GameState::new_game();
  self.searcher.clear();` (`instance.rs:73-76`) — with `set_position` replacing
  state wholesale (`:78-84`). **INVARIANT 1 is sound and cheap.**
- **INVARIANT 6's widening as gate-rejected dead code** (the dispatch's question
  4). It is not dead: `totals_of` keeps its in-crate consumer at
  `exchange.rs:76`, so `dead_code` cannot fire. And `clippy::redundant_pub_crate`
  is a **nursery** lint while gate 4 denies only `clippy::all`
  (`tools/ci.sh:86-87`, `Cargo.toml:47-48`). **The visibility change is genuinely
  output-neutral and no gate rejects it.** The finding against §5 is MAJOR 6 and
  MAJOR 7 — the mutant and the missing consumer — not the widening itself.

---

## What I could not settle by reading, and the run that would

- **BLOCKING 1's magnitude.** That `nps`/`time` differ across two runs is
  asserted by `determinism.sh:153-154` and by the gate's own closing line; I did
  not measure the spread. **The run that settles it:** two `arena --config`
  passes over one experiment and
  `LC_ALL=C diff <(…) <(…)` on the raw `info totals` lines, against the same
  diff after `determinism.sh`'s `normalize`. Refused here per the dispatch.
- **MAJOR 11's line count after the edit.** 283 is measured; the size of the
  third arm is not. **The run that settles it:** write the arm and run
  `tools/file_justification_check.sh`.
