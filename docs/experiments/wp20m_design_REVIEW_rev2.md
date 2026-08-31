# SCOPED RE-REVIEW — `docs/experiments/wp20m_design.md` revision 2

## Header

- **Named revision adjudicated:** `7af62e7322a8db5674b95bd2cd4e4ef8b9802dc0`
  (`docs(wp20m): the meaning-free slogan falls and four capture decisions come
  onto the document's face …`).
- **Matches HEAD:** **yes.** `git rev-parse HEAD` →
  `7af62e7322a8db5674b95bd2cd4e4ef8b9802dc0`, branch `dev`.
- **Tree state:** `git status --porcelain` → **empty**. Clean.
- **Revision 1 compared against:** `5064b0563775c013bc0c797ee8092c3b891625f9`,
  read via `git show 5064b05:docs/experiments/wp20m_design.md` and diffed
  line-for-line against the subject.
- **What I ran:** `git`, `git grep`, `/usr/bin/grep`, `sed`, `diff`, `wc`, `cat`,
  `ls`, `sort`. Every recorded grep is `/usr/bin/grep` or `git grep` (D-265).
- **What I refused to run:** `cargo` in any form and `tools/ci.sh`, per the
  dispatch. Where a claim needs a run I name the run rather than assert it.
- **Read as binding:** CLAUDE.md, `docs/process.md`,
  `docs/experiments/wp20m_design_REVIEW.md` (in full),
  `docs/experiments/wp20_dispatches.md` §requirements, `docs/book_v2_ledger.md`,
  D-5/6/80/200/252/423/424/477/483/500/518/527/540/542/544.
- **Code read at this revision:** `pistol-arena` `config.rs`, `validate.rs`,
  `transcript.rs`, `identity.rs`, `replay.rs`, `exchange.rs`, `report.rs`,
  `lib.rs`, `bin/arena.rs`, `tests/common/mod.rs`, `Cargo.toml`; `pistol-cli`
  `report.rs`, `protocol.rs`, `corpus/emit.rs`, `tests/arena_smoke_gate_tests.rs`;
  `pistol-engine` `position.rs`, `position_token.rs`; `pistol-core` `turn.rs`;
  `tools/determinism.sh`, `tools/file_justification_check.sh`,
  `tools/arena_smoke.sh`, `tools/ci.sh`.

---

## VERDICT: **FAIL**

**1 BLOCKING, 5 MAJOR, 6 MINOR** newly found, against a fix round that applied
most of what it was asked to apply.

This is not a verdict on effort. Of the 20 prior findings, **7 are cleanly
APPLIED**, 3 more are moot because the offending text is gone, 2 are PARTIALLY
APPLIED, 5 are NOT APPLIED, and **3 are APPLIED BUT INTRODUCED A NEW DEFECT**.
Revision 2 fixed the two hardest things it was asked to fix — the
nps/time contradiction and the undefined asked set — and fixed them correctly
against the tree. I verified both by hand and they hold.

It fails on one thing, and it is the thing the rewrite itself caused:

> **§3's two-engine refusal rejects every report it is meant to accept.** The
> arena REQUIRES the two engine sections to differ. Revision 1 said so, in
> those words. The rewrite deleted that sentence and asserted its negation.

That is a mechanism that refuses 100% of its inputs, stated as the section that
answers *"WHICH ENGINE ANSWERS"*, and the section still does not answer it.

---

# PART 1 — WHAT THE REWRITE LOST

This was my first job. The document was rewritten rather than patched, and
**seven things that were right in revision 1 are gone.** One of them is the
BLOCKING finding.

### L1 — the true sentence about labels, deleted and replaced by its negation. **This is the BLOCKING.**

Revision 1, §1:

> both engine sections naming the same committed config. Self-play is
> expressible today: **`validate_engines` refuses identical *labels*, not
> identical binaries or configs.**

Revision 2, §1, same sentence, truncated:

> both engine sections naming the same committed config. Self-play is
> expressible today.

Revision 2, §3, asserting the opposite:

> Pass 2 **refuses a report whose two engine sections are not identical**, by
> name. **A self-play report — the only kind this pipeline produces — has
> identical sections**, so the refusal costs nothing it is meant to accept.

Revision 1 knew the fact. The rewrite dropped the clause that carried it and
then built a mechanism on its negation. Full analysis in **BLOCKING A**.

### L2 — §4 COLDNESS, entire, including an obligation on the pilot that now lives nowhere

`/usr/bin/grep -n -i "cold\|D-540\|D-500\|fresh-process" docs/experiments/wp20m_design.md`
→ **no output.** Revision 2 contains the word "cold" zero times.

Revision 1's §4 carried three things, and the prior review **passed all of
them** — its own "strongest attack that did not land" section re-derived the
coldness chain and closed with *"INVARIANT 1 is sound and cheap."* D-544 names
coldness as one of the four verified things that licensed this package to land
at all. Gone with it:

1. **The verified chain** — `Table::clear` a true `fill(EMPTY)` and not the
   epoch bump beside it, `Solver::reset` rebuilding, `Position::reset_to`
   unwinding the eval, `params` immutable, `census` `None`, the `PvTable`
   per-`Run`. Revision 2 keeps the `newgame` invariant (INVARIANT 4) with **no
   statement of why a `newgame` is sufficient.**
2. **The cost, not claimed free** — *"a memset per captured position. The pilot
   measures it (D-500)."*
