# DECISION-RED-TEAM ROUND 3 — `matrix_anchor_openings_reader.md` REVISION 3

**REVISION READ**: `60359cfbb71c9ffb73b6f2a7ab6ea9cacb4f17e5` (a `git stash create`
object; HEAD `d83ac01`, work uncommitted). **DOES IT MATCH HEAD?** No — HEAD is
`d83ac01` and this document does not exist there. `git diff 60359cf --
docs/experiments/matrix_anchor_openings_reader.md` is EMPTY, so **the working tree
matches the named revision** and everything below was read from the working tree.

**SCOPE.** Round 3 as dispatched: attacks 1–5 only. §1's seventeen-row table is not
re-attacked — round 2 re-ran the grep hit by hit and I take that as settled. The
lockfile row (+7, 22 → 29) is not re-attacked — round 2 re-derived it with `comm` and
I spot-checked the two lockfiles and agree. O4's dilemma is not re-attacked.

---

# ATTACK 1 — THE THREE NEW MEASURED CLAIMS, EACH RUN

## CLAIM 1 — *"`crates/pistol-core/tests/common/sha256.rs` is 92 lines, zero `use` lines, pure std, D-37-licensed, FIPS-pinned"*

```
$ wc -l crates/pistol-core/tests/common/sha256.rs
92 crates/pistol-core/tests/common/sha256.rs

$ /usr/bin/grep -n "^use\|use " crates/pistol-core/tests/common/sha256.rs
(no output; exit 1)

$ /usr/bin/grep -rn "FIPS\|180-4\|ba7816bf" --include='*.rs' crates/
crates/pistol-core/tests/golden_board_tests.rs:30:  "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
crates/pistol-solver/tests/threat_query_tests.rs:37: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
crates/pistol-cli/src/sha256.rs:184:  /// The published FIPS 180-4 vectors (docs/decisions.md D-37, D-60)...
crates/pistol-cli/src/sha256.rs:191:  "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"

$ diff crates/pistol-core/tests/common/sha256.rs crates/pistol-solver/tests/common/sha256.rs
(no output — byte-identical)
```

**92 lines: TRUE. Zero `use` lines: TRUE. Pure std: TRUE** (I read all 92 — two
`const` tables, `sha256_hex`, `sha256`, `compress`; `format!`, `String`, `Vec`,
`chunks_exact`, nothing else). **D-37-licensed: TRUE for the mechanism, and D-37 is
in fact stronger than the matrix uses it.** **FIPS-pinned: FALSE OF THIS FILE.** The
file contains no `#[test]` and no vector. Every existing copy carries its pin in the
CONSUMING test file — `golden_board_tests.rs:22-47` and `threat_query_tests.rs:27-46`,
the latter opening *"A fixture pin computed by an unverified hash pins nothing."*
See MAJOR 2.

## CLAIM 2 — *"first-party lines: 4 542 today, 31 384 under O1, +26 842, a 6.9x increase"*

```
$ find tools/sealbot/matchserver/src crates/pistol-core/src -name '*.rs' -print0 \
    | xargs -0 wc -l | tail -1
4542 total

$ for c in arena cli engine eval search solver; do
    find crates/pistol-$c/src -name '*.rs' -print0 | xargs -0 wc -l | tail -1; done
arena 7056 | cli 6138 | engine 2029 | eval 981 | search 5816 | solver 4822

$ for c in arena cli engine eval search solver; do
    find crates/pistol-$c/src -name '*.rs' -not -path '*/src/bin/*' -print0 \
      | xargs -0 wc -l | tail -1; done
arena 6029 | cli 5223 | engine 2029 | eval 981 | search 5816 | solver 4603

$ find crates -type d \( -name bin -o -name examples \) | LC_ALL=C sort
crates/pistol-arena/examples      crates/pistol-arena/src/bin
crates/pistol-cli/examples        crates/pistol-cli/src/bin
crates/pistol-engine/examples     crates/pistol-eval/examples
crates/pistol-search/examples     crates/pistol-solver/examples
crates/pistol-solver/src/bin

$ python3 -c "..."   # arithmetic, both ways
doc delta 26842  doc total 31384  ratio 6.91
lib delta 24681  lib total 29223  ratio 6.43
bin-only overcount 2161
```

