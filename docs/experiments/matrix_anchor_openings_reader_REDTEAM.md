# DECISION-RED-TEAM — `matrix_anchor_openings_reader.md`

**Revision read**: `5a76a47ef72eee16cf773a70b4a3dae49446835f` (a `git stash create`
object). **Does it match HEAD?** No — HEAD is `d83ac01` and this revision is HEAD
plus the working tree's 19 uncommitted paths, of which
`docs/experiments/matrix_anchor_openings_reader.md` is one (added). The tree did
not move under this review: `git diff 5a76a47e -- .` printed nothing, so every
file below was read from the working tree at the named revision.

**Fresh context; I did not write the matrix.** No `cargo`, no `tools/ci.sh`, no
bench, no match was run — a wall-clock gate is live on this box. Every number
below was obtained with `/usr/bin/grep`, `sed`, `wc`, `comm` and `git` reads.

**Files read in full**: `CLAUDE.md`;
`docs/experiments/matrix_anchor_openings_reader.md`;
`docs/experiments/anchor_v3_openings_design.md` (revision 2);
`docs/experiments/anchor_v3_openings_design_REVIEW.md`;
`crates/pistol-arena/src/openings.rs`; `crates/pistol-engine/src/position.rs`;
`crates/pistol-engine/src/position_token.rs`;
`tools/sealbot/matchserver/src/config.rs`;
`tools/sealbot/matchserver/Cargo.toml`; `crates/pistol-arena/Cargo.toml`;
`crates/pistol-engine/Cargo.toml`; `crates/pistol-cli/Cargo.toml`;
`crates/pistol-{eval,search,solver,core}/Cargo.toml`.
**Read in part**: `crates/pistol-core/src/{turn,symmetry,state,play,lib}.rs` (the
parse, pair, canonical-form and public-surface paths);
`tools/sealbot/matchserver/src/pistol_client.rs:155-200`;
`crates/pistol-arena/tests/openings_tests.rs` (test names + call sites);
`docs/experiments/wp21_prereg.md` §2–§3; `docs/book_v2_ledger.md`;
`docs/experiments/overnight2_ledger.md:160-185`;
`docs/experiments/wp21_DISPATCH.md`; `tools/wp21_tranche_config.py`;
`tools/ci.sh` (the gate list); `tools/sealbot/run_match.sh`;
`tools/sealbot/tests/run_tests.sh` (build path);
`docs/decisions.md` (D-6, D-37, D-51, D-52, D-137, D-143, D-147, D-148, D-175,
D-202, D-291, D-423, D-424, D-505, D-568); `Cargo.lock` and
`tools/sealbot/matchserver/Cargo.lock` (package sets).

---

# FINDINGS

## FATAL to the recommendation — §1's "eight" is a TRANSCRIPTION of REVIEW-design MAJOR 4, not a derivation from the file, and it is materially incomplete; the checklist that is the recommendation's entire mitigation therefore reproduces the defect it was written to prevent

**CLAIM ATTACKED.** §1's preamble: *"Read from `crates/pistol-arena/src/openings.rs`
at the line ranges named, so the comparison below is against a list rather than
against a memory of one."* And §4: *"**OPTION B**, with §1's table promoted into
the design as the checklist a REVIEW-impl reviewer answers row by row."* And §B's
mitigation: *"a REVIEW-impl reviewer checks eight rows and a missing one is a
finding rather than a judgement call."*