3. **What the pilot's pre-registration owes** — *"D-540's second clause — a
   **fresh-process agreement criterion**, proving the construction holds by
   agreement between a pass-2 capture and the same position re-asked in a fresh
   process, and **naming the defect class it excludes**, because a criterion
   that is a property of the named defect passes vacuously (D-527)."*

Item 3 is the one that matters. **D-540 is standing law** — *"labels must be
cold by construction with a registered fresh-process agreement criterion in the
pilot"* — and revision 1 was the only place in this package that recorded the
obligation. A successor writing the pilot's pre-registration from revision 2
will not find it. This is D-544's own "requirement 5 falls between the
packages" defect, repeated inside one package across two revisions of one file.

### L3 — the legality precondition that makes INVARIANT 1 decidable

Revision 1, §2:

> **`transcript::read` legality-checks every game through `pistol-core` at read
> time**, refusing the whole report on an illegal turn or on moves after a win.
> So **every move list pass 2 walks is a guaranteed legal prefix before pass 2
> exists** — it cannot be handed something that panics. **This is the property
> the scoped re-review confirmed as its own strongest failed attack.**

Revision 2's INVARIANT 1 now turns on the phrase *"at which the engine can
legally be asked"* — and the ground that makes legality already established,
and checkable in advance, was deleted in the same rewrite. The property is
still true (I re-verified it: `transcript::replays`, `transcript.rs:359-379`,
refuses an illegal turn and refuses turns recorded after a win). It is simply no
longer written down, in the revision whose central invariant depends on it.

### L4 — "WHY VERBATIM", the package's reason to exist

Revision 1, §1:

> A consumer that disagrees with WP-2.0-S can re-parse the capture without
> re-running the engine, **which is the property that makes an expensive corpus
> survive a schema mistake.**

That sentence is the answer to *"why is this package worth building before the
schema is settled"*, and it is revision 1's own commit-message thesis. Revision
2 deletes it and puts nothing in its place. §4 now says what is written; nothing
says why.

### L5 — revision 1's INVARIANT 3, with no successor. **See MAJOR B.**

Revision 1 INVARIANT 3: *"The captured totals and bestmove lines are
byte-identical to what the engine wrote."* Revision 2 has **nine invariants and
none of them is that one.** Detailed in MAJOR B.

### L6 — a registered test dropped while the rule it pinned survived

Revision 1 registered `the_capture_refuses_a_report_it_cannot_read_by_name`.
Revision 2 drops it. Revision 2's §6 **still states that refusal** as row 1 of
its failure table (*"the report is unreadable … refuse the run, by name"*), and
INVARIANT 8 still asserts it. The rule kept its place; its test did not.

### L7 — a digest input lost its test and its mutant

Revision 1 registered `a_capture_run_is_identified_by_its_experiment_and_its_budget`
and the mutant *"the label budget dropped from the identity."* Revision 2 drops
both. Revision 2's §5 **still names the label `go` line as an input to
`capture_sha256`.** Delete that input and no registered test in revision 2
fails.

---

# PART 2 — DISPOSITION OF EVERY PRIOR FINDING

## BLOCKING

### BLOCKING 1 (nps/time; INVARIANT 3 vs 5; the vacuous test) — **APPLIED, BUT INTRODUCED TWO NEW DEFECTS**

**The remedy is correct and I verified it against the tree rather than the
document's word. It holds.** This is the dispatch's check 1 and it comes back
clean:

**(i) The sed matches the field order the totals line actually emits.**
`crates/pistol-cli/src/report.rs:62-90`:

```
"{INFO_PREFIX}{marker} depth_turns {} seldepth {} nodes {}{solver_field} {NPS_FIELD} {} \
 {TIME_FIELD} {} hashfull {} score {} pv",
```

and `solver_field` (`report.rs:62-78`) is
`" search_nodes … solver_root_nodes {}"`. **The solver fields are interpolated
between `nodes` and `nps`, never between `nps` and `time`.** So `nps <n>` and
`time <n>` are adjacent, in that order, whether or not the solver fields are
present, and `determinism.sh:154`'s `sed -E 's/ nps [0-9]+ time [0-9]+//'`
matches in both cases. The dispatch's worry does not fire.

**(ii) `hashfull` is deterministic, and gate 9 compares the SAME output — not a
different one.** I checked the dispatch's hypothesis directly.
`tools/determinism.sh:218-243` writes a script of literally
`newgame\nposition <p>\ngo <budget>\n` per position — **pass 2's own loop
shape** — captures the engine's raw stdout, pipes it through the same
`normalize`, and `diff -u`s run A against run B. The totals lines are in that
diff (`check_content` greps `^info totals depth_turns [1-9]`), so `hashfull`,
`score`, `pv`, `nodes`, `seldepth` and the solver fields are all inside the
compared bytes. The gate passes in CI. **Stripping only nps/time is therefore
sufficient, on the evidence of the one instrument this project trusts to say two
runs agree, over the same output and the same loop.** INVARIANT 6 is
achievable.

**(iii) The contradiction is gone** because revision 1's flat INVARIANT 3 is
gone and §4's *"Nothing **else** is touched"* scopes the exception correctly.

**The two new defects the remedy introduced are MAJOR B and MAJOR C below.**

### BLOCKING 2 ("every to-move position" undefined at three boundaries) — **APPLIED**

All three limbs are taken, and all three are correct against the tree:

- **(a) ply vs turn.** §2 takes it as a **decision**, not a domain fact, with
  three grounds. Verified: `PositionSpec::Start { moves }`
  (`position.rs:10-14`) carries no `phase`, and its doc says *"it always names a
  position at a turn boundary"*; `PositionSpec::Set` (`:37-46`) is the only form
  with `phase`. The claim that `Start` cannot express mid-turn is exactly true.