**The arithmetic reproduces exactly; the DENOMINATOR is wrong.** `src` includes
`src/bin`, and a **library** dependency compiles no bin target: `pistol-arena`'s two
declared `[[bin]]`s are 1 027 lines, `pistol-cli`'s auto-discovered bins 915,
`pistol-solver`'s 219 — **2 161 lines O1 would not compile**. `examples/` is outside
`src` and correctly excluded. No double-count. Generated code exists
(`tools/sealbot/matchserver/target/release/build/*/out/private.rs`, 11 lines) and is
correctly excluded by the `src` scope — but the printed command elides that scope.
**Corrected: +24 681, 4 542 → 29 223, 6.43x.** See MAJOR 3.

## CLAIM 3 — *"8 of the 17 refusals have a direct test; nine do not; S1 and S2 have none"*

```
$ /usr/bin/grep -c "^#\[test\]" crates/pistol-arena/tests/openings_tests.rs
10
```

I mapped all ten by hand against §1's A1–A17:

| test | line | refusal reached |
|---|---|---|
| `arena_loads_primary_book_with_digest` | `:10` | accept path only |
| `a_correct_fixture_loads_and_reports_what_it_holds` | `:44` | accept path **+ S1** (`:59-62`) |
| `a_body_that_does_not_match_its_digest_is_refused` | `:66` | **A2** |
| `a_file_with_no_digest_line_is_refused` | `:78` | **A7** |
| `arena_refuses_duplicate_opening_up_to_symmetry` | `:91` | **A13** |
| `a_blank_or_commented_line_in_the_body_is_refused` | `:147` | **A8**, **A9** |
| `a_file_that_mixes_opening_lengths_is_refused` | `:164` | **A14** |
| `taking_more_openings_than_the_file_holds_is_refused` | `:183` | **A17** |
| `openings_skip_yields_disjoint_sample` | `:193` | **A17** |
| `a_turn_cap_that_does_not_clear_the_opening_is_refused` | `:240` | **A16** |

**"8 of the 17" and the SPECIFIC LIST A2, A7, A8, A9, A13, A14, A16, A17: VERIFIED,
exactly right.** **"Nine have none" — the count is right and the LIST IS SHORT BY
ONE.** **"S1 and S2 have none" — FALSE for S1.** See MAJOR 4.

---

# FINDINGS

## FATAL to the recommendation 1 — obligation 2 is the limb the recommendation expressly stakes itself on, and as described it cannot bound the attack it is named to bound, cannot be written where the matrix implies, and would run under no gate

**CLAIM ATTACKED.** §4 obligation 2: *"**A DIFFERENTIAL TEST over both readers** — the
mechanism is O5's dev-dependency — running `pistol-arena::openings::load` and the
matchserver's reader over the committed books and asserting **the same verdict and the
same `Vec<Turn>`**. This is the strongest surviving attack's own remedy and it is
owned here rather than named."* And §4's closing: *"THAT ATTACK IS NOT ANSWERED BY
THIS DOCUMENT. It is answered by obligation 2, and if that test is not written, the
recommendation is not the one this matrix made."*

The recommendation is therefore a compound whose second limb carries the whole of the
strongest surviving attack. Three independent legs of that limb do not hold.

### (a) The committed books cannot exhibit the failure class. The class is "accepts what the arena refuses", and every committed book is accepted.

**SITES.** `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` (2 000 openings,
pinned at `openings_tests.rs:33-36`), `random_openings_v2.txt`, `openings_v1.txt`.

```
$ /usr/bin/grep -rln "body_sha256" --include='*.txt' crates/
crates/pistol-cli/tests/fixtures/bench_positions_v1.txt
crates/pistol-cli/tests/fixtures/openings_v1.txt
crates/pistol-cli/tests/fixtures/random_openings_v1.txt
crates/pistol-cli/tests/fixtures/random_openings_v2.txt
```

A committed book is, by construction, a file the arena LOADS — that is what makes it
committed and what its own document test pins. The weaker-reader class the strongest
attack names is a reader that **accepts a file the arena refuses**. Over inputs the
arena accepts, that class is invisible: not one of the seventeen refusals fires, and
S2's scope rule cannot fire either, because S2 only bites when a defect lies OUTSIDE
the played window and a clean book has no defect anywhere.

