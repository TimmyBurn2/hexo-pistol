# DECISION-RED-TEAM ROUND 2 — `matrix_anchor_openings_reader.md` REVISION 2

**Revision read**: `c5d900a4a764b5b294eb5ff609ac719a56d0e1bf` (a `git stash create`
object). **Does it match HEAD?** No — HEAD is `d83ac0162b6badd539dc940789977c1ba0b43a11`
and this revision is HEAD plus the working tree's 21 uncommitted paths, of which
`docs/experiments/matrix_anchor_openings_reader.md` is one (added). Every file below
was read from the working tree.

**Fresh context. I did not write the matrix and I am not the round-1 reviewer.** No
`cargo`, no `tools/ci.sh`, no bench, no match, no build of any kind was run — a
wall-clock gate is live on this box. Every number below came from `/usr/bin/grep`,
`git grep`, `sed`, `wc`, `find`, `comm`, `awk` and `git` reads.

**THE §1 GREP, RUN BY ME, AND ITS HIT COUNT.**

```
$ /usr/bin/grep -n "ArenaError::\|return Err(" crates/pistol-arena/src/openings.rs \
    | LC_ALL=C sort -t: -k1n
$ /usr/bin/grep -c "ArenaError::\|return Err(" crates/pistol-arena/src/openings.rs
17
```

Hit lines: `59, 63, 71, 81, 87, 99, 126, 134, 143, 157, 164, 178, 181, 191, 208,
230, 243`. **Seventeen. I read every one of them in the file.**

**Files read in full**: `CLAUDE.md`; `docs/experiments/matrix_anchor_openings_reader.md`
(rev 2); `docs/experiments/matrix_anchor_openings_reader_REDTEAM.md` (round 1);
`docs/experiments/matrix_anchor_openings_reader_rev1_SUPERSEDED.md`;
`docs/experiments/anchor_v3_openings_design.md` (rev 2);
`crates/pistol-arena/src/openings.rs`; `crates/pistol-arena/tests/openings_tests.rs`;
`tools/sealbot/matchserver/src/config.rs`; `tools/sealbot/run_match.sh`;
`tools/wp21_tranche_config.py`; all five `Cargo.toml` of the chain plus the root one;
both `Cargo.lock`. **Read in part**: `crates/pistol-core/src/turn.rs` (`pair`,
`from_str`, `first`); `crates/pistol-engine/src/position.rs` (`replay`,
`replay_moves`); `tools/sealbot/matchserver/src/{referee,pistol_client}.rs`;
`tools/sealbot/matchserver/src/bin/replay_check.rs:1-90`; `tools/ci.sh:160-180`;
`docs/decisions.md` (D-6, D-37, D-46, D-51, D-52, D-137, D-143, D-147, D-148, D-199,
D-202, D-291, D-423, D-505, D-568); `.gitignore`.

---

# 0. THE ONE THING THAT MATTERED MOST — IS §1 A SECOND TRANSCRIPTION?

**THE TABLE IS NOT. I CHECKED IT THE WAY THE BRIEF ASKED AND IT HOLDS.** Every one
of the seventeen grep hits has exactly one row, and every row has exactly one hit:

| row | claimed site | grep hit it owns | range says what the row says? |
|---|---|---|---|
| A1 | `:58-59` | 59 | yes |
| A2 | `:60`,`:63-67` | 63 | yes |
| A3 | `:70-71` | 71 | yes (but see **minor 1** — it cannot fire) |
| A4 | `:80-81` | 81 | yes |
| A5 | `:125-126` | 126 | yes |
| A6 | `:133-138` | 134 | yes |
| A7 | `:143-151` | 143 | yes |
| A8 | `:156-161` | 157 | yes, exactly |
| A9 | `:162-168` | 164 | **no — the block is `:163-169`** (minor 2) |
| A10 | `:177-179` | 178 | yes |
| A11 | `:180-187` | 181 | yes |
| A12 | `:189-191` | 191 | yes (`:188-189` is the comment, `:190-191` the code) |
| A13 | `:200-222` | 208 | yes (function span) |
| A14 | `:225-241` | 230 | yes |
| A15 | `:242-244` | 243 | yes |
| A16 | `:86-95` | 87 | yes |
| A17 | `:97-108` | 99 | yes |

Nothing is unrowed and nothing is invented. **The receipt is real and the table is
complete against it.** I also checked the receipt's own completeness: the four `?`
propagations at `:60`, `:78`, `:83`, `:85` all resolve into rowed callees; the
`drain` at `:109` cannot panic given `:80` and `:98`; `uniform_turn_count`'s
`openings[0]` at `:226` is guarded by `:80`; `replayed`'s `unreachable!` at `:254` is
a panic on an already-validated value, not a refusal. **There is no eighteenth
refusal.**

**BUT THE TWO ITEMS THE RECOMMENDATION ACTUALLY TURNS ON ARE NOT IN THAT GREP AT
ALL**, and they are transcribed. See **MAJOR 1**.

---

# FINDINGS