- **(b) the empty prefix.** Verified both halves: bare `position start` parses
  (`position_token.rs:83` — `None => Vec::new()`); `position start moves` with
  nothing after it is refused by name (`:84-86`); and
  `exchange::position_line(&[])` (`exchange.rs:154-161`) emits precisely the
  refused form. §2 says pass 2 does not use it for the empty case. Correct.
- **(c) the decided terminal position.** Verified: `PositionSpec::replay`
  (`position.rs:68-73`) refuses `Outcome::Win`. §2 reframes the exclusion as the
  protocol's own precondition rather than an exclusion by outcome, and rewords
  INVARIANT 1 so it no longer forbids the one exclusion the rules force. The
  self-contradiction is gone.

**And INVARIANT 2 is implementable — I checked the dispatch's question.** Pass 2
*can* know a position is decided without asking. `Transcript.games[].moves`
(`transcript.rs:21-22`) is the full move list; `Outcome` has exactly two
variants, `Ongoing` and `Win` (`pistol-core/src/turn.rs:34-45`), with
`is_decided`; and `transcript::replays` (`:359-379`) refuses any turn recorded
after a win, **so only the final prefix `k = len` can ever be decided.** Pass 2
replays through pistol-core — which `transcript::read` already does at read time
— and tests the last prefix. One check, on data it holds.

*Residual, MINOR G:* the review's FIX asked for the prefix range in the open —
*"prefixes `k ∈ [0, len]`, less `k = len` when the last recorded turn wins"* —
and revision 2 states the property instead of the range, having deleted (L3) the
paragraph that established the property. An implementer derives it; nothing in
the document tells them only the last prefix can be decided.

### BLOCKING 3 (the false licence used as a licence) — **APPLIED**

*"A capture that adds no meaning cannot be wrong about meaning"* and
*"deliberately unable to get them wrong"* are gone. The replacement is the
review's own proposed wording, and the four capture decisions are tabulated on
the first page with section pointers. D-544's overstated split premise is
corrected on the face of the document, naming the five findings that crossed the
line unfixed. Cleanly done.

## MAJOR

### MAJOR 1 (pass 2's failure modes undesigned) — **APPLIED**

§6 is a named five-row table, INVARIANT 8 pins it, and
`an_error_answer_refuses_the_run_and_names_the_game_and_turn` plus the mutant
*"an `error` answer skipped instead of refusing"* pin the hard case. The closing
rule — *"Every failure is a refusal of the whole run and none is a skip"* —
is total, so `Received::Closed` and `Received::Overlong` are covered even though
not named as rows (MINOR H).

**I verified the dispatch's check 5 — what the protocol actually emits on a
refused `position`, and whether `exchange`'s reader recognises it.** It does,
and it is neither an unsolicited line nor an out-of-turn forfeit:

- `protocol.rs:103-106` — `POSITION => { self.position(line, rest)?; }`; the
  error propagates and `:82` writes `out(&error_line(&error))`, so the engine
  emits `error <NamedError>: <why>` on its normal stdout and stays alive (D-5).
- `exchange::ask` (`exchange.rs:31-36`) checks `channel.unsolicited()` **before**
  sending, so the error — which arrives after the send — is read inside the
  loop, not as a stray.
- `exchange.rs:71-76` — `if line.starts_with("error ")` → `Answer::Forfeit {
  reason: ForfeitReason::ProtocolError, line: Some(line) }`. **Recognised by
  name.**

So §6's `error` row is implementable. Pass 2 maps that outcome to a refusal
rather than a forfeit, which is the right call in a pass that plays no game.

### MAJOR 2 (which seat answers; no two-engine refusal) — **APPLIED BUT INTRODUCED A NEW DEFECT**, and its first half is **NOT APPLIED**

The refusal is now stated — and it is stated backwards. **BLOCKING A.**

*Which seat answers* is still not said, for the fourth revision running. §3 is
titled *"WHICH ENGINE ANSWERS"* and its answer is *"Pass 2 spawns **its
engine**"* — singular, unnamed slot. The design's escape is that it does not
matter because the sections are identical; since they cannot be identical, it
matters, and the implementer decides.

### MAJOR 3 (pass 2 never verifies the engine it spawns) — **APPLIED**

§3's second paragraph and INVARIANT 3 add it, and
`a_respawned_engine_that_does_not_match_the_report_is_refused` pins it. Verified
that the function §3 gestures at exists and does what §3 claims:
`replay::verify_engines` (`replay.rs:216-241`) re-captures each slot's identity
and compares against `transcript.identities[slot]`, with the doc comment stating
the D-252 rationale in the same words §3 uses. `identity::verify_respawn`
(`identity.rs:99-122`) also exists — it is the per-spawn seat check taking
`(&EngineSection, &EngineIdentity, &Identity)` — so both functions the dispatch
asked after are real. §3 names neither, saying *"the way the arena's own replay
does"*; that is enough to locate `replay::verify_engines`.

### MAJOR 4 (the capture file's own shape undesigned) — **PARTIALLY APPLIED**

**Applied:** the sink is named (`pistol_cli::corpus::emit::Fixture` — verified
reachable: `pistol-cli/src/lib.rs:36` `pub mod corpus`, `corpus/mod.rs:6`
`pub mod emit`, and `pistol-cli` is a dependency of `pistol-arena`); the header
shape is right (`Fixture::param` / `::derived` at `emit.rs:36-44`, in-band
`BODY_DIGEST = "# body_sha256 "` at `:6`); the format-version field and the
`source_sha256` + `experiment_sha256` provenance are on the file's face.