**SCENARIO.** An implementer writes obligation 2 exactly as specified. It loads three
committed books through both readers, gets `Ok` from both and identical `Vec<Turn>`,
and goes green. It stays green against a matchserver reader that: dedupes only its
50-opening window; calls `Turn::pair` (`pistol_client.rs:170` already does, in the file
the implementer is working beside) instead of `Turn::from_str` and so silently
canonicalises the `a/b` spelling D-5/D-46 refuses by name; and computes `total` from a
line count without parsing (MAJOR 5). Every one of those is the drift the test was
written to catch. The reviewer signs off *"both readers agree over the committed
books"* — a sentence that is true and that means nothing.

### (b) "the same verdict" is FALSE BY DESIGN against the landed design this matrix serves.

**SITES.** `docs/experiments/anchor_v3_openings_design.md` §2.2: **R5** (*"the opening
does not leave the game UNDECIDED and AT A TURN BOUNDARY"*), **R8** (*"`skip + take >
total`, **or `take < 1`**"*), **R10** (*"`games != 2 * take`"*), **R11**. The arena
implements none of these: `openings.rs` has no `state.outcome()` check outside
`spec.replay()`, and `take = 0` loads an empty window (round 1 recorded this in its
own abandoned list: *"an addition the design makes, not a refusal the matrix
omitted"*). The design REQUIRES the matchserver reader to refuse strictly more than
the arena.

A test asserting *"the same verdict"* therefore either fails on those four classes or
must be scoped to exclude them. **The matrix says neither.** An obligation whose
literal statement is refuted by the design document it is written to serve is not an
obligation an implementer can discharge.

### (c) It cannot live where the matrix implies, and nothing in CI would run it.

**SITES.** `tools/sealbot/matchserver/src/` — `bin/ budget.rs client.rs config.rs
main.rs pistol_client.rs referee.rs report.rs sealbot_client.rs transcript.rs`.
**There is no `src/lib.rs`, and `Cargo.toml` declares no `[lib]`.** `main.rs:11-18`
declares `mod budget; mod client; mod config; mod pistol_client; mod referee; mod
report; mod sealbot_client; mod transcript;` — every module hangs off the BINARY crate
root. A `tests/*.rs` integration test can link only a crate's LIB target, and there is
none, so the differential test **cannot be written as an integration test at all**.
The only home available without restructuring is a `#[cfg(test)] mod` inside the
binary crate.

And nothing executes it. Gate 3 is `cargo test --workspace --locked` (`ci.sh:81`) and
the matchserver is deliberately detached — its `Cargo.toml` says so: *"Deliberately
NOT a workspace member (the empty `[workspace]` table below detaches it) … the
fresh-clone gate builds the workspace only."* Gate 16 is
`tools/sealbot/tests/run_tests.sh` (`ci.sh:170-171`), which drives the shipped
`run_match.sh` — whose only cargo invocation is `( cd "$MS_DIR" && cargo build
--release --locked )` at `:51` — and then runs the built `replay_check` binary.
**`cargo test` appears nowhere in that path.** The design's §8 discharges its CI
obligation with the row *"CI gate 16 green"*.

**WHAT WOULD CLOSE IT.** Re-specify obligation 2 in three parts, none of them a run:
(i) the corpus is ADVERSARIAL, not the committed books — the shapes
`openings_tests.rs` already builds in scratch (tampered body, missing marker,
malformed marker, blank line, body comment, mirrored duplicate, mixed lengths,
window past the end) plus the three drift shapes the strongest attack names
(uncanonical pair spelling, an annotated body line, a defect outside the window),
with the committed books kept as the accept-path control; (ii) the assertion is *the
same verdict except on design §2.2's R5, R8-`take < 1`, R10 and R11, which are named
as licensed divergences and asserted as divergences*; (iii) name the home — a
`#[cfg(test)]` module in the matchserver binary crate, or `src/lib.rs` with the
restructure costed — and add `cargo test --locked` in `$MS_DIR` to
`tools/sealbot/tests/run_tests.sh`, or state that the obligation is undischargeable
by CI and say what replaces it.

---

## MAJOR 2 — the "FIPS-pinned" half of MEASURED claim 1 is false about the file, and three ADRs say the vectors must travel with any copy — so the vendoring row is under-costed and, in the matchserver, unrunnable

**CLAIM ATTACKED.** §1.1: *"vendor `crates/pistol-core/tests/common/sha256.rs` | **92
lines, zero `use` lines, pure std**, pinned against published FIPS 180-4 vectors |
**D-37**"*. And §4: *"~180 lines including the digest"*.

**SITES.** The file has no `#[test]` and no vector (I read all 92 lines). Its pin is
`crates/pistol-core/tests/golden_board_tests.rs:22-47`
(`sha256_matches_published_test_vectors`, four vectors, ~30 lines). The
byte-identical solver copy is pinned the same way at
`crates/pistol-solver/tests/threat_query_tests.rs:27-46`, opening *"A fixture pin
computed by an unverified hash pins nothing."* `crates/pistol-cli/src/sha256.rs:184`
carries the same vectors as a `#[cfg(test)]` unit test.

Three ADRs make the pairing a rule, not a habit. **D-37**: *"pinned by a SHA-256
computed in the test tree from a std-only implementation **that is itself pinned
against the published FIPS 180-4 vectors**"*. **D-91**: *"the pin is computed by a
second copy of pistol-core's test-tree SHA-256, **which carries the FIPS vectors with
it so that a copy that drifted fails on its own**"*. **D-144**: *"The FIPS 180-4
vectors **move with the implementation** as a `#[cfg(test)]` unit test, so the hash
that computes every pin is still itself pinned."* Every existing copy in this
repository obeys it.

**SCENARIO.** An implementer takes §1.1's row literally, copies 92 lines into
`tools/sealbot/matchserver/src/sha256.rs`, and ships. The matchserver now refuses or
accepts books on the word of a digest implementation nothing in this repository has
checked — and by FATAL 1(c) there is no gate that would run a vector test even if one
were written, because gate 16 never invokes `cargo test`. A transposed constant in
the copy makes every book fail its own in-band digest; a subtler one makes two books
collide. `run_match.sh` exits 2 and the anchor never runs, or worse, it runs.

**WHAT WOULD CLOSE IT.** State the vendoring cost as **92 + ~30 lines** (the vectors,
per D-144's standing form), cite D-91/D-144 alongside D-37, and say where the vector
test runs — which is the same gate hole FATAL 1(c) opens. If the answer is "nowhere",
**O6 becomes the only viable route for A2**, not merely its cheapest, and §4's
"or by O6's shell-out" should become "or, if no gate runs the matchserver's tests, by
O6".

---

## MAJOR 3 — the line-count row is round 2's number, reproduced as this document's own MEASURED derivation with the command's scope elided, and it is wrong by 2 161 lines

**CLAIM ATTACKED.** §2: *"**first-party lines the matchserver compiles today** | **4
542** … | **MEASURED** — `find … -name '*.rs' \| xargs wc -l`"*, *"**first-party
lines under O1** | **31 384** … | **MEASURED**, same command"*, *"**+26 842
first-party lines, a 6.9x increase**"*. And the paragraph beneath: *"The line count
was one `find | wc` away and revision 2 did not take it — which is D-291's named
target."*

**SITES.** `matrix_anchor_openings_reader_REDTEAM_round2.md` MAJOR 4 prints, in a
fenced block, `core 2362 | eval 981 | search 5816 | solver 4822 | engine 2029 | cli
6138 | arena 7056` and `-> 2180`, then: *"Gate 16's `cargo build --release --locked`
… today compiles **4,542** first-party lines. Under O1 it compiles **31,384** … add
**26,842**, a **6.9×** increase"*, and its verdict instructs *"Add the compiled-lines
row (4,542 → 31,384)"*.

**Every one of the six figures in revision 3's row is round 2's, unchanged.** They are
presented as this document's own measurement, marked **MEASURED**, with the command
printed as `find … -name '*.rs' | xargs wc -l` — **the ellipsis sitting exactly where
the SCOPE goes.** Revision 1's FATAL was a table transcribed from a review's prose and
dressed as a derivation. Revision 2's failure, in this document's own words, was *"a
grep whose scope excludes the answer"*. This row is both at once, and the receipt that
would let a reader catch it is the one part elided.

And the inherited number is wrong. `src` includes `src/bin`; a library dependency
compiles no bin target. `pistol-arena/Cargo.toml` declares `[[bin]] arena` and
`[[bin]] arena-stub-engine` (1 027 lines); `pistol-cli` and `pistol-solver` carry
auto-discovered bins (915 and 219). **O1 adds 24 681 compiled first-party lines, not
26 842: 4 542 → 29 223, 6.43x.**

**IT DOES NOT FLIP THE OPTION** — 24 681 lines on the shipped path is still the
argument, and I say so plainly. It is a finding because of what it is, not what it
changes: the third number in this arc inherited from a review and marked as derived.

**WHAT WOULD CLOSE IT.** Print the command with its scope (`find crates/pistol-<c>/src
-name '*.rs' -not -path '*/src/bin/*'`), state the corrected figures, and attribute
the row to round 2's MAJOR 4 — attribution costs nothing and is what round 2's own
MAJOR 1 asked for on S1/S2.

---

## MAJOR 4 — §2.1's derivation is wrong in three ways inside one sentence, and one of them is the revision-1 FATAL exactly: a refusal dropped in transcription

**CLAIM ATTACKED.** §2.1: *"mapping each to §1's table gives **8 of the 17 refusals**
a direct test: A2, A7, A8, A9, A13, A14, A16, A17. **Nine have none** — A1, A3, A5,
A6, A10, A11, A12, A15 — and so do **S1 and S2**, the two scope rules."*

**(i) The tested list is right.** I re-derived it from all ten tests (attack 1, claim
3). No dispute.

**(ii) "Nine have none" is followed by a list of EIGHT.** A1, A3, A5, A6, A10, A11,
A12, A15 — count them. `17 − 8 = 9`, so the count is right and the enumeration is
short by one. **The missing row is A4** (`openings.rs:80-81`, *"the body states no
openings"*). It is genuinely untested — nothing in the repository builds a
zero-opening body (`/usr/bin/grep -rn "openings_prefix(0)\|openings_fixture(&\[\])"
crates/` returns nothing) — and it is present in round 2's own list: *"**untested** |
**A1, A3, A4, A5, A6, A10, A11, A12, A15, and S2**"*. **A4 was dropped in
transcription.** A refusal that appears in neither the tested list nor the untested
list is exactly the omission-versus-oversight failure §3's O2 row calls *"the one that
already happened twice in this document"* — and obligation 1 promotes this note into
the design as the implementer's checklist.

**(iii) "S1 … [has] none" is false.** `openings_tests.rs:59-62`:

```rust
assert!(
    !loaded.taken[0].position_tail.contains('#'),
    "the commentary is stripped before the line goes down a pipe"
);
```

That runs over `openings_prefix(6)`, which `tests/common/mod.rs:101-104` builds from
`committed_body()` — the body of `crates/pistol-cli/tests/fixtures/openings_v1.txt`,
whose every body line carries a trailing comment (`/usr/bin/grep -c " #"` → **1 592**;
line 51 is `start moves 0,0 0,1/1,-1 1,0/4,0 2,0/3,-1 # src 0035f32035e5468b elo 1056
games 2 p1 1`). Delete the D-143 strip at `openings.rs:171-176` and that assertion
goes red. **S1 has a direct, non-vacuous test.** Round 2 recorded it as *"S1
(partially, `:58-61` asserts the strip)"*; revision 3 downgraded it to "none" without
a receipt. S2 having none is correct — nothing loads a file whose only defect is
outside the window.

**WHAT WOULD CLOSE IT.** Restore A4 to the untested list (nine: A1, A3, **A4**, A5,
A6, A10, A11, A12, A15), move S1 to the tested column citing `:59-62`, and keep S2
where it is. Then §2.1 says 9 of 18 checked rows tested, and obligation 1's checklist
has all seventeen plus both scope rules accounted for.

---

## MAJOR 5 — "S2's parse limb is FREE under O2" is derived from the implementation being REPLACED, not from the class of readers O2 licenses, so the concession that follows it is incomplete

**CLAIM ATTACKED.** The opening paragraph: *"`total` is `parsed.len()` **after the
loop has parsed every body line** (`openings.rs:75-79`, `:96`), so **any reader** that
refuses `skip + take > total` has already parsed the whole body."* Repeated in §3's O2
row: *"S2's PARSE limb is free — `total` is `parsed.len()` after every body line is
parsed, so a reader that refuses `skip + take > total` has already read the whole
body."*

**SITES.** `openings.rs:75-79` (the parse loop), `:97` `let total = parsed.len();`,
`:98` the refusal. The line citations are off by one (`:96` is the closing brace of
the `turn_cap` block; `total` is at `:97`) — trivial, and not the finding.

**The finding is the quantifier.** In the ARENA, `total` is `parsed.len()`, so the
entailment holds — of the arena. **O2 is a REIMPLEMENTATION**, and a reimplementation
is free to write `let total = body.lines().count();` and refuse `skip + take > total`
without parsing a single line. That reader satisfies A17 by the letter, passes a
row-by-row review of §1's table, and is strictly weaker on identical bytes. The
document derives a property of a reader nobody has written from the code that reader
exists to replace — which is the same shape as deriving a table from a review's prose,
one artifact over.

**SCENARIO.** The anchor points `file` at `random_openings_v2.txt` (D-568 limb 3
reserves a 1 000-opening holdout of it; limb 5 licenses `book_v1` for anchors, so both
are live). `skip = 0, take = 50`. A malformed line sits at index 3 700. The
matchserver's `lines().count()` gives 4 500, `0 + 50 ≤ 4 500` passes, the anchor plays
100 games. A later arena run over the same file refuses it. Two instruments disagree
about whether a committed fixture is a fixture, both citing the same
`# body_sha256` — round 1's scenario verbatim, reached through the limb this revision
declares free.

**WHY IT MATTERS BEYOND THE SENTENCE.** This is one of exactly two "facts that were
wrong" on which §4 rests its reversal. It does not restore O4 — round 2's FATAL 1
dilemma is independent of both facts and this document says so correctly. But the
sentence as written is not derived, and the residual row §3 does concede ("the SCOPE
of the digest and of the symmetry dedup") is therefore short: **the parse limb is a
required row too.**

**WHAT WOULD CLOSE IT.** Rewrite as: *"in the arena `total` is `parsed.len()`, so the
arena's A17 entails a full parse; a REIMPLEMENTATION does not inherit that, so 'parse
the whole body before computing `total`' joins the digest scope and the dedup scope as
a required row."* Three required rows, stated once, pointed at from §4 (D-424 part 3).

---

## MAJOR 6 — "the repository ends with two readers" undercounts to three under the design this matrix serves, and the restructure that would make it two is the horn that killed O4

**CLAIM ATTACKED.** §4: *"THE COST, STATED WITHOUT DECORATION: ~180 lines including
the digest … and **the repository ends with two readers of one book format**."*

**SITES.** Design §5.4 (BLOCKING 3's remedy, which §3's O4 row treats as
non-negotiable): *"`replay_check` therefore gains a second argument — the book path
with its two digests — and asserts, per game: … **the book's line at that number
parses to turns whose replay yields exactly `opening_moves`** … All three are
`pistol-core` work."* That is a third book reader. `src/bin/replay_check.rs:1-9` pins
its independence in its own words: *"The referee and this tool share pistol-core
deliberately: the rules are not the stage under doubt. The stage under doubt is the
RECORD."* Its entire import list is `use std::path::Path;` and `use
pistol_core::{Coord, GameState, Outcome, Player};` — it shares no module with
`main.rs`, and cannot, because the matchserver has no lib target (FATAL 1(c)).

So O2 ends with **three** readers: the arena's, the matchserver referee's, and
`replay_check`'s. The only way to make it two is to add `src/lib.rs` and have
`replay_check` import the referee's reader — at which point the second instrument
shares the code of the thing it checks, which is precisely the horn round 2's FATAL 1
used to kill O4 (*"the second instrument reads the same config as the thing under
test"*). Round 1's MAJOR 4 already established that every option in the field carries
this; revision 3 records it as two.

**WHAT WOULD CLOSE IT.** Say three, and say which of the two horns O2 takes — an
independent `replay_check` reader (the design's own preference, and more lines), or a
shared one (fewer lines, weaker second instrument). Either way ~180 lines is not the
number.

---

## minor 1 — §1.1's three-route table carries three numeric claims and marks none MEASURED or ESTIMATED

*"92 lines, zero `use` lines"*, *"one line"*, *"one line"*. D-291 governs *"a numeric
claim in an option matrix"* without qualification, and round 2's minor 4 already found
two unmarked numbers in revision 2. The first is measurable and I measured it; the two
"one line" figures are estimates and minor 2 says they are low.

## minor 2 — O6's cited precedent computes a different thing from what A2 needs

§1.1: *"shell out to `sha256sum` | one line | **`tools/sealbot/run_match.sh:91` already
does it** for the bytes each seat writes"*. `:91` is `( cd "$ROOT/$OUT_DIR" &&
sha256sum report.json report.txt ./*.jsonl )` — whole-FILE digests of the run's OUTPUT
artifacts, not "the bytes each seat writes". A2 is the digest of `bytes[body_offset..]`
where `body_offset` is the byte just past the FIRST line matching `# body_sha256 `
(`header_digest`, `:127-141`; D-147 fixes the convention in as many words: *"the body
begins at the first byte after the newline ending the `# body_sha256` line"*). In
shell that is a byte-offset split whose offset must be derived by scanning for the
marker, then `tail -c +N | sha256sum`, then a hex compare — not one line, and not the
operation `:91` performs. The route is still viable and still cheapest; the cost row
is wrong and the precedent does not carry it.

## minor 3 — §4's self-description reads round 1's CONDITIONAL as a standing verdict, and drops the condition

§4: *"O2 is not chosen here because a third analysis favoured it; it is where the field
has stood since the first attack, and two revisions of this document were the noise."*
Round 1's verdict is *"**THE OPTION SURVIVES. THE MATRIX DOES NOT — RE-TAKE, MINIMAL
IN SCOPE.**"* and closes: *"I expect option B to survive that re-take. **It should be
recorded as surviving a matrix that was actually right**, not one whose decisive table
was inherited from the review that asked for it."* That is a declined kill plus an
explicit condition — round 1 could not flip the decisive column and said so, which is
not the same as endorsing. The difference matters exactly here: MAJOR 3 and MAJOR 4
show the matrix is still not right, in the same inheritance failure round 1 named, so
the condition round 1 attached is still unmet. "Two revisions were the noise" is a
fair account of the OPTION's history and an unfair account of the DOCUMENT's.

## minor 4 — "the same mistake as the FATAL, one directory deeper" overcharges

Revision 2's quoted claim was that `pistol-core` has no SHA-256, evidenced by a grep
over `crates/pistol-core/src`. At the level of what depending on `pistol-core` GIVES
you, that claim is true and remains true: `tests/common/sha256.rs` is a test-tree
module invisible to every dependent, and D-144 says so in as many words — *"D-37 …
makes pistol-core std-only including dev-dependencies, so that crate's test tree keeps
its own copy and cannot do otherwise"*. What revision 3 found is a **vendoring source**
revision 2 did not think of: a missed option, not a false fact. The revision-1 FATAL
was a table asserting what it had not read. Equating the two inflates the warrant for
the reversal, and the reversal does not need it — round 2 killed O4 on a dilemma
independent of both facts.

---

# ATTEMPTED AND ABANDONED

**"The decisive column is wrong for A10/A11/A12 — they go through `pistol-engine`."**
True of the arena: `openings.rs:6` is `use pistol_engine::PositionSpec` and `:177`/`:190`
call `PositionSpec::from_str` and `spec.replay()`. But the column asks what a
REIMPLEMENTATION needs, and `Turn::from_str` + `GameState::make_turn` +
`state.outcome()` are all `pistol-core`; `pistol_client.rs:251` already calls
`Turn::from_str` in the matchserver. The column holds. ABANDONED.

**"The +7 package row is wrong."** `comm` over the two lockfiles gives exactly
`pistol-arena, pistol-cli, pistol-engine, pistol-eval, pistol-search, pistol-solver,
serde_path_to_error` (`pistol-api` is empty by rule 11 and not a dependency); 22 + 7 =
29. Correct, and round 2 already re-derived it. ABANDONED.

**"Obligation 2 cannot depend on `pistol-arena` across the workspace boundary at all,
because `pistol-arena` uses `version.workspace = true`, `[lints] workspace = true` and
`pistol-core = { workspace = true }`."** Cargo resolves workspace inheritance against
the DEPENDENCY's own workspace root, so a path dependency from the detached matchserver
onto `crates/pistol-arena` resolves, and the two `pistol-core` path entries canonicalize
to one package so `Turn` unifies. Read from the manifests and standard cargo semantics
— **I did not build, per the hard constraint.** The obstacle is the missing lib target
and the missing gate, not manifest inheritance. ABANDONED as a manifest attack; kept as
FATAL 1(c).

**"No committed book exercises the D-143 strip, so obligation 2 cannot cover S1
either."** `random_openings_v1.txt` and `_v2.txt` carry ZERO ` #` body comments
(`/usr/bin/grep -c " #"` → 0 for both). But `openings_v1.txt` carries 1 592, has its
`# body_sha256` at `:50` with the body starting `:51`, and is uniform at 13 turn
tokens across all 1 591 openings (`awk '{print NF-2}' | sort -n | uniq -c` → `1591
13`), so it is a loadable committed book that does exercise the strip. ABANDONED as an
attack on obligation 2; it survives as the receipt that refutes §2.1's "S1 has none"
(MAJOR 4(iii)).

**"§1's table is a third transcription."** Round 2 re-ran the printed grep, opened all
seventeen hits and confirmed bijectivity. I spot-checked A4, A7, A9, A13, A16 and A17
against the code and found them right, with A9's range `:162-168` still off by one at
both ends (round 2's minor 2, unfixed — not re-raised as a finding). ABANDONED, and
§1's table remains the one part of this document that has passed an attack.

---

# THE STRONGEST SURVIVING ATTACK, for the ADR line

> The recommendation is a compound, and its second limb is the whole of its answer to
> the failure class it concedes it does not answer: *"if that test is not written, the
> recommendation is not the one this matrix made."* The test as described cannot be
> written and could not work if it were. Its corpus is the committed books, and the
> class is a reader that ACCEPTS what the arena refuses — a class no accepted file can
> exhibit, so the test goes green against a reader that dedupes only its window, calls
> `Turn::pair` where the arena calls `Turn::from_str`, and counts lines where the arena
> parses them. Its assertion, *"the same verdict"*, is refuted by the design it serves,
> whose §2.2 R5, R8-`take < 1`, R10 and R11 require the matchserver's reader to refuse
> strictly more than the arena's. And it has no home: the matchserver declares no
> library target, so no integration test can reach the reader, while gate 3 skips the
> detached workspace and gate 16 runs `cargo build`, `run_match.sh` and the
> `replay_check` binary and never `cargo test` — so the vendored digest's own FIPS
> vectors, which D-37, D-91 and D-144 all require to travel with any copy, would run
> nowhere either. Naming a mitigation that mitigates nothing is worse than recording
> the cost unmitigated, because it stops the search; this document quotes that sentence
> and then does it, one obligation further down.

---

# VERDICT

**THE OPTION SURVIVES — for the fourth time. THE RECOMMENDATION AS WRITTEN DOES NOT.
BUT THE MATRIX MUST NOT BE RE-TAKEN A THIRD TIME; §4 MUST BE AMENDED.**

**On the option.** Nothing I found moves the field. §4's own flip condition —
*"a refusal that needs an engine, an eval or a search"* — is still unmet; I looked
again and there is none. O1's corrected cost is 24 681 first-party lines on the
shipped path, which is the argument round 2 asked for and it still holds at 6.43x.
O3 is still a re-layering. O4 is still dead on round 2's dilemma, which none of my
findings touches. O5 is still not a substitute. O6 is strengthened by MAJOR 2 — if no
gate runs the matchserver's tests, the shell-out is not merely A2's cheapest route but
its only checkable one. **O2 is the right home for the reader and I could not make it
otherwise.**

**On the document.** Three of my six findings — MAJOR 3, MAJOR 4, MAJOR 5 — are the
same defect this arc has now failed on twice: a claim inherited from somewhere else
and presented as this document's own derivation. MAJOR 3 is round 2's line count with
the scope elided and the bins left in. MAJOR 4 is round 2's untested list with a row
lost and a scope rule mis-copied. MAJOR 5 derives a property of an unwritten reader
from the code it replaces. **A fourth full revision would reproduce them**, because
that is what a fix round that transcribes does, and this arc has the receipts.

**What is required instead is bounded and none of it reopens the option**: amend §4's
obligation 2 per FATAL 1 (adversarial corpus, named divergences, a stated home and a
gate); correct §2.1's two lists (MAJOR 4); correct §2's line figures and attribute
them (MAJOR 3); add the parse limb to the required rows (MAJOR 5); say three readers
and pick the horn (MAJOR 6); state the vendoring cost as 92 + ~30 with D-91/D-144
(MAJOR 2); mark §1.1's three numbers (minor 1) and re-cost the shell-out (minor 2).
§1's table is untouched by all of it.

**And the ADR line's strongest surviving attack must be replaced.** Revision 3
currently records round 1's attack, which it says obligation 2 answers. Obligation 2
does not answer it. The attack above — that the answer itself cannot be written — is
the one that survives, and it must be the one the ADR carries, so that a successor who
writes the differential test knows it has to be adversarial, has to permit the
design's four divergences, and has to have a gate that runs it.