**SITE 1 — the transcription.** REVIEW-design MAJOR 4
(`docs/experiments/anchor_v3_openings_design_REVIEW.md:294-302`) enumerates the
same eight refusals in the same order with the same eight line ranges:
`:60-68`, `:156-170`, `:180-187`, `:190-191`, `:200-222`, `:225-245`, `:98-108`,
`:86-95`, carrying the same D-numbers (D-147/D-148, D-137, rule 6's distinct-n).
The matrix's §1 table is that list, reordered into rows R1–R8 and given a fourth
column. Nothing in it was derived from the bytes. This is the shape
`MEMORY.md`'s *"fix rounds derive, never transcribe"* names, and the matrix's own
§1 asserts the opposite in its first sentence.

**SITE 2 — what the transcription inherits.** The reviewer was exhibiting
*"strictly stronger semantics"*, not taking a refusal census. Reading
`crates/pistol-arena/src/openings.rs` directly, the following refusals exist and
have **no row**:

| refusal | site | in the table? |
|---|---|---|
| the file cannot be read (IO) | `openings.rs:58-59` | no |
| the file is not UTF-8 | `openings.rs:125-126` | no |
| a `# body_sha256 ` line that does not carry 64 hex digits | `openings.rs:133-139` | no |
| **NO `# body_sha256` line at all** | `openings.rs:143-151` | **no** |
| the body states no openings | `openings.rs:80-82` | no |
| **the line is not a position at all (`PositionSpec::from_str` fails)** | `openings.rs:177-179` | **no** — R3 cites only `:180-187`, the `else` arm |
| an opening longer than a `u32` turn counter | `openings.rs:242-244` | no |

Seven omissions against eight rows. Two of them are load-bearing:

- **The missing-marker refusal is a separately named behaviour with its own
  test** — `crates/pistol-arena/tests/openings_tests.rs:78`,
  `fn a_file_with_no_digest_line_is_refused()`, distinct from `:66`
  `fn a_body_that_does_not_match_its_digest_is_refused()`. R1 folds two tested
  behaviours into one row. This is exactly D-148's reason for the in-band digest
  (*"a consumer holding only the file … can refuse a corrupted fixture without
  carrying an out-of-band constant"*) and exactly the residue REVIEW-design's own
  R6 rejection flagged (*"a hand-edited copy with no `# body_sha256` line passes
  the design's check"*). A checklist that cannot distinguish them cannot catch the
  omission it was written to catch.
- **The parse refusal at `:177-179` is a different refusal from R3.** `:177` is
  `PositionSpec::from_str` failing (`"not a position: {why}"` — a malformed
  coordinate, a `/` too many, an uncanonical pair spelling); `:180-187` is a
  well-formed `set …` line being refused for being the wrong arm. An implementer
  answering "R3: done" has satisfied one of them.

**SCENARIO.** The design promotes this table into §2.2 as instructed. A
REVIEW-impl reviewer walks eight rows against a matchserver reader that reads
the file, finds no `# body_sha256` line, and — having no row that says otherwise
— proceeds on the config's `body_sha256` alone. The registered `file_sha256`
still passes, because the config carries the digest of the very copy that lost
its header. The anchor runs 100 games from a book whose fixture identity was
never checked, and the reviewer's report says "eight of eight, checked". That is
the *"nobody could tell omission from oversight"* failure the matrix names as
option B's *"THE ONE THAT ALREADY HAPPENED"*, reproduced by the artefact
introduced to prevent it.

**WHAT WOULD CLOSE IT.** Re-derive §1 from `openings.rs` itself — every `return
Err` and every `?` on the load path — rather than from MAJOR 4, and state the
derivation method (a `git grep` receipt at a pinned revision, D-568's standing
form). Expect a table of twelve to fifteen rows. Split R1 into "no marker",
"malformed marker" and "digest mismatch"; split R3 into "not a position" and
"not the `start` arm". Nothing in the recommendation's direction is at stake in
this fix — see the verdict — but the recommendation as written names this table
as its mitigation, and the table as written is not one.

---

## MAJOR 1 — the decisive column's R1 cell is false: SHA-256 is in neither `pistol-core` nor the matchserver's four dependencies, and both of option B's costed claims exclude it

**CLAIM ATTACKED.** §1 row R1: *"the fixture's in-band `# body_sha256` disagrees
with the body | `:60-68` | **yes** — a digest over bytes"*, under the column
*"reachable on `pistol-core` alone?"*. And §2 B: *"**Buys**: the dependency stays
four crates"* and *"**Roughly 80 lines** — ESTIMATED"*.

**SITES.** `crates/pistol-arena/src/openings.rs:61` —
`pistol_cli::sha256::sha256_hex(&bytes[body_offset..])`. `crates/pistol-cli/src/sha256.rs`
is 206 lines. `crates/pistol-core/src/lib.rs:61-90` exports no hash but
`zobrist` (`Key128`, a 128-bit position key — not a content digest).
`tools/sealbot/matchserver/Cargo.toml:15-19` — `pistol-core`, `serde`,
`serde_json`, `toml`; `tools/sealbot/matchserver/Cargo.lock` contains no hashing
package.

The matrix's own prose concedes it one line later — *"Every refusal is
expressible over `pistol-core` plus a digest"* — and then never asks where the
digest comes from. The column says `pistol-core` **alone**; the answer for R1 is
**no**.

**SCENARIO.** An implementer takes option B, reaches R1, and has three moves,
none of them costed:
(i) add `sha2` to the detached workspace — the dependency does not stay at four,
and a detached lockfile gains a supply-chain edge nothing in this repository
vets;
(ii) depend on `pistol-cli` for `sha256_hex` — `crates/pistol-cli/Cargo.toml:24`
takes `pistol-engine`, so this is option A's whole closure arriving through the
back door;
(iii) vendor a fourth std-only SHA-256 (D-37 licenses the pattern — the
implementations at `crates/pistol-core/tests/common/sha256.rs` and
`crates/pistol-solver/tests/common/sha256.rs` are precedent) — which is ~200
lines plus the FIPS 180-4 vector pin D-37 requires, on top of the estimated 80.
And the design's §2 requires **two** digests, so this is not optional.

The estimate being marked ESTIMATED is fine under D-291; what is not fine is
that the estimate's stated derivation — *"from the arena's own `:44-245` less the
parts `PositionSpec` supplies"* — subtracts `pistol-engine`'s contribution and
never adds back `pistol-cli`'s.

**WHAT WOULD CLOSE IT.** Change R1's cell to "no — needs a SHA-256, which lives
in `pistol-cli` (`sha256.rs`), not `pistol-core`", add a row to §3 naming which
of (i)/(ii)/(iii) option B takes and at what cost, and restate the ~80 lines
against that choice. Note for the record that this does **not** flip the
recommendation: a hash needs no engine, eval or search, so §4's flip condition is
untouched.

---

## MAJOR 2 — the one cost row the argument rests on is marked **MEASURED** and is wrong; the true delta is 7 packages, and the row's own parenthetical contradicts its own value

**CLAIM ATTACKED.** §3: *"crates option A adds to the closure | 4 (`pistol-arena`,
`pistol-engine`, `pistol-eval`, `pistol-search`, `pistol-solver` less those
already present) | **MEASURED** — the `Cargo.toml` chain"*. And §3's closing:
*"**What IS measured is the thing the argument rests on**: the dependency
closure, read from the `Cargo.toml` chain."*

**SITES.** `crates/pistol-arena/Cargo.toml:23-29` names **`pistol-cli`** and
`serde_path_to_error` alongside `pistol-core` and `pistol-engine`. The matrix's
§1 preamble and §2 A both spell the chain as *"`pistol-arena → pistol-engine →
{pistol-eval, pistol-search, pistol-solver}`"* — `pistol-cli` appears nowhere in
the document, even though `openings.rs:61`, the very function under discussion,
is what needs it.

**MEASURED, by me, in seconds, without `cargo`** — `comm` over the two committed
lockfiles at the named revision:

```
/usr/bin/grep '^name = ' Cargo.lock            | sed 's/name = //;s/"//g' | LC_ALL=C sort  > ws
/usr/bin/grep '^name = ' tools/sealbot/matchserver/Cargo.lock | ...          | LC_ALL=C sort  > ms
comm -23 ws ms
```
→ `pistol-api  pistol-arena  pistol-cli  pistol-engine  pistol-eval
pistol-search  pistol-solver  serde_path_to_error`.

`pistol-api` is empty by hard rule 11 and is in nobody's closure. The remaining
**seven** packages are exactly what option A adds:
`pistol-arena, pistol-cli, pistol-engine, pistol-eval, pistol-search,
pistol-solver, serde_path_to_error`. The matchserver's closure goes **22 → 29
packages**.

Three defects in one row, then: it says 4; it lists 5 names beside the 4; and the
correct answer is 7. It is marked **MEASURED**. D-291's target is named in as
many words — *"what the sentence forbids is a number that LOOKS measured and is
not, because that is what a red-team cannot attack"* — and its availability test
is *"whether the measurement was AVAILABLE to the session writing the matrix, on
the machine it was standing on, without a build it had not already made"*. Two
committed lockfiles and `comm` is that measurement.

**On the cost UNIT (the brief's question).** "4 crates added" is also the wrong
unit for the argument it serves. The matrix compares a **direct-dependency**
count (4: `pistol-core`, `serde`, `serde_json`, `toml`) against a **closure**
count, which are two different quantities; and the argument it is standing under
— *"a harness … would link the whole engine"* — is about the closure. The
closure is the unit, it is 22 → 29, and it was readable without a build.

**WHAT WOULD CLOSE IT.** Replace the row with the lockfile delta, name the
method, and add `pistol-cli` to every place in the document that spells the
chain.

---

## MAJOR 3 — option B's drift bound is false: R1 bounds WHICH BYTES two readers read and says nothing about WHAT THEY MEAN, and pistol-core hands the matchserver a ready-made way to disagree

**CLAIM ATTACKED.** §2 B, second failure mode: *"the two readers drift, and a
book that loads in the arena is refused by the anchor or the reverse. **Bounded
by R1**: both verify the same in-band digest, so they cannot disagree about which
bytes they read."* Repeated as the recommendation's mitigation in §4: *"The
mitigation is R1 — both verify the same in-band digest over the same bytes."*

The premise is true and it bounds nothing. A digest match is precisely the
condition under which two readers hold **identical bytes**; every drift that
matters happens after that. Three concrete divergences, all reachable on
`pistol-core` alone, i.e. all inside option B's own build:

1. **Uncanonical pair spelling.** The arena parses through
   `PositionSpec::from_str` → `Turn::from_str`
   (`crates/pistol-core/src/turn.rs:209-244`), which **refuses** `1,0/0,0` by
   name (`PAIR_NOT_CANONICAL`, D-5/D-46 — *"accepting … would let two lines that
   differ mean the same turn"*). `Turn::pair` (`turn.rs:119-127`) silently
   **canonicalises** either order. A matchserver reader that splits on `/` and
   calls `Turn::pair` accepts a book line the arena refuses. This is not a
   hypothetical idiom: `tools/sealbot/matchserver/src/pistol_client.rs:169`
   **already calls `Turn::pair`** in exactly this shape, in the file an
   implementer of option B would be working beside.
2. **The trailing-comment rule.** `openings.rs:171-176` strips everything from
   `" #"` onward as commentary (D-143) *inside a body line*, while
   `openings.rs:163-170` refuses a line that *starts* with `#`. That is a
   two-part rule with a subtle boundary and it appears in **no row of §1's
   table**. A reader that omits the strip refuses every annotated book line; a
   reader that strips `#` anywhere accepts a line the arena would refuse.
3. **The SCOPE of the checks.** The arena parses, digest-verifies and
   symmetry-dedupes **the whole file before cutting the window** — `openings.rs:76-83`
   and D-202 in as many words (*"The whole file is still parsed, digest-verified
   and symmetry-deduped before the window is cut, so a defect outside the window
   still refuses the file"*). A matchserver reader that applies R5/R6 to the
   `take`-long window it actually plays is a strictly weaker reader over
   identical bytes. §1's table records the refusals and not their scope, so a
   row-by-row checklist marks that reader compliant.

Every one of these is a case of two readers holding the same bytes, agreeing on
the digest, and returning different verdicts. The bound as stated is a
restatement of the thing that cannot go wrong.

**SCENARIO.** A successor anchor points `file` at `random_openings_v2.txt` — the
sweep's book, and `kind = "book"` accepts any file by design §3's own admission.
The matchserver loads it; a later arena run over the same book refuses it at
`openings.rs:208` for a symmetry collision outside the anchor's window. The two
instruments now disagree about whether a committed fixture is a fixture, both
citing the same `# body_sha256`, and the disagreement is invisible until
something reads both.

**WHAT WOULD CLOSE IT.** Say what the digest actually bounds (identical input
bytes) and state the drift that remains (identical bytes, different verdicts),
then either bound it for real — a test that runs both readers over the committed
books and asserts the same accept/reject and the same `Vec<Turn>`, which is
`pistol-core`-only work in the arena's test tree — or record it unbounded and
let it be the strongest surviving attack. As written the sentence is a mitigation
that mitigates nothing, which is worse than an unmitigated cost because it stops
the search.

---

## MAJOR 4 — the "two readers" concession is misassigned: the design's own §5.4 puts a book reader in `replay_check` and §5.1 pins `replay_check` to `pistol-core`, so EVERY option in the matrix leaves two readers

**CLAIM ATTACKED.** §4: *"**What the recommendation costs and this document does
not hide**: two readers of one file format in one repository, which is the
two-documents-one-claim shape D-423 names."* Presented as the price of B, weighed
against A's dependency weight.

**SITES.** `docs/experiments/anchor_v3_openings_design.md:300-307` (§5.4):
*"`replay_check` therefore gains a second argument — the book path with its two
digests — and asserts, per game: … the book's line at that number parses to turns
whose replay yields exactly `opening_moves`."* And §5.1, `:264-269`:
*"`replay_check` depends on `pistol-core` only. It can consume `[[q,r], …]` …
or `start moves …` tokens through `Turn::from_str` + `make_turn` — but **not** a
`PositionSpec`, which lives in `pistol-engine`."* `replay_check` is
`tools/sealbot/matchserver/src/bin/replay_check.rs`, a second binary in the same
detached workspace, and it is the externally-derived referent BLOCKING 3
required — the whole point of which is that it must **not** share the suspect
input's reader.

So under option A the matchserver reads the book through `pistol-arena` and
`replay_check` reads it through a `pistol-core`-only reader that must still be
written. **Option A does not produce one reader.** It produces two readers built
on different foundations, which is a worse instance of the D-423 shape than B's
two, and the matrix costs A as though it produced one (*"one reader in the tree,
so a refusal added later reaches both callers"* — false: it reaches the
matchserver and not `replay_check`).

Worse, `replay_check` verifying *"the book path with its two digests"* means the
matchserver workspace needs a SHA-256 under **every** option — which is MAJOR 1
again, now unavoidable rather than optional.

**SCENARIO.** The operator reads §4, weighs "the whole engine's dependency
weight" against "two readers", and picks B on that trade. The trade was not
real: both sides of it carry the two readers, and A carries them at greater cost.
A decision recorded on a comparison whose discriminating term is carried by both
options is a decision recorded on nothing — CLAUDE.md's own test, *"where both
sides of a distinction license the same conclusion it is not a distinction"*
(D-424).

**WHAT WOULD CLOSE IT.** Cost `replay_check` explicitly in every option row. B's
honest cost is then *one* new reader module shared by two bins in one crate;
A's is *two* readers on two foundations plus seven packages. Stated that way B
wins by more than the matrix claims — which is why leaving it out is a defect and
not a convenience.

---

## MAJOR 5 — the recommendation promotes a table whose row labels COLLIDE with the labels already in the document it is promoted into

**CLAIM ATTACKED.** §4: *"**OPTION B**, with §1's table promoted into the design
as the checklist a REVIEW-impl reviewer answers row by row."*

**SITES.** The design already carries a refusal table at §2.2
(`anchor_v3_openings_design.md:129-142`) with rows **R1–R11**, and the two
numberings disagree from R5 onward:

| label | matrix §1 | design §2.2 |
|---|---|---|
| R5 | two openings equal up to a lattice symmetry | the opening must leave the game UNDECIDED and at a turn boundary |
| R6 | a file mixing turn counts | two openings equal up to a lattice symmetry |
| R7 | `skip + take > total` | a file mixing turn counts |
| R8 | a `turn_cap` leaving no room for an engine move | `skip + take > total`, or `take < 1` |
| R9 | — | `turn_cap` does not exceed the opening's turn count |

Design §2.3 and §2.4 are titled *"R5 REPLACES…"* and *"R9 REPLACES…"*, so the
design's numbering is load-bearing prose, not a table-local convenience.

**SCENARIO.** A REVIEW-impl reviewer is told "answer the eight rows". They open
the design, find eleven rows, and answer §2.2's R5 (undecided/turn boundary) for
the matrix's R5 (symmetry). The symmetry refusal — the one REVIEW-design MAJOR 4's
own failure scenario is about, *"a successor pointing `file` at any other book
inherits an unguarded 1-1 pair that doubles reported n"* — is marked answered and
was never asked. The mitigation fails silently, in the manner it exists to
prevent.

**WHAT WOULD CLOSE IT.** One numbering. Either the design's §2.2 absorbs the
re-derived §1 rows and the matrix stops carrying its own labels, or the matrix's
rows get a distinct prefix (`M1…Mn`). This is not style: two live meanings for
"R5" in two documents governing one review is precisely D-423's
*"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING."*

---

## MAJOR 6 — option C is dismissed on a speculative risk where a CERTAIN cost was available, and its stated cost is wrong in kind: `openings.rs` cannot move to a crate "on `pistol-core`" at all

**CLAIM ATTACKED.** §2 C: *"**Costs**: a refactor of `pistol-arena` — moving code
that is under test and cited by a landed pre-registration … **A refactor of the
sweep's own reader, on the eve of the sweep.**"* And: *"**Failure mode**: … A
defect introduced by the move is a defect in 3,487 openings' worth of corpus,
found late."* And: *"C is the right answer LATER. It is the wrong answer this
week, and the reason is scheduling rather than design."*

**(a) The premise is TRUE and I verified it.** The sweep does go through this
reader: `tools/wp21_tranche_config.py:116-121` emits `openings_file`,
`openings_take`, `openings_skip`; `crates/pistol-arena/src/bin/arena.rs:231`
calls `openings::load`. Sixteen tranches, 3 487 openings
(`docs/experiments/wp21_prereg.md` §2). This is not risk theatre — the reader is
on the sweep's critical path.

**(b) But "extract a `pistol-openings` crate on `pistol-core`" is not a move.**
`openings.rs` uses `pistol_cli::sha256::sha256_hex` (`:61`) and
`pistol_engine::PositionSpec` (`:6`, `:177`, `:180`, `:190`). A crate depending on
`pistol-core` alone cannot host it. C is therefore a **re-layering** — sha256
descends out of `pistol-cli`, and either the `start moves` grammar descends out
of `pistol-engine` (`position_token.rs:58-99`) or the new crate keeps a
`pistol-engine` edge and buys nothing. The matrix costs C as a file move. That is
a cost asserted, not derived, and it happens to under-state the cost of the
option the matrix rejects — the mirror of D-291(i)'s *"wrong by an order of
magnitude in the recommender's favour"*.

**(c) The scheduling argument is answerable, and the matrix answered the wrong
question.** The sweep is **held**:
`docs/experiments/overnight2_ledger.md:167-175` — *"THE SWEEP IS HELD BY A
STANDING OPERATOR INSTRUCTION … TRANCHE ONE DOES NOT START WITHOUT THE OPERATOR
SAYING SO"* — and three things are owed before the hold is even binding. "On the
eve of" is therefore an intention, not a schedule, and "a defect found late" is
speculative. The **certain** cost was available and is sharper: the sweep's
governing revision is the WP-2.0b closure head, and CLAUDE.md's Process is
categorical — *"A pre-registration is reviewed at the revision that GOVERNS the
run — that revision must itself pass a fresh-context review before the first run
it governs, and reviews of superseded revisions do not transfer."* A C-refactor
landing before tranche one **moves the governing revision and reopens the
review**, which is this arc's most expensive recurring event. That argument kills
C on this week's schedule without needing a hypothetical defect, and the matrix
did not make it.

**(d) And the risk that IS claimed is unsized.** The reader carries 10 behaviour
tests (`crates/pistol-arena/tests/openings_tests.rs`), one of which
(`arena_loads_primary_book_with_digest`, `:10`) loads all 2 000 openings of the
primary book. That is a fact bearing directly on *"a defect introduced by the
move"*, it was one `grep` away, and the matrix asserts the risk without it.

**WHAT WOULD CLOSE IT.** Rewrite C's cost as what C is (a re-layering of sha256
and the position grammar, not a move), and rest its rejection on the
governing-revision rule rather than on an unsized defect risk. C still loses.
The finding is that it loses for a reason the document does not give.

---

## MAJOR 7 — the field is not complete: a fifth option exists, is viable, has in-tree precedent, and is not named

The brief asked for four candidates. My dispositions:

**E1 — `pistol-arena` as a dev/test-only dependency. NOT VIABLE, and it should
not appear in a corrected matrix.** The reader is needed by the shipped
`pistol-matchserver` binary at run time; a `[dev-dependencies]` edge is not
linked into a `[[bin]]`'s normal build. Naming it would be an option that widens
the field without being one — the §8 defect the brief warns about.

**E2 — a vendored copy of `openings.rs` with a CI drift check. WEAK, close to
non-viable.** There is precedent for duplicated code under a mechanical check
(D-37's three std-only SHA-256s pinned to FIPS vectors;
`tools/label_consistency_check.sh` is gate 19/19 and exists solely to keep two
documents consistent). But the copy cannot be byte-identical: it must lose
`pistol_cli::sha256` and `pistol_engine::PositionSpec`, so no textual drift check
is possible and what remains is a **behavioural** equivalence test — which is
what MAJOR 3 says option B needs anyway. E2 is therefore option B plus a name.

**E3 — the anchor pre-computes its openings into the matchserver's own config;
the matchserver reads no book. VIABLE, UNNAMED, and it changes the framing.**
The matchserver already loads an explicit, complete, `deny_unknown_fields` TOML
document (`tools/sealbot/matchserver/src/config.rs:17-33`), and this repository
already generates such documents from a committed generator carrying its own
digest — `tools/wp21_tranche_config.py`, with
`crates/pistol-arena/tests/wp21_tranche_config_tests.rs` driving the shipped
script, adopted for exactly the reason at issue (*"Sixteen near-identical
committed documents is the two-documents-one-claim defect at scale (D-423)"*,
`wp21_prereg.md` §2). A workspace-side generator calls `openings::load` — where
the dependency is **free** — and emits fifty `start moves …` lines plus the book
path, both digests and each line's absolute index. Then R1–R7 are discharged
once, by the reader that already has them, and the matchserver keeps only the
`pistol-core` work it cannot avoid (`Turn::from_str`, `make_turn`, `outcome`,
`phase`) plus the arithmetic guards (turn cap, `games == 2 * take`).
**Its real objection is MAJOR 4's, and the matrix cannot make it because it never
named E3**: design §5.4 still puts a book reader in `replay_check`, so E3 does
not reduce the workspace's reader count below B's — it relocates the eight (or
fifteen) refusals to where they already exist. That is a genuine advantage and a
genuine limit, and neither is on the record.

**E4 — the matchserver becomes a workspace member. VIABLE MECHANICALLY, out of
proportion, and it re-arms a rejection.** `tools/sealbot/matchserver/Cargo.toml:8-13`
detaches it deliberately and says why (*"a comparison harness, not shipped engine
code, and the fresh-clone gate builds the workspace only"*), so this is an
architecture change needing its own ADR, not a reader decision. And note that
REVIEW-design's **R9 rejection is conditional on detachment**: *"`tools/solver_link_check.sh`
enumerates *the workspace's* binaries … and the matchserver is a detached
workspace."* I confirmed the enumeration is workspace-scoped and is driven by
`crates/pistol-cli/tests/solver_link_check_tests.rs:12`, which asserts a fixed
shipped-binary count (`:700`, *"10 shipped binaries"*). E4 **plus** option A
would put a `pistol-solver`-linked binary into the workspace's binary set and
change that count. A corrected matrix should record that R9's rejection does not
survive E4, so a successor does not read it as unconditional.

**WHAT WOULD CLOSE IT.** Name E3 as a fourth option with its buys, costs and the
`replay_check` objection; name E1 and E2 as considered-and-rejected in one line
each so the field is on the record; name E4 with the R9 caveat.

---

## minor 1 — option A's "second failure mode" cannot occur without a change nobody has proposed

§2 A: *"`openings::load` takes `turn_cap` and applies R8 with the ARENA's turn
accounting. The matchserver's cap means the same thing today; nothing pins that
it will."* Both caps count turns from the start of the game and both break the
same way: `openings.rs:86-95` (*"the cap counts from the start of the game"*) and
`tools/sealbot/matchserver/src/referee.rs:161-164` (`state.turn() > turn_cap`).
A failure mode that requires an unspecified future divergence, stated beside
failure modes that are reachable today, pads A's cost column. Say "identical
today; a divergence would be a silent one" or drop it.

## minor 2 — "the only thing `pistol-engine` supplies is `PositionSpec`" omits the grammar

§1: *"The only thing `pistol-engine` supplies is `PositionSpec`, whose `Start`
arm is a ten-line `make_turn` loop plus a won-position check
(`position.rs:78-88`, `:68-73`)."* Both citations check out. But `openings.rs:177`
calls `PositionSpec::from_str`, which is `crates/pistol-engine/src/position_token.rs:58-99`
— the `start` / `moves` / `set` keywords, the "expected `start` or `set`, got X"
refusal, the "`moves` with no turns after it" refusal, and the delegation to
`Turn::from_str` that carries D-56's uncanonical-pair refusal. Option B
reimplements a **grammar**, not just a replay loop, and the grammar is the
`position` verb's protocol contract (D-6). This is the same omission as FATAL 1's
`:177-179` row, seen from the dependency side.

## minor 3 — a refusal in the enumerated range that cannot fire

`openings.rs:70-71` returns "not UTF-8", but `header_digest` at `:60` has already
run `std::str::from_utf8` over the same `bytes` and returned `Err` if it failed.
The second check is unreachable. Not a defect in the matrix's recommendation —
it is evidence for FATAL 1, since a table derived from the file would have met
this line and a table transcribed from a review would not.

## minor 4 — the build-time argument is honest in its reasoning and skips a fact that bears on it

§3's paragraph is right that measuring A's build requires speculatively
implementing A, and right that a build number would decorate rather than test the
argument. But `tools/sealbot/run_match.sh:51` runs
`cargo build --release --locked` in the matchserver directory, and
`tools/ci.sh:170-171` runs the suite that drives it as **gate 16/19**. So option
A's compile weight is not a developer convenience question — it lands on a CI
gate's critical path, on a box whose gate 14 is a wall-clock movetime ceiling.
That does not make the omitted row necessary; it makes *"it would not move the
decision"* a claim the paragraph should defend rather than assert.

---

# ATTEMPTED AND ABANDONED

**"The §1 line ranges are fabricated or mis-cited."** ABANDONED — all eight check
out against the file: `:60-68` is the digest compare, `:156-170` is the blank and
comment refusals, `:180-187` is the non-`Start` arm, `:190-191` is `spec.replay()`,
`:200-222` is `refuse_symmetry_duplicates`, `:225-245` is `uniform_turn_count`,
`:98-108` is `skip + take`, `:86-95` is the turn cap. The defect is
under-inclusion, not misdirection, which is why FATAL 1 is framed as a census
failure rather than a citation failure.

**"A refusal needs `pistol-engine`, so the recommendation flips to A."** ABANDONED
— I looked for this specifically, because it is the matrix's own stated flip
condition and it is what would decide the brief. It is not there.
`Turn::from_str`, `Coord::from_str`, `GameState::make_turn`, `state.outcome()`,
`state.phase()`, `state.played()` and `canonical_form` are all exported from
`crates/pistol-core/src/lib.rs:76-90`. `PositionSpec` contributes a keyword
grammar and a won-position check, both re-expressible over those. The nearest
thing to a flip is MAJOR 1's SHA-256, and a hash is not an engine, an eval or a
search — §4's flip condition is worded to exclude it and, on its own terms,
correctly.

**"Option A would trip the solver-link gate."** ABANDONED — REVIEW-design R9
already disposed of it and I re-verified the disposal:
`tools/solver_link_check.sh` takes a workspace root as argv and is exercised by
`crates/pistol-cli/tests/solver_link_check_tests.rs` against the main workspace
only. It becomes live under E4 and I recorded that under MAJOR 7 rather than as a
finding against A.

**"Slicing `book_v1` is a D-505 breach, so the package is void."** ABANDONED —
D-568's fifth limb licenses anchors explicitly (*"ANCHORS MAY USE `book_v1` …
because an anchor makes no strength claim and so cannot launder a used opening
set into one"*) and design §7.5 cites both. Out of the matrix's scope in any case.

**"The ~80-line estimate is a D-291 finding for being an estimate."** ABANDONED —
D-291 is *"about marking and not about forbidding"*, the line count of unwritten
code cannot be measured, and it is correctly marked ESTIMATED. The live finding is
what the estimate **excludes** (MAJOR 1), not that it is one.

**"The matrix should have run `cargo tree` or `cargo build --timings`."**
ABANDONED — a wall-clock gate was live for that session as it is for mine, and
D-291's availability test is *"without a build it had not already made"*. This is
also why MAJOR 2 survives and this does not: the lockfile comparison needs no
build at all.

**"R5's symmetry refusal is dead weight, since `book_v1` reports
`# derived symmetry_collisions 0`."** ABANDONED — design §3 states that
`kind = "book"` accepts any file, so the refusal is live for any successor, and
dropping it is verbatim REVIEW-design MAJOR 4's failure scenario. The matrix is
right to carry it.

**"`take < 1` is a ninth refusal the matrix missed."** ABANDONED as an attack on
the matrix — design §2.2 R8 adds it, but `openings.rs` does not implement it
(`take = 0` loads an empty window), so it is an addition the design makes, not a
refusal the matrix omitted. Worth a REVIEW-impl note; not a matrix defect.

---

# THE STRONGEST SURVIVING ATTACK

> Option B's stated bound on its own worst failure — *"the two readers drift …
> **Bounded by R1**: both verify the same in-band digest, so they cannot disagree
> about which bytes they read"* — bounds the one thing that cannot go wrong. A
> digest match is exactly the condition under which two readers hold identical
> bytes, and every drift that matters begins there: `pistol-core` exports two
> ways to build a `Turn` from one book token that disagree about the same bytes —
> `Turn::from_str` refuses an uncanonical `a/b` spelling by name (D-5/D-46) while
> `Turn::pair` silently canonicalises it, and the matchserver's own
> `pistol_client.rs:169` already calls the canonicalising one; the arena strips a
> trailing `" #"` comment inside a body line (D-143) under a rule that appears in
> no row of the matrix's table; and the arena parses, digest-verifies and
> symmetry-dedupes the WHOLE file before cutting the window (D-202), a scope the
> table records nowhere, so a matchserver reader that checks only the window it
> plays passes a row-by-row review while being a strictly weaker reader of
> identical bytes. Two readers agreeing on the bytes and disagreeing on what the
> bytes mean is the entire failure class, R1 does not touch it, and naming a
> mitigation that mitigates nothing is worse than recording the cost unmitigated,
> because it stops the search. Nothing about option A escapes this either — the
> design's own §5.4 puts a second book reader in `replay_check` and §5.1 pins it
> to `pistol-core` alone — which is why the honest bound is not a digest but a
> test: both readers, over the committed books, asserting the same verdict and
> the same `Vec<Turn>`.

---

# VERDICT

**THE OPTION SURVIVES. THE MATRIX DOES NOT — RE-TAKE, MINIMAL IN SCOPE.**

I attacked the decisive column head-on and could not flip it: no refusal in
`crates/pistol-arena/src/openings.rs` requires an engine, an eval or a search to
decide whether an opening is admissible, so §4's own flip condition is not met and
option A does not win. Two findings even strengthen B — MAJOR 4 shows A carries
the two-readers cost as well, and at greater weight.

But the recommendation as written cannot be adopted, because the recommendation
is a compound — *"OPTION B, with §1's table promoted into the design as the
checklist"* — and the table is a transcription of REVIEW-design MAJOR 4 that
omits at least seven refusal sites, one of which has its own behaviour test
(`a_file_with_no_digest_line_is_refused`). A checklist that cannot distinguish
"no digest line" from "wrong digest" reproduces the exact defect the matrix
exists to close. Alongside it, the document's only MEASURED cost row is wrong in
three ways at once (7 packages, not 4; five names beside the value 4;
`pistol-cli` absent from every spelling of the chain) — D-291's named target, in
a matrix whose §3 argues at length that measurement discipline is what it is
being careful about.

**What the re-take must do, and it is bounded work, none of it a run**: re-derive
§1 from `openings.rs` at a pinned revision with a stated method (expect twelve to
fifteen rows); correct R1's decisive-column cell to "no — needs a SHA-256, which
is in `pistol-cli`" and cost that choice; replace §3's closure row with the
lockfile delta; cost `replay_check` in every option; give the checklist a
numbering that does not collide with design §2.2's R1–R11; restate C's cost as a
re-layering and its rejection on the governing-revision rule; and name option E3.
I expect option B to survive that re-take. It should be recorded as surviving a
matrix that was actually right, not one whose decisive table was inherited from
the review that asked for it.