**Not applied, and it is the half the finding was about:** the record line still
has **no field order, no delimiter and no quoting rule.** It carries four
fields, three of which are whole whitespace-bearing lines — the position as sent
(`position start moves q,r q,r …`), the normalised totals line (ending in a
variable-length `pv`), and the `bestmove` line. `transcript.rs:124-131` already
refuses a path containing whitespace *"because the format is whitespace-delimited
and does not quote"*; this crate has been bitten by exactly this field. And
INVARIANT 6 pins the file byte-for-byte, so **this package fixes the shape
whether or not the document does.** Dispatch requirement 2's *"loader test"* is
still registered nowhere.

### MAJOR 5 (`capture_sha256` omits the instrument, duplicates the identity) — **NOT APPLIED**

Both limbs stand, verified at this revision:

- **Duplication.** `report::experiment_digest` (`report.rs:41-76`) closes over,
  per engine, `label`, `binary_sha256`, `config_sha256`, `weights_sha256`
  (`:62-74`). §5 adds *"the engine identity pass 2 verified"* on top of
  `experiment_sha256`, so the identity is in the digest twice, in two spellings,
  and the second is given no canonical form and no slot.
- **The instrument is absent.** `arena_version` is written into the report
  (`report.rs:130`, `env!("CARGO_PKG_VERSION")`) and is **not** in
  `experiment_digest`; §5 does not add it. So the only thing between a change in
  pass 2's own behaviour and a colliding capture identity remains a
  hand-maintained format version. `docs/process.md`'s "Instrument governing
  revision" is the rule this misses.

### MAJOR 6 (the `totals_of` mutant names a mutation this package does not make) — **APPLIED**

§8 removes it and says so by name: *"Revision 1 registered a mutant for a
mutation this package does not make … It is removed."* The mutant table no
longer carries the row. Correct.

*Still owed, and it is an ADR act rather than a document edit:* D-542 records
branch B as *"raised to `pub(crate)` and widened"*, and no package is building
the widening. Hard rule 10 wants the amendment.

### MAJOR 7 (a second recogniser of the totals marker) — **NOT APPLIED**, and the fix round sharpened it

§8 restates revision 1 nearly verbatim: *"Pass 2 does not call it: it captures
the totals line without parsing it."* The finding was that pass 2 must still
tell `info totals …` from `info …` — the marker exists for exactly that (D-80,
`pistol-cli/src/report.rs:20-29`) — so the package ships a widening with no
consumer *and* a second recogniser of the same marker, in one crate.

**And BLOCKING 1's remedy makes it worse.** §4 now requires pass 2 to locate and
excise ` nps <n> time <n>` from inside that line. Field-level text surgery on
the totals line is the thing §8 says pass 2 does not do. The two sections are
not contradictory — you can excise a substring without parsing — but §8's
sentence is the design's whole ground for the widening having no consumer, and
after the rewrite it is less true than it was in revision 1.

### MAJOR 8 (the label budget's KIND constrained nowhere) — **PARTIALLY APPLIED**

**The substance is taken.** §7 is a whole section: *"`nodes`, and never
`movetime_ms`"*, with the arena's own refusal quoted and the INVARIANT 6
consequence stated. Verified the crate really does refuse movetime in every
other budget path: `validate.rs:41-42` (`MovetimeBudgetRefused`),
`transcript.rs:163-170` (a movetime **source report** refused by name),
`config.rs:120-134` (`go_line()` → `None` for `MovetimeMs`), and `bin/arena.rs:229`'s `unreachable!("validate refuses a movetime budget before this point")`.

**Not taken:** the review asked for `a_movetime_label_budget_is_refused_by_name`.
**§10 registers no such test and §10's mutant table no mutant.** §7 is the one
rule in this document that nothing pins — delete the refusal and every
registered test still passes. And `BudgetSection::go_line()` is not named as the
builder, so the label `go` line gets formatted a third time or does not; the
implementer decides.

### MAJOR 9 (requirement 5 in neither package) — **APPLIED BUT INTRODUCED A NEW DEFECT**

The FIX asked for *"one line in §8 assigning it, so a successor can find it."*
§11 assigns it — to **this** package — and then declares it already delivered.
**Assignment satisfies the letter; the delivery claim does not survive checking,
and closing a requirement by assertion is worse for a successor than the gap
that was found.** This is the dispatch's check 6, and both halves fail:

**(a) "delivered by §4's header plus the `body_sha256` the fixture type
appends" is not a manifest in the sense the dispatch means.** Requirement 5
(`wp20_dispatches.md:85-86`) asks for a *"corpus manifest with digests"*, and
hard rule 8 fixes what a manifest is for: *"Nets, books, match logs, bench
outputs are never committed; **a committed manifest may sha-index them.**"* The
`Fixture` body digest is an in-band self-digest **inside the uncommitted corpus
file** — a file whose own digest is consistent with itself no matter what it
holds. Nothing committed names the capture, so from the repository alone a
successor cannot say which corpus files a governed run produced, or verify one.
That is precisely the job the finding said had fallen out of the arc.

**(b) the `book_v2` ledger claim — *"pass 1's config, which is the arena's
existing business"* — is false, and checkably so.**
`git grep -in "ledger" -- crates/ tools/` returns **no `pistol-arena` hit at
all**; the arena writes no ledger row and reads none. The ledger is
`docs/book_v2_ledger.md`, a hand-maintained committed document, and its own rule
(`:16-18`) is:

> A new pre-registration takes the next unconsumed range, **adds its row here in
> the same commit that adds its arena config**, and never re-reads a range this
> table already holds.

So the row is an obligation of **whoever lands pass 1's arena config** — which
is this package (see MINOR L: that config is not named either) or the pilot's
pre-registration. "The arena's existing business" names nobody, and D-518's
closing line records that the ledger *"starts EMPTY"* and flips *"when a
governed pre-registration draws its first slice."* This package's pass 1 is that
draw.

### MAJOR 10 (game outcome unrecoverable) — **APPLIED**

§4's *"THE SOURCE IS NAMED ON THE FACE OF THE FILE"* takes the review's own fix:
the header carries `experiment_sha256` **and** `source_sha256`, stated with the
reason — so WP-2.0-S can find the report holding the outcomes and forfeit flags.
Correct, and correctly separated from §5's identity, which excludes
`source_sha256` on grounds I re-attacked and could not break.

### MAJOR 11 (`bin/arena.rs` at 283 against a hard 300) — **APPLIED**

§1 gains a whole *"WHERE THE CODE LIVES"* paragraph, and the dispatch's check 4
says its answer is sufficient:

- `wc -l crates/pistol-arena/src/bin/arena.rs` → **283**;
  `tools/file_justification_check.sh:65` → `SOFT_CAP=300`. Both as stated.
- The `USAGE` const spans `src/bin/arena.rs:16-59` — **44 lines**. Extracting it
  leaves the binary near 239 with about 61 lines of headroom for a `Mode`
  variant, a match arm and a call. Sufficient.
- **Does moving a `const` out of a `[[bin]]` into a module work?** **Yes.**
  `crates/pistol-arena/src/lib.rs:47-70` is a plain `pub mod` list and the
  binary already does `use pistol_arena::{identity, openings, outpath, replay,
  …}` (`bin/arena.rs:7-13`), so a new `pub mod usage;` is reachable from the bin
  exactly as the others are. One consequence the design does not state: the bin
  is a **separate crate**, so the const must become `pub`, not `pub(crate)` — and
  a `pub` item takes a `///` doc under this project's style rule. Mechanical, but
  it is the one place where "a mechanical extraction that adds no behaviour"
  is not quite the whole story.
- The CLI grammar (review-1 MINOR 7, open for three revisions) is now given:
  `arena --capture <report> --out <capture> --go <go line>`. Residual in MINOR I.

## MINOR

- **MINOR 1** (*"less the trailing newline"* understates the channel) — **moot
  by deletion.** The phrase is gone with revision 1's INVARIANT 3. But so is the
  invariant (L5 / MAJOR B), and the channel's actual behaviour
  (`from_utf8_lossy` then `trim_end_matches(['\n', '\r'])`) is now stated
  nowhere, in a document whose INVARIANT 6 is about bytes.
- **MINOR 2** (`MAX_LINE_BYTES` does not truncate) — **nothing was owed.**
  Recorded-only finding; no action needed and none taken.
- **MINOR 3** (`label_sha256` → `capture_sha256` renamed with no ADR line) —
  **NOT APPLIED.** `/usr/bin/grep -c "capture_sha256" docs/decisions.md` → **0**;
  `label_sha256` → **1** (D-544, the verified item). Revision 2 keeps the new
  name. Hard rule 10 wants the amendment.
- **MINOR 4** (invariants with no test; tests pinning no invariant) — **NOT
  APPLIED, and worsened.** Full re-tally in MAJOR D.