## FATAL to the recommendation 1 — O4's headline "Buys" and O4's compliance with the design it serves are MUTUALLY EXCLUSIVE: it either keeps a book reader and a SHA-256 in the matchserver workspace and buys nothing, or it dissolves the remedy REVIEW-design raised as BLOCKING 3 and reopens the exact defect that remedy exists to catch

**CLAIM ATTACKED.** §3 O4: *"**The matchserver reads no book.**"* and *"**Buys**:
**ONE reader of the book format in the repository.** No new dependency (22 stays 22).
**No second SHA-256.**"* And §3 O4's second failure mode, which is where the document
notices the problem and then walks past it: *"the binding from game to BOOK LINE
becomes a property of the generator's receipt rather than of a run-time check. The
design's §5.4 binding then reads against the config's own list, and the book binding
is re-derivable rather than re-checked."*

**SITES.**
- `docs/experiments/anchor_v3_openings_design.md:292-307` (§5.4, titled *"…WHICH IS
  BLOCKING 3"*): *"`replay_check` therefore gains a second argument — **the book path
  with its two digests** — and asserts, per game: the transcript's `opening_line` is
  `skip + (g-1)/2`; **the book's line at that number parses to turns whose replay
  yields exactly `opening_moves`**."*
- `anchor_v3_openings_design.md:296-299`, the defect class §5.4 exists for: *"**A
  window off-by-one gives 100 games on one opening, exit 0, and a report echoing
  `skip` and `take` from the config.**"*
- `anchor_v3_openings_design.md:316-318` (§6, dry-run limb 1, a REGISTERED criterion):
  *"every game's transcript replays under `replay_check` **with the book argument**,
  exit 0 — which is §5.4's binding, exercised."*
- `tools/sealbot/matchserver/src/bin/replay_check.rs:34-42` — it takes one argument
  today, an artifacts directory, and reads no digest of anything.
- `anchor_v3_openings_design.md:100-107` (§2): *"**Neither digest alone is the
  check.**"*
- `anchor_v3_openings_design.md:141` (§2.2 R11): a refusal whose whole content is
  *"a document naming values nothing reads"*.

**THE DILEMMA, both horns concrete.**

*Horn A — O4 keeps §5.4.* `replay_check` still opens the book, still finds body line
`skip + (g-1)/2`, and still verifies *"the book path with its two digests"*. To do
that it must split header from body (A7/A6's rule), skip nothing and strip nothing
wrongly (S1), and compute SHA-256 twice. **The matchserver workspace therefore
contains a book reader and a SHA-256 under O4 exactly as under O2.** Every one of
O4's three "Buys" bullets is then false, and O4 is O2 plus a generator.

*Horn B — O4 amends §5.4 to read against the config, which is what the matrix
proposes.* Then `replay_check` and the matchserver read **the same document**, and
BLOCKING 3's remedy is gone: a generator with a window off-by-one writes fifty wrong
openings into the config, the matchserver plays them, `replay_check` compares the
transcripts against that same config, agrees, and exits 0 — *"100 games on one
opening, exit 0, and a report echoing `skip` and `take` from the config"*, verbatim,
with the second instrument now unable to see it. The matrix's stated mitigation —
*"re-running the generator reproduces it byte for byte"* — is a REPRODUCIBILITY
check, not an INDEPENDENCE check: re-running a generator that has an off-by-one
reproduces the off-by-one. And design §6 limb 1 is then a registered dry-run
criterion the shipped binary cannot satisfy as written — **the same shape as
BLOCKING 1's go-line read-back, which §6's own closing paragraph exists to record**.

Horn B also strands §2: with the matchserver reading no book and `replay_check`
reading no book, `body_sha256` and `file_sha256` sit in the config verified by
nobody. §2 says *"Neither digest alone is the check"*; under horn B neither digest is
a check. That is `kind = "book"` carrying keys nothing reads — **the precise
condition §2.2's R11 refuses for `platform_standard`**, reintroduced by the
recommended option on the other arm of the same enum.

**WHAT WOULD CLOSE IT.** State which horn O4 takes, in the matrix, before selection.
On horn A, delete the three "Buys" bullets and re-run the comparison — O4 is then
strictly O2 plus a generator and the recommendation must be re-argued. On horn B,
cost the amendment as what it is: a re-opening of a BLOCKING finding and of §6's
registered limb 1, with a replacement second instrument named (the only obvious one
is `replay_check` reading the book anyway, which is horn A).

---

## FATAL to the recommendation 2 — §4 says the recommendation changed *"because two derived facts changed"*, and NEITHER fact survives: S2 costs O2 nothing at all, and A2's cost is inflated by a `grep` scoped to exclude the repository's own 92-line std-only SHA-256

**CLAIM ATTACKED.** §4: *"**OPTION O4.** And it changed from revision 1's O2 because
**two derived facts changed**, not because a newer option looked better: 1. **A2 needs
a SHA-256 the matchserver does not have and `pistol-core` does not provide.** O2's
cost was understated by an entire digest implementation. 2. **S2 exists** … Any
option that puts a reader in the matchserver either reproduces that or is a weaker
reader of identical bytes — and O4 is the only option that does not have to reproduce
it."* These two sentences are the entire stated warrant for changing the
recommendation. Both fail.

**(a) S2 IS FREE UNDER O2. IT IS NOT A COST; IT IS A CONSEQUENCE OF A REFUSAL O2 MUST
IMPLEMENT ANYWAY.**

SITES: `crates/pistol-arena/src/openings.rs:76-79` (the loop parses *every* body
line), `:97` (`let total = parsed.len();`), `:98-108` (A17, `skip + take > total`),
`:83` and `:85` (symmetry and turn-count checks run over `parsed`, i.e. all of it),
`:109` (the window is cut last). `anchor_v3_openings_design.md:138` makes A17 a
required refusal of every option (R8).

A reader that can refuse `skip + take > total` **has already parsed every line of the
file**, because `total` is not knowable otherwise. Validating those lines is then the
default; producing the "strictly weaker reader" the matrix fears requires an
implementer to go out of their way — parse the window, then separately count the rest
without checking it. **S2 is what O2 gets for free the moment it satisfies R8.** The
matrix's sentence *"either reproduces that or is a weaker reader"* is true and empty:
reproducing it costs zero lines. Presenting a zero-cost property as the fact that
moved a recommendation is a cost asserted, not derived.

**(b) A2's COST IS INFLATED, AND THE GREP THAT ESTABLISHES IT IS SCOPED TO MAKE IT
SO.**

SITE: §1.1, *"`pistol-core` has no SHA-256 (`/usr/bin/grep -rn "sha256\|Sha256"
crates/pistol-core/src` is empty)"*. The scope `crates/pistol-core/**src**` is doing
load-bearing work. Widen it by one directory level:

```
$ /usr/bin/grep -rln "sha256\|Sha256" crates/pistol-core | LC_ALL=C sort
crates/pistol-core/tests/common/mod.rs
crates/pistol-core/tests/common/sha256.rs      <-- 92 lines, std-only
...
$ find . -name sha256.rs -not -path './target/*' | xargs wc -l
  206 ./crates/pistol-cli/src/sha256.rs
   92 ./crates/pistol-core/tests/common/sha256.rs
   92 ./crates/pistol-solver/tests/common/sha256.rs
```

**The repository holds two 92-line std-only SHA-256 implementations and an ADR that
licenses exactly this pattern.** D-37 (`docs/decisions.md:90`): *"pinned by a SHA-256
computed in the test tree from a **std-only implementation that is itself pinned
against the published FIPS 180-4 vectors** — pistol-core takes no dependency,
dev-dependencies included"*. The matchserver is, by its own `Cargo.toml:8-12`, *"a
comparison harness, not shipped engine code"* — squarely the shape D-37 licenses.

§2 O2's costed line is *"**ESTIMATED ~280 lines** with a digest"*, which is 80 + 206,
i.e. `pistol-cli`'s copy. The cheapest available copy is 92 lines. The honest number
is ~172. **The estimate is high by ~60%, in the recommender's favour, and the number
that makes it high was excluded by the scoping of a grep** — in a document whose
opening paragraphs are about greps beating prose. This is D-291(i)'s recorded shape:
*"wrong by an order of magnitude in the recommender's favour, with both measurements
available in eleven seconds."*

**(c) AND THE DECISIVE COLUMN IS READ TWO WAYS AT ONCE.** For A2 the cell reports
where **the arena's implementation** gets the capability (`pistol_cli::sha256`,
`openings.rs:61`). For A10, A11 and A12 the cell reports where **a hypothetical
reimplementation** would get it (*"no — `Turn::from_str`"*, *"no — `make_turn` +
`state.outcome()`"*) — while the arena's own implementation of all three goes through
`PositionSpec` (`openings.rs:6`, `:177`, `:180`, `:190`), which is `pistol-engine`.
Under the arena-implementation reading the answer is **four** exceptions, not one.
Under the reimplementation reading A2's answer is *also* "no" — a 92-line std-only
digest is re-expressible with zero dependencies, exactly as the `start moves` grammar
is. **On neither consistent reading is "15 of 17, and the exception is A2" the
answer**, and that row is marked **MEASURED**.

**WHAT WOULD CLOSE IT.** Withdraw reason 2 outright. Re-cost reason 1 at 80 + 92 with
D-37 cited, and state which reading the decisive column takes, applied to all
seventeen rows. Then re-argue the change from O2 — or don't make it.

---

## MAJOR 1 — S1 and S2 are TRANSCRIBED from the round-1 red team, are not findable by the printed grep, and are cited at PROSE rather than at code — one of them at a `///` doc comment, where the red team had correctly cited the implementation

**CLAIM ATTACKED.** §1's framing sentence: *"**Revision 2's §1 is derived by grepping
the file for its refusal sites and reading each one**, and the grep is printed so a
reader checks the derivation rather than the conclusion."* And §1's *"AND TWO RULES
THAT ARE NOT REFUSALS AND APPEAR IN NO ROW OF REVISION 1, both load-bearing"*.

**SITES.** Neither `" #"` nor the parse-then-cut ordering matches
`ArenaError::\|return Err(`. **S1 and S2 appear in zero of the printed grep's
seventeen hits**, and the document states no second method by which they were found.
They came from round 1:

| | round-1 red team (MAJOR 3) | matrix rev 2 §1 | which cites code? |
|---|---|---|---|
| comment strip | *"`openings.rs:171-176` strips everything from `" #"` onward … (D-143)"* | **S1, `:170-174`** | round 1. `:171` is a `//` comment; `:176` (`.trim()`, part of the rule) falls outside the matrix's range |
| scope | *"`openings.rs:76-83` and D-202 in as many words"* | **S2, `:50-53`** | round 1. `:50-53` are four lines of the `///` doc comment on `load`; the code is `:76-79`, `:83`, `:85`, `:109` |

Same two items, same order, same two D-numbers, same "load-bearing" framing — and
the citations moved *away* from the code. **S2, the item §4's reason 2 rests on
entirely, is cited at a doc comment.** D-568's standing law is quoted in this
document's own second paragraph: *"a comment asserting a property about call sites is
not evidence of that property."*

The claims themselves are TRUE — I verified S2 against `:76-109` and S1 against
`:171-176` — which is why this is MAJOR and not a second FATAL. But the document's
first-page promise is that its §1 is derived by a printed method, and the two items
that decide the recommendation were not.

**WHAT WOULD CLOSE IT.** Cite S1 at `:171-176` and S2 at `:76-79`, `:83`, `:85`,
`:109`; state the method by which a non-refusal scope rule is found (it is not the
printed grep); and attribute both to round 1's MAJOR 3, which is where they came
from.

---

## MAJOR 2 — "22 stays 22" is false the moment O4 discharges the test §4 concedes it owes, and O5 was struck from the field as a SUBSTITUTE when it is the MECHANISM that test needs

**CLAIM ATTACKED.** §3 O4: *"No new dependency (22 stays 22)."* §3 O5: *"**NOT
VIABLE and named so the field is not padded.** The reader is needed by the shipped
`run_match.sh` path, not by tests. A dev-dependency does not reach it."* §4's quoted
attack: *"**The honest bound … is a test that runs both parsers over the committed
books and asserts the same verdict and the same `Vec<Turn>`.** O4 owes that test
exactly as O2 would have."*

**SITES.** `tools/sealbot/matchserver/Cargo.toml:13` (`[workspace]`, detached);
`tools/sealbot/run_match.sh:47` (*"no Cargo.lock under $MS_DIR; generate and commit
one"*) and `:51` (`cargo build --release --locked`).

**THE MECHANICS.** `Cargo.lock` records the whole dependency graph, dev-dependencies
included. The only way to run the arena's reader and the matchserver's parser in one
process is a `[dev-dependencies] pistol-arena` edge in the detached workspace — which
is O5's mechanism. **That edge moves the matchserver's committed lock from 22 to 29
packages**, the exact number O4's cost column claims it avoids. The release binary's
link set stays small; the MEASURED row that decides the matrix does not.

So O5's one-line dismissal disposes of the wrong question. It is correct that a
dev-dependency cannot supply the *shipped* reader. It is not "padding the field" —
it is the *complement* every remaining option needs to discharge the obligation §4
concedes, and striking it as a substitute removes the answer to a problem the same
document raises three sections later.

**AND UNDER O4 THAT TEST MAY NOT BE CONSTRUCTIBLE AT ALL.** O4's second parser never
reads the book; it reads `start moves …` strings out of a config. "Both parsers over
the committed books" does not typecheck for O4. The nearest honest test is *"the
generator's config, parsed by the matchserver, yields the same `Vec<Turn>` as
`openings::load` did"* — which needs both crates in one process (the dev-dependency
again, 22 → 29) **or** a shell-level comparison in `tools/sealbot/tests/run_tests.sh`
that the matrix does not name, cost, or assign.

**AND THE OBLIGATION IS UNOWNED AND CONTRADICTS THE COST COLUMN.** §4 says O4 *"owes
that test"*. §3 O4's Costs bullet says only: *"a generator plus its coverage test …
The config grows fifty lines."* One document, two accounts of what O4 costs — D-423's
named shape. `anchor_v3_openings_design.md:367-376` (§8, the obligations table) has no
row for it either.

**WHAT WOULD CLOSE IT.** Restore O5 to the field as a dev-dependency COMPLEMENT with
its own row; state the lock delta it costs (22 → 29) and that the release binary is
unaffected; put the equivalence test in O4's Costs bullet and in design §8's
obligations table, naming where it lives.

---

## MAJOR 3 — the field is still short: two options the round-1 red team told this matrix to name are absent, and a sixth option kills reason 1 outright and is unnamed

**CLAIM ATTACKED.** §3's implicit claim of completeness, and §3 O5's *"named so the
field is not padded"*. Round 1's MAJOR 7 closed with: *"name E1 and E2 as
considered-and-rejected in one line each so the field is on the record; name E4 with
the R9 caveat."* Revision 2 named E1 (as O5) and E3 (as O4). **E2 and E4 are absent.**

**E2 — a vendored copy of the reader with a mechanical drift check.** Round 1 graded
it WEAK; that is a disposition, and a disposition belongs on the record. Precedent
exists (`tools/label_consistency_check.sh` is gate 19/19 and exists only to keep two
documents consistent; D-37's three std-only SHA-256s).

**E4 — the matchserver becomes a workspace member.** Absent, and with it round 1's
finding that REVIEW-design's **R9 rejection is conditional on detachment** — so a
successor reading R9 will read it as unconditional. That is the silent-drift shape
hard rule 10 names.

**AND AN OPTION NOBODY HAS NAMED — O6, THE DIGEST MOVES TO THE SHELL.**
`tools/sealbot/run_match.sh` **already** reads the config with `tomllib` (`:62-71`),
already guards the values it reads (`:75-84`), and already computes SHA-256 by
shelling out (`:91`, `sha256sum report.json report.txt ./*.jsonl`). Fifteen lines in
that script — read `[openings] file`, `body_sha256`, `file_sha256`; verify with
`sha256sum`; `fail` on mismatch — discharge **A2 with no Rust digest anywhere**, and
`run_match.sh` is the shipped path CI gate 16 drives (`tools/ci.sh:170-171`).

**O2 then costs ~80 lines again, and §4's reason 1 evaporates.** O6 has a real
objection — a caller who runs the binary directly skips the check, which is a hard
rule 3 shape — and that objection is exactly the kind of thing an option matrix is
for. It was never put.

**WHAT WOULD CLOSE IT.** Name E2 and E4 with one line each and the R9 caveat; name
O6 with its buys (A2 discharged, 22 stays 22, no new Rust, precedent at `:91`) and
its cost (the guard lives outside the binary); and re-derive §4 with reason 1 costed
against O6 rather than against the 206-line copy.

---

## MAJOR 4 — the decisive MEASURED number is the wrong cost, and the paragraph that refuses the right one is unchanged from revision 1 despite a round-1 finding against it

**CLAIM ATTACKED.** §2: *"**delta** | **+7 packages** | **MEASURED**"*, and §2's
closing: *"**STILL NO BUILD-TIME ROW, and the reason is unchanged** … The argument
against A is not build time."*

**THE +7 IS ARITHMETICALLY RIGHT — I re-derived it**:

```
$ /usr/bin/grep -c '^name = ' Cargo.lock                             -> 26
$ /usr/bin/grep -c '^name = ' tools/sealbot/matchserver/Cargo.lock   -> 22
$ comm -23 <workspace names> <matchserver names>
pistol-api pistol-arena pistol-cli pistol-engine pistol-eval pistol-search
pistol-solver serde_path_to_error
```
`pistol-api` is empty by hard rule 11; the other seven are exactly O1's addition, and
every workspace external except `serde_path_to_error` is already in the matchserver
lock at the same version (I checked `serde 1.0.229`, `toml 1.1.4+spec-1.1.0`,
`winnow 1.0.4`; only `syn` differs, 3.0.3 vs 3.0.4, which resolution absorbs). **22 →
29 is correct.**

**AND IT IS THE WRONG UNIT FOR THE ARGUMENT IT CARRIES.** Six of the seven are
first-party path crates in this repository; the seventh, `serde_path_to_error 0.1.20`,
is already in the workspace lock at that version. **O1 introduces zero unvetted
third-party code.** The supply-chain reading of "+7 packages" — the reading that makes
the number feel like a cost — is not available. What O1 actually costs is compile
work on a CI gate's critical path, and that was measurable in seconds:

```
$ find crates/pistol-<c>/src -name '*.rs' | xargs wc -l   (per crate, src only)
core 2362 | eval 981 | search 5816 | solver 4822 | engine 2029 | cli 6138 | arena 7056
$ find tools/sealbot/matchserver/src -name '*.rs' | xargs wc -l   -> 2180
```

Gate 16's `cargo build --release --locked` (`run_match.sh:51`, `ci.sh:170-171`) today
compiles **4,542** first-party lines. Under O1 it compiles **31,384** — `pistol-eval`,
`-search`, `-solver`, `-engine`, `-cli`, `-arena` add **26,842**, a **6.9×** increase,
with `pistol-search` (5,816 lines) and `pistol-solver` (4,822) in release. That is a
`find | wc -l`, no build, D-291's *"available … on the machine it was standing on,
without a build it had not already made"* met exactly.

Round 1's minor 4 named the same fact (gate 16 builds the matchserver; gate 14 is a
wall-clock movetime ceiling) and said *"it would not move the decision" is a claim the
paragraph should defend rather than assert*. Revision 2's paragraph is revision 1's
with the sentence deleted rather than defended. **The one MEASURED row that decides
between O1 and O4 measures a proxy that costs nothing real, while the quantity that
would decide sits behind a `find`.**

**WHAT WOULD CLOSE IT.** Keep the +7 row and label what it is (a link-set count, not
a supply-chain delta). Add the compiled-lines row, MEASURED. Then say whether the
argument against O1 is build time, and defend it either way.

---

## MAJOR 5 — "already tested" is a property asserted about the reader without a receipt, and the receipt refutes it for nine of seventeen refusals including two the recommendation names

**CLAIM ATTACKED.** §3 O1: *"**Buys**: all seventeen refusals and both scope rules
S1/S2, already reviewed, **already tested**."* §3 O4: *"A2, A6, A7, A13, A14, A17, S1
and S2 are performed **once**, by the reader that is already reviewed **and tested**."*

**SITES.** `crates/pistol-arena/tests/openings_tests.rs` holds eleven `#[test]`
functions (`:10, :44, :66, :78, :91, :147, :164, :183, :193, :240` plus the `mirror`
helper at `:119`). Mapping them onto the seventeen rows:

| tested | A2 (`:66`), A7 (`:78`), A8/A9 (`:147`), A13 (`:91`), A14 (`:164`), A16 (`:240`), A17 (`:183`,`:229`), S1 (partially, `:58-61` asserts the strip) |
|---|---|
| **untested** | **A1, A3, A4, A5, A6, A10, A11, A12, A15, and S2** |

**A6 — the "64 hex digits" marker-shape refusal — has no test at all**
(`/usr/bin/grep -n "hex\|BODY_SHA_MARKER\|64" crates/pistol-arena/tests/openings_tests.rs`
returns only the digest constant at `:34`). **S2 has no test at all**: nothing in that
file loads a file whose only defect is outside the window. Both are named in O4's own
"Buys" bullet as things the reader already performs *and is tested for*.

This is the same object D-568 governs — *"a comment asserting a property about call
sites is not evidence of that property"* — with "already tested" in place of the
comment. The correct receipt was one `grep` over the test file, and it says 8 of 17.

**WHAT WOULD CLOSE IT.** Replace "already tested" with the count and the list, and
note in O1's row that adopting the arena's reader adopts nine untested refusals along
with eight tested ones.

---

## MAJOR 6 — under O4 the anchor's own report names a book BY CONTENT that nothing in the run ever opened, and the openings it played exist only in an untracked file

**CLAIM ATTACKED.** §3 O4: *"**The matchserver reads no book.** … writes the anchor's
config with the openings in it verbatim, plus the book path, its `body_sha256`,
`skip`, `take`."* (Brief item 6: does O4 make the anchor less readable?)

**SITES.** `docs/decisions.md:326` (D-147): the header identifies the corpus *"by
SHA-256 AND BY NOTHING ELSE"* so a run names the book it played from by CONTENT.
`crates/pistol-arena/src/openings.rs:35-37`: `body_sha256` is *"echoed into the report
so a run names the book it played from by content rather than by path (D-147)"*.
`anchor_v3_openings_design.md:286-290` (§5.3): `distinct_openings_played` is
*"computed from the transcripts' own `opening_line` values, and reported as '50 of 50,
each played twice'"*. `.gitignore:35-36`: `/local/*` with a single exception for
`local/sealbot.example.toml` — **the anchor's config is untracked**, and
`git ls-files | grep -i sealbot` confirms no committed match config anywhere.

**THE READABILITY DEFECT.** Under O1/O2 the report's `body_sha256` is a digest the
run computed over bytes the run read: the sentence *"played from `random_openings_v1.txt`,
body `7b1b3a99…`, lines 0–49"* is a measurement. Under O4 that same sentence is a
string the generator copied into a config, echoed by a process that never opened the
file. **A report that names a fixture by content-digest it did not compute is making
a claim of a kind D-147 exists to make honest.** `distinct_openings_played` degrades
the same way: it counts distinct `opening_line` values that the generator wrote and
the matchserver copied, so "50 of 50" restates the config rather than measuring the
run — and with §5.4 amended (FATAL 1, horn B) nothing else checks it.

**AND REPRODUCTION MOVES OFF COMMITTED BYTES.** The fifty openings the anchor plays
would live only in a gitignored `local/*.toml`. Their sole identity is a digest
printed by a generator. That is not forbidden — hard rule 8 lets a manifest sha-index
an artifact — but it is a cost of O4 that appears nowhere in O4's cost column, and it
is a real change from O1/O2, where the openings are lines of a committed, sha-pinned
fixture the run reads.

**WHAT WOULD CLOSE IT.** Add to O4's costs: the report's book identity becomes
transcribed rather than computed, and the played openings live in an untracked
document. Then say what makes the report's `body_sha256` true at run time — or drop
it from the report, which is R11's own logic.

---

## minor 1 — A3 cannot fire, round 1 said so, and revision 2 promoted it to a row without noticing

`openings.rs:70-71` returns *"not UTF-8"*, but `header_digest` at `:60` has already
run `std::str::from_utf8` over the same `bytes` (`:125`) and returned `Err` if it
failed. **A3 is unreachable.** Round 1's minor 3 named it and called it *"evidence for
FATAL 1, since a table derived from the file would have met this line"*. Revision 2's
table met the line and recorded it as a live refusal. So *"Seventeen refusal sites"*
over-counts the reachable ones by one, and §2's **MEASURED** row *"15 of 17"* is 14 of
16 in refusals that can happen.

## minor 2 — A9's line range is off by one at both ends

The comment-in-body block is `openings.rs:163-169` (`if raw.starts_with('#') {` at
`:163`, `));` at `:169`). The row says `:162-168`; `:162` is the closing brace of the
blank-line check and `:168` is mid-string. A8 immediately above is exact (`:156-161`),
which is what makes this a slip rather than a convention.

## minor 3 — O1's second failure mode still cannot occur, one round after being told so

§3 O1: *"`load` also takes `turn_cap` and applies A16 with the ARENA's turn accounting;
nothing pins that the two stay the same."* Both caps count turns from the start of the
game and break identically: `openings.rs:86-95` (*"the cap counts from the start of
the game"*) and `tools/sealbot/matchserver/src/referee.rs:162` (`if state.turn() >
turn_cap`). Round 1's minor 1 said this pads the rejected option's cost column with a
divergence nobody has proposed. It is unchanged.

## minor 4 — two unmarked numeric claims, one of them the recommended option's only numeric cost

CLAUDE.md: *"Every numeric claim in the matrix is marked **MEASURED** or
**ESTIMATED**."* §1.1's *"`crates/pistol-cli/src/sha256.rs`, **206 lines**"* is
unmarked (it is correct — I measured it). §3 O4's *"**The config grows fifty lines**"*
is unmarked, and it is the only number in the recommended option's cost column. Fifty
body lines of `random_openings_v1.txt` are at most 38 characters each
(`awk '{print length}' | sort -n | tail -1` over the body → 38), so the claim is
sound; it is the marking that is missing, in the document whose §2 argues about
marking.

## minor 5 — the matrix supersedes two claims in the design it serves and names no amendment

`anchor_v3_openings_design.md:111-121` (§2.1) says the matrix *"enumerates the
**eight** from the code"* and that **"ALL EIGHT REFUSALS ARE REACHABLE ON `pistol-core`
ALONE. So the matrix is about code sharing and dependency weight, not capability."*
Revision 2 makes both false — seventeen, and its own §1.1 headline is *"THE DECISIVE
COLUMN, CORRECTED"*. §2.2's table is titled *"THE EIGHT"*. Whichever option wins, both
sections need rewriting, and that obligation appears in no option's cost column and in
no row of design §8.

## minor 6 — the cited precedent points the other way when read at the code

§3 O4: *"**Precedent, in this project, for this exact shape**: `tools/wp21_tranche_config.py`."*
Read at the file: the generator writes a config from **registered scalars** — `skip`,
`take`, a binary digest, fixed constants (`:113-160`) — and the config it writes still
names `openings_file = "{BOOK}"` (`:117`) so that `openings::load` reads the book **at
run time** (`crates/pistol-arena/src/bin/arena.rs:231`). **It materialises no opening
data whatsoever.** It is precedent for *generating a config that points at a book* —
which is O1's and O2's division of labour — and it is the nearest thing in the tree to
a counter-example for *copying a book's contents into a config*. Citing it for O4's
shape is a precedent asserted rather than read.

---

# ATTEMPTED AND ABANDONED

**"§1's table is a second transcription."** ABANDONED, and this was the brief's first
question. I ran the grep, got seventeen hits, opened every one, and mapped them onto
the rows: bijective, with one off-by-one range (minor 2). Nothing in the table is
inherited from REVIEW-design MAJOR 4's eight — the ordering, the D-numbers and the
ranges all differ, and the seven omissions round 1 listed are all present as A1, A3,
A5, A6, A7, A10, A15. **The table's derivation is real.** What is transcribed is S1
and S2 (MAJOR 1), which the printed grep cannot reach.

**"The grep receipt is itself incomplete — a refusal escapes the pattern."**
ABANDONED. The four `?` sites (`:60`, `:78`, `:83`, `:85`) resolve into rowed callees;
`parsed.drain(skip..skip + take)` at `:109` is unreachable-panic-free given `:80` and
`:98` (`saturating_add` closes the overflow path); `openings[0]` at `:226` is guarded
by `:80`; `replayed`'s `unreachable!` at `:254` is a panic on validated input, and the
doc comment at `:247-248` says why. There is no eighteenth refusal.

**"+7 is wrong — version skew makes it larger."** ABANDONED. I diffed name+version
across both locks. Only `syn` differs (3.0.3 / 3.0.4) and the matchserver's is the
newer, which resolution absorbs. Every other shared external matches exactly. **+7 is
right**; the finding against it is the unit, not the value (MAJOR 4).

**"O4 breaks the paired-balanced schedule (design §4)."** ABANDONED. `book line =
skip + (g-1)/2` and `a_is_p1 = g % 2 == 1` are arithmetic over `take`; where the
openings came from does not enter. §4 is untouched by any option.

**"O4 breaks the sealbot seat."** ABANDONED. Design §1's two seat facts
(`sealbot_client.rs:69-83` splitting an arbitrary ply list, `pistol_client.rs:157-174`
refusing an even list and re-chunking) are opening-source-agnostic, and I re-read
`pistol_client.rs:150-174` to confirm.

**"A refusal needs `pistol-eval`, `-search` or `-solver`, so O1 wins on capability."**
ABANDONED, as in round 1. I re-walked all seventeen. The heaviest anything needs is
`pistol-engine`'s `PositionSpec` (A10–A12) and `pistol-cli`'s digest (A2). §4's flip
condition is not met.

**"O4 commits a second copy of a retired fixture's data — hard rule 8 / D-505."**
ABANDONED as a breach. `.gitignore:35-36` puts the anchor's config outside the tree,
so nothing is committed twice. The residue is a reproducibility cost, recorded under
MAJOR 6, not a rule breach.

**"S1 is dead weight because the registered book has no commentary column."**
ABANDONED as an attack on the matrix. `random_openings_v1.txt`'s own header says
*"There is NO commentary column"*, so S1 never fires on the anchor's book — but design
§3 records that **`kind = "book"` accepts any file**, and `openings_v1.txt` carries
1,592 annotated lines. The matrix is right to keep S1 live.

**"The ~280-line estimate is a D-291 finding for being an estimate."** ABANDONED —
D-291 is *"about marking and not about forbidding"*, and it is marked. The live
finding is what the estimate is built on (FATAL 2b).

---

# THE STRONGEST SURVIVING ATTACK

> O4's three headline buys and O4's compliance with the design it serves cannot both
> be had, and the matrix never says which it is giving up. If `replay_check` keeps the
> second argument §5.4 registers — *"the book path with its two digests"*, the remedy
> REVIEW-design raised as BLOCKING 3 — then the matchserver workspace still contains a
> book reader and still contains a SHA-256, and "ONE reader of the book format", "22
> stays 22" and "no second SHA-256" are all false: O4 is O2 plus a generator. If
> instead `replay_check` reads the config, as §3's second failure mode quietly
> proposes, then the second instrument and the thing under test read the same
> document, and §5.4's own named defect returns word for word — *"a window off-by-one
> gives 100 games on one opening, exit 0, and a report echoing `skip` and `take` from
> the config"* — with the generator's receipt unable to catch it, because re-running a
> generator that has an off-by-one reproduces the off-by-one; the book's two digests
> then sit in the config verified by nobody, which is the very condition §2.2's R11
> refuses as *"a document naming values nothing reads"*; the dry run's registered
> limb 1, *"replays under `replay_check` **with the book argument**"*, becomes a
> criterion the binary cannot satisfy, which is BLOCKING 1's shape re-armed; and the
> anchor's report ends up naming `random_openings_v1.txt` by a `body_sha256` no
> process in the run ever computed, when D-147 put that digest in the file precisely so
> a run could name its book by content rather than by transcription. And the two
> derived facts §4 says moved the recommendation onto this option do not hold either:
> S2 costs O2 nothing, because a reader that can refuse `skip + take > total` has
> already parsed every line and validating them is the default, not an extra; and A2's
> digest costs 92 lines, not 206, because `crates/pistol-core/tests/common/sha256.rs`
> is a std-only FIPS-pinned SHA-256 that D-37 licenses in as many words — excluded from
> the matrix only by a grep scoped to `crates/pistol-core/src`, in a document whose
> opening argument is that greps beat prose.

---

# VERDICT

**THE MATRIX MUST BE RE-TAKEN AGAIN, AND THIS TIME THE RECOMMENDATION DOES NOT
SURVIVE AS WRITTEN.**

Round 1 could say *"the option survives, the matrix does not"* because its FATAL was
about the mitigation's paperwork and left the direction intact. That is not this
round. **§1's table is genuinely derived and genuinely complete — the census defect
did not recur, and I verified it hit by hit.** What failed is the argument built on
top of it. §4 states exactly two reasons for moving from O2 to O4 and marks both as
derived facts; one is free under O2 and the other is inflated by ~60% by a scoped
grep. Remove them and there is no stated warrant for the change. Worse, O4's own buys
are conditional on an amendment to a BLOCKING finding's remedy that the document
mentions as a "second failure mode" and never costs — and on that amendment the
anchor's second instrument stops being independent of the thing it checks.

**WHAT THE RE-TAKE MUST DO, none of it a run.** State which horn of FATAL 1 O4 takes
and cost it. Withdraw §4's reason 2. Re-cost A2 at 80 + 92 with D-37 cited, and add
**O6** — the digest moves into `run_match.sh`, which already shells `sha256sum` at
`:91` — because it disposes of reason 1 directly. Restore **O5** as a dev-dependency
complement with its 22 → 29 lock cost, and name **E2** and **E4** (with the R9
caveat) so the field is on the record. Re-cite S1 and S2 at code, and attribute them.
Replace "already tested" with 8 of 17. Add the compiled-lines row (4,542 → 31,384) or
withdraw *"the argument against A is not build time"*. Fix A3's reachability, A9's
range, O1's phantom failure mode, the two unmarked numbers, and the `wp21_tranche_config.py`
precedent, which on reading points at O1's shape rather than O4's.

I did **not** find a refusal that needs an engine, an eval or a search, so §4's own
flip condition is still unmet and **O1 does not win on capability**. My expectation
after an honest re-take is that the field narrows to **O2 or O6**, not O4 — but that
is the re-take's call, not mine, and I record it as an expectation rather than a
finding.