- **MINOR 5** (§2's `Transcript` enumeration reads as complete and is not) —
  **moot by deletion**; revision 2 has no such enumeration. Residual: §6 refers
  to *"the watchdog"* and the document never says it is the report's own
  `hang_timeout_ms` (`transcript.rs:44-47`), which is the field a pass 2
  watchdog must use.
- **MINOR 6** (pass 1's config is a commit on a gate path and is not named) —
  **NOT APPLIED.** §1 still says only *"both engine sections naming the same
  committed config"*, which is the **engine** config. Pass 1 additionally needs a
  new **arena experiment** config in `configs/` (thirteen exist today,
  `configs/arena_*.toml`), validated by CI gate 6 (`tools/config_check.sh`,
  `ci.sh:92-93`). §11 does not list it as deferred, so it reads as though pass 1
  costs nothing to land — and it is the very commit MAJOR 9(b)'s ledger row
  attaches to.

---

# PART 3 — NEW DEFECTS

## BLOCKING

### BLOCKING A — §3's two-engine refusal rejects every report it is meant to accept

This is the dispatch's check 3, and it lands.

**What §3 says:**

> Pass 2 **refuses a report whose two engine sections are not identical**, by
> name. **A self-play report — the only kind this pipeline produces — has
> identical sections**, so the refusal costs nothing it is meant to accept …

**"Engine section" is a type in this crate**, and §3's next paragraph names
`EngineIdentity` separately, so the reference is unambiguous:

```
crates/pistol-arena/src/config.rs:170-193
pub struct EngineSection {
    pub label: String,
    pub binary: PathBuf,
    pub binary_sha256: String,
    pub config: PathBuf,
}
```

**Both layers that can produce or admit a report REFUSE identical labels:**

```
crates/pistol-arena/src/validate.rs:242-250
if self.engine_a.label == self.engine_b.label {
    return Err(ArenaError::config("engine_b.label",
        format!("the two sides must be told apart in the report; both are labelled `{}`", …)));
}
```

```
crates/pistol-arena/src/transcript.rs:189-194
if engines[0].label == engines[1].label { … "both seats carry the label `{}`, so no
game in it can be attributed to a seat at all" … }
```

There is a third, load-bearing reason it can never be relaxed:
`read_games` (`transcript.rs:270-298`) **attributes each game's seats by label**
and refuses a game whose two seat labels are not the two the report's engines
carry. Identical labels would make every game unattributable.

**Therefore, for every report pass 2 can read, `engines[0] != engines[1]` —
they differ at `label`, always, by construction. §3's mechanism refuses 100% of
its inputs, and its stated justification — "has identical sections, so the
refusal costs nothing it is meant to accept" — is exactly inverted: it costs
everything and accepts nothing.**

Revision 1 recorded the correct fact in the sentence the rewrite deleted (L1),
so this is not a fact the author never had. It is a fact the rewrite lost.

**This is not a wording slip an implementer absorbs.** It is the design's answer
to *"WHICH ENGINE ANSWERS"*, and repairing it requires deciding something the
design should have decided: **which fields make two engines "the same
engine."** The prior review's own FIX named the answer, and revision 2 did not
take it — *"a source report whose two seats do not attest the same
`binary_sha256`, `config_sha256` and `weights_sha256` is refused by name."*
Those are `EngineIdentity`'s fields, not `EngineSection`'s:

```
crates/pistol-arena/src/identity.rs:13-22
pub struct EngineIdentity {
    pub id_lines: Vec<String>,
    pub binary_sha256: String,
    pub config_sha256: String,
    pub weights_sha256: String,
}
```

**None of them carries a label**, and `Transcript` holds them per slot
(`transcript.rs:32-34`). Two seats running one binary with one config produce
identical identities. **So the check the design wanted is available, free, and
one type to the left of the one it named.**

**FIX.** State the refusal over `Transcript::identities`, not
`Transcript::engines`, and name the fields. Then §3's title question still needs
its answer: **say which slot pass 2 spawns** (slot 0 is the obvious choice once
the identities are attested equal — but the design must say it). Re-register the
test as `a_report_whose_two_seats_attest_different_engines_is_refused_by_name`,
and add the case the current wording would break:
`a_self_play_report_whose_seats_carry_distinct_labels_is_accepted`. That second
test is the one that would have caught this, and its absence is why the defect
survived a rewrite.

## MAJOR

### MAJOR B — the package's central property is now pinned by no invariant

Revision 2 has nine invariants. **Not one of them says the captured lines are
what the engine wrote.** Revision 1's INVARIANT 3 said it; the rewrite deleted it
(L5) and put no successor in the list.

The property survives only in §4 prose and in one test. That is a real weakening,
and it is visible in the invariant set's own logic: **INVARIANT 6 (a re-run is
byte-identical) is satisfied by a capture that mangles every totals line, as long
as it mangles it the same way twice.** The mutant table knows this — it registers
*"the normalisation widened to strip another field"* — but the mutant is killed by
a **test that pins no invariant**, which is exactly MINOR 4's shape.

Worse, the **`bestmove` line has neither an invariant nor a test.** Revision 1's
INVARIANT 3 covered it and the prior review flagged that only the totals line had
a test (MINOR 4). Revision 2 removed the invariant and still did not add the test.
The bestmove line is half the label.

**FIX.** One invariant: *"Every captured line is the engine's own bytes, less the
two wall-clock fields; no other field is reordered, renamed, dropped or
combined, and the `bestmove` line is untouched."* Register
`a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote`.

### MAJOR C — §10's test obligation is not dischargeable where the design puts it

§10 says the re-run test *"**is driven by the real `pistol` binary**"*, because
against the stub the normalisation is unobservable. **The obligation is exactly
right — it is the remedy for BLOCKING 1's second limb and for D-527's defect
class. The design does not say where such a test can live, and in the crate
where every other test in §10 lives, it cannot.**

- `crates/pistol-arena/tests/common/mod.rs:9-11` offers exactly two engines:
  `env!("CARGO_BIN_EXE_arena")` and `env!("CARGO_BIN_EXE_arena-stub-engine")`.
- `CARGO_BIN_EXE_<name>` is set **only for the `[[bin]]` targets of the package
  being built.** `crates/pistol-arena/Cargo.toml` declares `arena` and
  `arena-stub-engine`; `pistol` is a `pistol-cli` binary.
- Confirmed by sweep: `git grep -rn "CARGO_BIN_EXE" -- crates/` returns
  `CARGO_BIN_EXE_pistol` **only** under `crates/pistol-cli/tests/`. **Zero
  occurrences in `crates/pistol-arena/`.**
- And nothing else in `crates/pistol-arena/tests/` reaches a real engine by any
  other route.

**The one place in this tree where the real `pistol` runs through the real
`arena` is `tools/arena_smoke.sh`** (CI gate 15/19, `ci.sh:157-158`) — it builds
`--bin pistol --bin arena` (`:94-95`), binds both seats by content
(`:192-247`), and runs the arena for real (`:249-252`). Its sibling
`crates/pistol-cli/tests/arena_smoke_gate_tests.rs` drives that *script* using
**stub** binaries built into a scratch workspace, so it is not a second route
either.

So the discharge route is a **shell gate**, or a test in `pistol-cli`'s test
crate — either way, **outside the crate the design puts the work in**, and a
shell-gate route pulls in `docs/process.md`'s tools/ review coverage rule and
`tools/SHELL_CHECKLIST.md`, neither of which this design mentions. Revision 2
proposes no `tools/` change at all.

**Why this is MAJOR and not MINOR:** an implementer who writes
`a_rerun_over_one_report_is_byte_identical` in `crates/pistol-arena/tests/`,
where §10's other twelve names plainly belong, has exactly one engine available
— the stub, whose `nps: 1, time_ms: 0` are hardcoded — and the test goes green
while pinning nothing. **That is the precise vacuity §10's paragraph exists to
forbid**, and the design gives the implementer nothing that stops them walking
into it.

**FIX.** Name the site: either *"the re-run test is a `pistol-cli` integration
test, where `CARGO_BIN_EXE_pistol` is defined"*, or *"the re-run receipt is taken
by an extension to `tools/arena_smoke.sh`, reviewed against
`tools/SHELL_CHECKLIST.md`."* The second carries a cost this design has not
stated and `arena_smoke.sh` pre-registers its own (`:29`).

### MAJOR D — the invariant/test/mutant sets do not close

This is the dispatch's check 8, done exhaustively. 9 invariants, 13 tests, 9
mutants.

**No two invariants are in tension.** Revision 1's BLOCKING 1 (INVARIANT 3
against 5) and BLOCKING 2c (INVARIANT 4 self-contradictory) are both genuinely
resolved: INVARIANT 1 is now scoped by *"at which the engine can legally be
asked"* and INVARIANT 2 states the consequence, so they agree rather than
collide. **That part is clean and I want it recorded, because it was revision
1's worst finding.**

**Invariants with no test — 2:**

| invariant | status |
|---|---|
| **5** — *"Pass 2 never plays a move"* | no test. Open since revision 1 (MINOR 4) |
| **9** — *"Pass 1 is unmodified"* | no test. Test 13 is mapped to INVARIANT 7 |

**Tests pinning no invariant — 2:**

| test | pins |
|---|---|
| `a_captured_totals_line_keeps_every_field_but_nps_and_time` | §4 prose only — **the package's central property, and no invariant states it** (MAJOR B) |
| `two_reports_of_one_experiment_share_a_capture_identity` | §5 prose only — no invariant states the capture identity rule |

**Rules with neither test nor mutant — 2:** §7's movetime refusal (MAJOR 8), and
§5's inclusion of the label `go` line in `capture_sha256` (L7 — revision 1 had
both and the rewrite dropped them while keeping the rule).

**Refusals stated by INVARIANT 8 with no test — 3 of 4:** the unreadable report
(revision 1 had that test; L6), the identity mismatch is covered by test 5, the
watchdog timeout has none, and only the `error` answer is pinned.

**Tests with no mutant — 3:** tests 1, 5 and 13. Not fatal, but the mutant table
is the document's own claim that each test can die.

### MAJOR E — the capture record's line format is still undecided while INVARIANT 6 pins its bytes

Carried from MAJOR 4's unapplied half, restated here because it is what an
implementer hits first. Three whitespace-bearing whole lines plus two indices go
onto one line, into a `Fixture` body, with no delimiter, no field order and no
quoting rule — in a crate that already refuses whitespace in a path because
*"the format is whitespace-delimited and does not quote"* (`transcript.rs:
124-131`). The `pv` is variable-length and unbounded, so the record has no fixed
arity either.

### MAJOR F — coldness and the pilot's owed criterion are gone (L2)

Restated as a finding because it is not merely lost prose. **D-540 is standing
law**, its second clause obliges the pilot's pre-registration to carry a
fresh-process agreement criterion **naming the defect class it excludes**, and
revision 1 was the only document in this package that recorded the obligation.
Revision 2 mentions neither coldness nor D-540 nor D-527. The `newgame`
mechanism survives as INVARIANT 4 with no stated ground, and the memset cost
(D-500) is unstated, so §7's *"the label budget is `nodes`"* now sits beside a
per-position cost the document no longer admits exists.

## MINOR

- **MINOR G** — the asked set's prefix range is stated as a property, not
  enumerated, with the paragraph that made it decidable deleted (L3). See
  BLOCKING 2.
- **MINOR H** — §6's table does not name `Received::Closed` (`channel.rs:33`)
  or `Received::Overlong` (`:30`) as rows. The closing total rule covers
  them; hard rule 3 prefers a named refusal per reason, which is
  `tools/SHELL_CHECKLIST.md` item 8's rule applied in Rust.
- **MINOR I** — `--go <go line>` takes a compound value. The `go` line is three
  words (`go nodes <n>`), the existing binary parses flags positionally with
  *"no other spelling"* (`bin/arena.rs:82-99`), and the design does not say
  whether the flag takes one quoted argv element, or the budget kind and value
  as two words built through `BudgetSection::go_line()` as §7's reasoning
  implies. One sentence.
- **MINOR J — a number crept in, and the document forbids it on its own face.**
  This is the dispatch's check 7 and the answer is not clean:

  ```
  $ /usr/bin/grep -oE "[0-9]+" docs/experiments/wp20m_design.md | LC_ALL=C sort -u | tr '\n' ' '
  0 05 1 10 11 17 2 256 283 3 300 4 483 5 5064 527 544 6 7 8 9
  ```

  Against revision 1's set, the additions are `05`/`5064` (the revision SHA),
  `10`/`11` (section numbers), `9`/`17` (gate numbers) — all legitimate — and
  **`283` and `300`**. `283` is a **measured line count** of
  `crates/pistol-arena/src/bin/arena.rs`, and it is load-bearing: §1's argument
  for a new module rests on it. D-483 is flat — *"design documents carry no
  measured numbers — mechanisms, invariants and tests only"* — and the
  document's own header repeats *"**D-483 binds this document** … No numbers"*
  four paragraphs above. It is also the shape hard rule 9 forbids in the
  neighbouring register (*"counts are derived, never asserted"*), and it goes
  stale on the next edit to that file. **FIX:** name the run, not the number —
  *"`wc -l` against `tools/file_justification_check.sh`'s cap leaves no room for
  a third arm."* I grade this MINOR rather than MAJOR because a line count is
  not a number a prereg or gate consumes, which is the harm D-483 was written
  against.
- **MINOR K** — the design does not state that pass 2 determines decidedness by
  replaying prefixes through pistol-core. It is the only way to obey INVARIANT
  2, `transcript::read` already does it, and one clause would say so.
- **MINOR L** — pass 1's arena experiment config (MINOR 6, unapplied) is
  unnamed, and MAJOR 9(b)'s ledger row attaches to the commit that adds it.

---

## Could an implementer build from this without deciding something the design should have decided?

## **No.**

Five decisions, before a line is written:

1. **What "identical engine sections" means** — because the literal reading
   refuses every input the pipeline can produce (BLOCKING A). The implementer
   must choose the comparison, and choosing it is choosing what makes two
   engines the same engine.
2. **Which slot pass 2 spawns and asks** (§3's own title question, open since
   review 1's MAJOR 5).
3. **The capture record's delimiter, field order and quoting** — while
   INVARIANT 6 pins the resulting bytes (MAJOR E).
4. **Where the real-binary re-run test lives** — and the natural answer is the
   one §10 forbids (MAJOR C).
5. **Whether `arena_version` enters `capture_sha256`, and in what spelling the
   engine identity appears** (MAJOR 5).

Decisions 1, 2 and 3 change what the corpus *is*. Decision 4 changes whether the
package's headline invariant is pinned or vacuous — and it is the same defect
class, D-527's, that this arc has now paid for three times.

Two of the prior review's nine implementer-decisions are genuinely retired
(ply-vs-turn, and the initial/terminal position handling), and the budget kind
and failure modes are decided. **The list is shorter and better than revision
1's. It is not empty, and item 1 is new.**

---

## The strongest attack that did not land

**I attacked §4's normalisation as hard as I could, on the dispatch's own
hypothesis — that stripping `nps`/`time` is insufficient, and that gate 9 only
appears to license it because gate 9 compares a different output. Every route
closed, and §4 is the best-verified paragraph in the document.**

Four routes:

1. **Field order — does the solver block split `nps` from `time`?** No.
   `report.rs:62-90`: `solver_field` is interpolated between `nodes` and
   `{NPS_FIELD}`. `nps <n> time <n>` is adjacent and in that order in **both**
   the solver and non-solver spellings, so `determinism.sh:154`'s
   `sed -E 's/ nps [0-9]+ time [0-9]+//'` matches either way. The dispatch's
   named failure mode does not exist.
2. **Is `hashfull` deterministic, making the strip insufficient?** Gate 9
   answers it, and answers it **over the same output**, which was the
   dispatch's doubt. `determinism.sh:218-243` emits
   `newgame\nposition <p>\ngo <budget>\n` per position — pass 2's own loop —
   captures raw stdout, normalises with that one expression, and `diff -u`s two
   full runs. The totals lines are inside the diff; `check_content` greps them
   by name. So `hashfull`, `score`, `pv`, `nodes`, `seldepth` and every solver
   field are all under comparison, and the gate is green in CI. **Not a
   different output — the same one.**
3. **Could the `sed` over-match into the `pv`?** No. `pv` tokens are turn tokens
   (`-?\d+,-?\d+`, and `check_content:164-168` pins that shape), never the
   literal word `nps`.
4. **Could a mangled-but-stable capture satisfy INVARIANT 6 and slip through?**
   Yes — and that is MAJOR B, a defect in the *invariant set*, not in the
   normalisation. The normalisation itself is right.

**Two smaller attacks also failed, recorded so nobody re-runs them:**

- **The capture's `--out` and evidence destruction.** I expected the new mode to
  need its own O_EXCL claim, since a capture corpus costs more machine-hours
  than a report and the existing `--out` refuses an existing file by name
  (D-200). It does not need one: `bin/arena.rs:103` calls
  `outpath::claim(&out_path)` **before** the mode match, so a third arm inherits
  the exclusive claim by structure. The design is silent and correct.
- **§5's exclusion of `source_sha256`.** I re-ran the prior review's own
  strongest attack — that two reports of one experiment could hold different
  games — and it closes the same way (SPRT stop computed over a contiguous
  prefix, aborted reports refused at read, `discarded_in_flight` confined to the
  timing block). §5's premise survives; its residual is MAJOR 5's missing
  instrument version, not its ground.

---

## What I could not settle by reading, and the run that would

- **MAJOR 11's headroom after the edit.** 283 and the 44-line `USAGE` span are
  measured; the third arm's own size is not. **The run:** write the arm and run
  `tools/file_justification_check.sh`.
- **MAJOR C's shell-gate route.** That `tools/arena_smoke.sh` is the only place
  the real `pistol` meets the real `arena` is established by reading; that an
  extension to it would carry the re-run receipt at an acceptable cost is not.
  **The run:** `tools/arena_smoke.sh` at its pre-registered cost, plus one
  capture pass over the report it writes.
- **BLOCKING A's blast radius on a real report.** I established the refusal from
  the types and the two validators. **The run that would show it end to end:**
  `arena --config configs/arena_smoke_v0.toml --out R`, then a pass-2 prototype
  comparing `Transcript::engines[0]` with `[1]` — which refuses, on a report the
  pipeline itself just produced.
