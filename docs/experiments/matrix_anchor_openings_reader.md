# OPTION MATRIX — where the anchor's opening READER lives. REVISION 3.

**REVISION 3 IN ONE PARAGRAPH.** Revision 1 failed its red team on a FATAL: its
refusal table was a transcription of a review's prose dressed as a derivation.
Revision 2 derived the table correctly — round 2's red team re-ran the printed
grep, opened all seventeen hits, and confirmed the mapping is bijective — **and
then failed on its RECOMMENDATION**, because both of the "derived facts" that
moved it off option O2 were wrong, and the option it moved to carries a dilemma.
**Revision 3 corrects the two facts, drops O4, and returns the recommendation to
O2** — which is what round 1's red team said survives. §1's table is unchanged
and is the one thing in this document that has now passed an attack.

**THE TWO FACTS THAT WERE WRONG, AND ONE OF THEM IS THE SAME MISTAKE AS THE
FATAL.**

- **A2 does not need `pistol-cli`.** Revision 2 said SHA-256 lives in
  `pistol_cli::sha256`, 206 lines, and that `pistol-core` has none — *"grep over
  `crates/pistol-core/src` is empty"*. **The grep was scoped to `src`.**
  `crates/pistol-core/tests/common/sha256.rs` is **92 lines, zero `use` lines,
  pure std**, licensed by D-37 and pinned against the published FIPS 180-4
  vectors. **A grep whose scope excludes the answer is the same defect as a table
  transcribed from prose**, one directory deeper.
- **S2's parse limb is FREE under O2.** Revision 2 said a matchserver reader
  would have to reproduce "validate the whole file before cutting the window".
  `total` is `parsed.len()` **after the loop has parsed every body line**
  (`openings.rs:75-79`, `:96`), so any reader that refuses `skip + take > total`
  has already parsed the whole body. What is NOT free is the SCOPE of the
  symmetry dedup and the digest, and that becomes a required row rather than a
  reason.

Revisions 1 and 2 are kept as `..._rev1_SUPERSEDED.md` and `..._rev2_SUPERSEDED.md`;
the two red-team reports are `..._REDTEAM.md` and `..._REDTEAM_round2.md`.

---

## PRIOR REVISION 2's OPENING, KEPT BECAUSE ITS FATAL IS THE ARC'S OWN LESSON

**REVISION 1 FAILED ITS DECISION-RED-TEAM ON A FATAL, AND THE FATAL IS THIS
DOCUMENT'S OWN SUBJECT MATTER.** Its §1 table claimed to enumerate
`crates/pistol-arena/src/openings.rs`'s refusals "from the code". It did not: it
was a **verbatim transcription of REVIEW-design's MAJOR 4** — same eight items,
same order, same line ranges, same D-numbers — and it **omitted at least seven
refusal sites**, including the one that refuses a file carrying no
`# body_sha256` line at all, which has a test of its own. The table was the
recommendation's whole mitigation, so it reproduced the very defect it existed to
close.

**THAT IS THE SECOND TIME IN THIS ARC**, and the first has a standing law
attached: D-568, *"a mutation set is specified against CALL SITES ENUMERATED BY A
`git grep` RECEIPT recorded in the mutation document, never against prose, and a
comment asserting a property about call sites is not evidence of that property."*
A matrix's refusal table is the same object under another name. **Revision 2's §1
is derived by grepping the file for its refusal sites and reading each one**, and
the grep is printed so a reader checks the derivation rather than the conclusion.

Revision 1 is kept as `matrix_anchor_openings_reader_rev1_SUPERSEDED.md`; the
red team's report is `matrix_anchor_openings_reader_REDTEAM.md`.

---

## 0. THE DECISION

The local match platform (`tools/sealbot/matchserver/`) must play a registered
opening book instead of its one hard-coded origin stone
(`referee.rs:17`). `crates/pistol-arena/src/openings.rs::load` already reads that
format. **Where should the anchor's opening reader live?**

---

## 1. THE REFUSALS, DERIVED

```
$ /usr/bin/grep -n "ArenaError::\|return Err(" crates/pistol-arena/src/openings.rs | LC_ALL=C sort -t: -k1n
```

Seventeen refusal sites, each read at the line the grep returned. **Not eight.**

| # | site | refusal | needs more than `pistol-core`? |
|---|---|---|---|
| A1 | `:58-59` | the file cannot be read | no |
| A2 | `:60`,`:63-67` | the body's digest disagrees with the header's claim | **YES — SHA-256** |
| A3 | `:70-71` | the file is not UTF-8 | no |
| A4 | `:80-81` | the body states no openings | no |
| A5 | `:125-126` | the header scan hits non-UTF-8 | no |
| A6 | `:133-138` | `# body_sha256 ` carries something other than 64 hex digits | no (a shape check) |
| A7 | `:143-151` | **there is NO `# body_sha256 ` line at all** — *"a file without that line and a file whose body matches must not look alike"* (D-147, D-148) | no (a presence check) |
| A8 | `:156-161` | a blank line in the body | no |
| A9 | `:162-168` | a comment inside the body | no |
| A10 | `:177-179` | the line does not parse as a position at all | no — `Turn::from_str` |
| A11 | `:180-187` | it parses, but is not a `start moves …` move list (D-6) | no |
| A12 | `:189-191` | the RULES refuse the replay — illegal move, **or an already-decided position** | no — `make_turn` + `state.outcome()` |
| A13 | `:200-222` | two openings equal up to a lattice symmetry (D-137, rule 6's distinct-n) | no — `canonical_form` |
| A14 | `:225-241` | the file mixes turn counts | no |
| A15 | `:242-244` | an opening longer than a turn counter | no |
| A16 | `:86-95` | `turn_cap` does not leave room for an engine move | no |
| A17 | `:97-108` | `skip + take > total` | no |

**AND TWO RULES THAT ARE NOT REFUSALS AND APPEAR IN NO ROW OF REVISION 1**, both
load-bearing:

- **S1, `:170-174`** — *"Everything from `" #"` onward is commentary"* (D-143). A
  reader without this rejects a legal annotated line; a reader that strips
  differently digests a different body.
- **S2, `:50-53`** — **the WHOLE file is parsed, digest-verified and
  symmetry-deduped BEFORE the window is cut** (D-202), *"so a defect outside the
  window still refuses the file"*. A reader that validates only the 50 openings
  it plays is a strictly weaker reader of identical bytes, and it would pass a
  row-by-row review of A1–A17.

### 1.1 THE DECISIVE COLUMN, CORRECTED TWICE

Revision 1 said *"8 of 8 reachable on `pistol-core` alone"*. Revision 2 said
*"fifteen of seventeen, and A2 is the exception that moves the recommendation"*.
**Both overstated the barrier, and A2 is not a barrier at all.**

`pistol-arena` computes the digest with `pistol_cli::sha256::sha256_hex`, so
depending on the ARENA does drag `pistol-cli` in — that much stands, and it is
`pistol-cli`'s absence from revision 1's chain that made its cost row wrong.
**But a reimplementation needs no such thing. THREE routes are open and all
three are already precedented in this repository:**

| route for A2 | cost | precedent |
|---|---|---|
| vendor `crates/pistol-core/tests/common/sha256.rs` | **92 lines, zero `use` lines, pure std** — **but the FIPS 180-4 vectors are NOT in it**: they live in `golden_board_tests.rs:22-47`, and D-37/D-91/D-144 require them to travel with any copy, so the real cost is **~122 lines**. **AND NO GATE WOULD RUN THEM** (§4's FATAL): the matchserver has no lib target and nothing runs `cargo test` there, so a vendored digest arrives with its pin unrunnable | **D-37** licenses the implementation; it does not licence shipping it unpinned |
| shell out to `sha256sum` | one line | **`tools/sealbot/run_match.sh:91` already does it** for the bytes each seat writes |
| let `run_match.sh` verify the book before it launches the binary | one line, and the refusal lands before any process | the same |

**SO EVERY ONE OF THE SEVENTEEN IS REACHABLE WITHOUT `pistol-engine` AND WITHOUT
`pistol-cli`**, and the matrix is about code sharing and dependency weight — which
is what revision 1 claimed and got right for the wrong reasons.

---

## 2. THE COST, MEASURED FROM THE LOCKFILES

Revision 1's one MEASURED row said *"4 crates added"*, listed five names, and
omitted `pistol-cli` — which is in the chain precisely because of A2.

```
$ grep -c "^name = " tools/sealbot/matchserver/Cargo.lock   ->  22
$ grep -c "^name = " Cargo.lock                             ->  26
```

| claim | value | mark |
|---|---|---|
| packages the matchserver resolves today | **22** | **MEASURED** — its own `Cargo.lock` |
| packages under option A | **29** | **MEASURED** — 22 plus `pistol-arena`, `pistol-cli`, `pistol-engine`, `pistol-eval`, `pistol-search`, `pistol-solver`, `serde_path_to_error` |
| **delta** | **+7 packages** | **MEASURED** |
| refusals re-expressible without `pistol-engine`/`pistol-cli` | **17 of 17**, A2 by any of §1.1's three routes | **MEASURED** — §1's table, derived |
| **first-party lines the matchserver compiles today** | **4 542** — its own 2 180 plus `pistol-core`'s 2 362 | **MEASURED** — `find … -name '*.rs' -not -path '*/bin/*' \| xargs wc -l` |
| **first-party lines under O1** | **29 223** — adding arena 6 029, cli 5 223, engine 2 029, eval 981, search 5 816, solver 4 603 | **MEASURED**, same command |
| **the real cost unit** | **+24 681 first-party lines, a 6.43x increase** | **MEASURED** |

**AN EARLIER SPELLING OF THESE ROWS SAID +26 842 AND 6.9x, AND THE DENOMINATOR
WAS WRONG.** It counted `src/bin` — arena 1 027, cli 915, solver 219, **2 161
lines of binaries a library dependency never compiles**. The command is printed
above WITH ITS `-not -path '*/bin/*'` so a reader checks the scope and not just
the arithmetic; the earlier spelling elided it, which on a document already
failed twice for scope is the defect and not a slip.

**THE PACKAGE COUNT WAS THE WRONG UNIT AND IS DEMOTED.** *"+7 packages"* counts
six first-party path crates and one already-vetted third-party crate as if they
were alike, and it hides that the six are 26 842 lines of this project's own
engine. The line count was one `find | wc` away and revision 2 did not take it —
which is D-291's named target, on a matrix that had already been failed for
asserting rather than deriving.

**STILL NO BUILD-TIME ROW**: measuring O1's side needs a `Cargo.toml` edit adding
the dependency this matrix has not chosen, which is a speculative implementation
rather than a measurement in seconds. The argument against O1 is the 26 842
lines, not the seconds.

### 2.1 "ALREADY REVIEWED, ALREADY TESTED" IS HALF TRUE, AND THE HALF MATTERS

`crates/pistol-arena/tests/openings_tests.rs` holds **10 tests**, and mapping
each to §1's table gives **8 of the 17 refusals a direct test**: A2, A7, A8, A9,
A13, A14, A16, A17. **NINE have none, and here are nine**: A1, A3, A4, A5, A6,
A10, A11, A12, A15. **S2 has none. S1 DOES** — `openings_tests.rs:59-62` reads
`openings_prefix`, built from `openings_v1.txt`, whose body lines carry ` #`
commentary, so the D-143 stripping rule is exercised.

**AN EARLIER SPELLING OF THIS PARAGRAPH WAS WRONG THREE WAYS IN ONE SENTENCE** —
it said "nine" and then listed eight (A4 dropped), and it called S1 untested. **A
count and a list that disagree is the same defect as a table transcribed from
prose**, and it is the third instance in this document.

O1's headline benefit is therefore *"seventeen refusals, eight of them tested"*,
which is a smaller claim than *"already tested"*.

---

## 3. THE OPTIONS

Labelled **O1–O5** so they cannot be confused with the design's own refusal
numbering, which revision 1's `R1–R8` collided with.

### O1 — depend on `pistol-arena`, call `openings::load`

- **Buys**: all seventeen refusals and both scope rules S1/S2 — **eight of them
  with a direct test** (§2.1); `Opening::moves` and `replayed`'s play-order
  recovery.
- **Costs**: **+26 842 first-party lines, 4 542 -> 31 384** (MEASURED). The
  harness stops being what its own `Cargo.toml` calls it — *"a comparison
  harness, not shipped engine code"* — and links the whole engine.
- **Failure mode**: a change made for the arena's reasons, reviewed against the
  arena's obligations, silently changes what an anchor plays. `load` also takes
  `turn_cap` and applies A16 with the ARENA's turn accounting; nothing pins that
  the two stay the same.

### O2 — reimplement a reader inside the matchserver

- **Buys**: the dependency stays at 22.
- **Costs**: seventeen refusals plus S1 and the dedup scope of S2. **A2 is 92
  vendored std-only lines or one shell-out (§1.1), not the barrier revision 2
  made of it.** S2's PARSE limb is free — `total` is `parsed.len()` after every
  body line is parsed, so a reader that refuses `skip + take > total` has already
  read the whole body. **ESTIMATED ~180 lines** including the vendored digest.
- **Failure mode**: **the one that already happened twice in this document** — an
  enumeration transcribed rather than derived, and nobody able to tell omission
  from oversight. The mitigation is §1's table promoted into the design as a
  row-by-row checklist, plus §2.1's honest note that eight of the seventeen rows
  have a test to copy and nine do not.
- **The residual S2 limb, named because it is the only one not free**: the SCOPE
  of the digest and of the symmetry dedup. A reader that dedupes only its window
  passes every row of §1 and is still weaker than the arena's. **It is a required
  row, not a reason to change option.**

### O3 — extract a shared crate

- **Costs**: revision 1 called this *"a refactor"*; it is a **RE-LAYERING**.
  `openings.rs` uses `pistol_cli::sha256` and `pistol_engine::PositionSpec`, so
  extracting it means deciding where a digest and a position replayer live too.
- **Failure mode**: revision 1 rejected it on the sweep's schedule. **The sweep is
  HELD by an operator instruction and has not started**, so that ground is gone.
  The remaining cost is certain rather than speculative: the sweep's governing
  revision moves, which reopens `wp21_prereg.md`'s review.

### O4 — pre-materialise the openings into the matchserver's config — **DROPPED**

A workspace-side generator reads the book with the reader that already exists,
validates the slice, and writes the anchor's config with the openings in it
verbatim; the matchserver reads no book. Revision 2 recommended it.

**IT IS DROPPED ON A DILEMMA ROUND 2's RED TEAM NAMED, AND THE DILEMMA HAS NO
THIRD HORN.** Its buys and its compliance with the landed design are mutually
exclusive:

- **Keep design §5.4** — which is BLOCKING 3's remedy, and which gives
  `replay_check` *"the book path with its two digests"* so that a window
  off-by-one cannot give 100 games on one opening at exit 0. Then **the workspace
  still holds a book reader and a SHA-256**, and O4's three headline buys — one
  reader, no new dependency, no second digest — are all false.
- **Drop §5.4's book argument**, as revision 2's own *"second failure mode"*
  quietly proposed. Then **the second instrument reads the same config as the
  thing under test**, §5.4's own defect returns verbatim, the config's two
  digests become *"values nothing reads"* — which the design's own R11 refuses —
  and the dry run's D1 becomes unsatisfiable, which is BLOCKING 1's shape
  re-armed.

**AND ITS TWO SUPPORTING FACTS COLLAPSED** (§1.1, §3's O2 row): A2 was never a
barrier and S2's parse limb was always free. **An option adopted for two reasons
that were both wrong is not rescued by a third**, and the dilemma above is
independent of them.

**ONE THING FROM O4 SURVIVES AND IS CARRIED INTO O2's OBLIGATIONS**: a run whose
played openings live only in a gitignored `local/*.toml`, named by a digest no
process computed, is not a reproducible run. **O2 must make the anchor's report
carry the book path, both digests, `skip` and `take`** — which the design's §5
already requires and which O4 would have made harder, not easier.

### O5 — `pistol-arena` as a dev-dependency only

**NOT VIABLE as a substitute for the reader**, and revision 2 struck it too
fast. The reader is needed on the shipped `run_match.sh` path, so a dev-dependency
does not reach it. **But it IS the mechanism for the obligation §4 names**: a
test that runs both readers over the committed books and asserts the same verdict
is a TEST, and a dev-dependency reaches a test. **It is carried into O2's
obligations rather than left struck.** Note that a dev-dependency does enter the
lockfile, so *"22 stays 22"* is false once that test exists — which is why §2's
cost unit is lines rather than packages.

### O6 — the digest never enters the matchserver at all

`run_match.sh` verifies the book's in-band `# body_sha256` and the whole-file
digest **before it launches the binary**, with `sha256sum`, which it already runs
at `:91`. The matchserver then reads a file whose bytes are already attested.

**VIABLE, and it is A2's cheapest route** — §1.1's third row. It is not a separate
option so much as an implementation choice inside O2, and it is recorded as one
because revision 2 built a whole recommendation on A2 being expensive.

---

## 4. RECOMMENDATION

**OPTION O2 — reimplement a minimal reader in the matchserver, with A2 taken by
§1.1's vendored 92-line std-only digest or by O6's shell-out.**

**AND THE HONEST STATEMENT OF WHY IS THAT NOTHING EVER MOVED IT.** Round 1's red
team said O2 survives. Revision 2 moved off it on two facts that were both wrong.
Round 2's red team killed the option those facts led to. **O2 is not chosen here
because a third analysis favoured it; it is where the field has stood since the
first attack, and two revisions of this document were the noise.**

**THE COST, STATED WITHOUT DECORATION**: ~180 lines including the digest, of
which the seventeen refusals are the substance; eight of them have a test in
`openings_tests.rs` to copy and **nine do not**; and the repository ends with two
readers of one book format.

**WHAT WOULD FLIP IT.** A refusal that needs an engine, an eval or a search to
decide whether an opening is admissible. None of the seventeen does. If the anchor
ever wants openings filtered by an engine's judgement, O1 wins immediately.

**THE OBLIGATIONS THIS RECOMMENDATION CARRIES — AMENDED AFTER ROUND 3, WHICH
FOUND THE LOAD-BEARING ONE UNWRITABLE.**

**ROUND 3's FATAL, AND IT IS A STRUCTURAL FACT NOBODY HAD LOOKED UP.** The
obligation this recommendation rested on was *"a differential test running both
readers over the committed books and asserting the same verdict and the same
`Vec<Turn>`"*. **It cannot be written as described, for three independent
reasons**, and any one of them is enough:

- **The committed books are all ACCEPTED.** The failure class is *a matchserver
  reader that accepts what the arena refuses*, so a differential run over files
  both readers accept can exhibit no refusal and no S2 scope defect. It would
  pass on the day the reader was written and every day after.
- **"The same verdict" is refused by the design itself.** `anchor_v3_openings_design.md`
  §2.2 requires the matchserver reader to refuse **strictly more** than the arena
  — R5 (undecided and at a turn boundary), R8's `take < 1`, R10's
  `games == 2 * take`, R11's platform-form key check. Two readers that must
  disagree cannot be tested for agreement.
- **THE MATCHSERVER HAS NO LIBRARY TARGET AND NO GATE RUNS ITS TESTS.** There is
  no `tools/sealbot/matchserver/src/lib.rs`; `main.rs:11-18` declares the modules,
  so **no integration test can reach the reader at all**. And nothing would run
  one: **gate 3** is `cargo test --workspace`, and the matchserver is a DETACHED
  workspace (its `Cargo.toml`'s empty `[workspace]` table says so); **gate 16**
  runs `tools/sealbot/tests/run_tests.sh`, which drives the shipped
  `run_match.sh` — and `run_match.sh:51` runs `cargo build --release --locked`
  and **never `cargo test`**. `/usr/bin/grep -n cargo` over both scripts returns
  two hits, both `build`.

**SO THE MITIGATION NAMED A TEST THAT NO TARGET COULD HOLD AND NO GATE WOULD RUN
— which is precisely the defect this revision accuses its own revision 2 of.**
The obligations are replaced with ones the tree can actually carry:

1. **§1's table promoted into the design as a row-by-row checklist**, with §2.1's
   note on which nine rows have no test to copy and which one (S1) does.
2. **THE DIFFERENTIAL BECOMES A REFUSAL-CONTAINMENT SUITE INSIDE GATE 16.**
   `tools/sealbot/tests/run_tests.sh` is the only thing that reaches the
   matchserver, and it drives the SHIPPED script, which is what
   `docs/process.md`'s coverage rule asks for anyway. It gains **book fixtures
   built to be REFUSED — one per row of §1's table** — and asserts the
   matchserver refuses each **by name and at the right exit class**. The claim is
   one-directional and therefore true: **everything the arena refuses, the
   matchserver refuses.** The reverse is false by design and is not asserted.
3. **A2 IS TAKEN BY O6, NOT BY VENDORING.** With no gate running matchserver
   tests, a vendored digest arrives with its FIPS pin unrunnable — and D-37,
   D-91 and D-144 all require the vectors to travel with the implementation.
   `run_match.sh` verifies both digests with `sha256sum` **before it launches the
   binary**, as it already does at `:91`, so the check sits in the one place gate
   16 exercises and the refusal lands before any process.
4. **The S2 scope row**: the reader digests and symmetry-dedupes the WHOLE file
   before cutting the window, not just the slice it plays. **And it may not reach
   A17 by counting lines** — round 3's point, and it is sharp: *"`total` is
   `parsed.len()` after every line is parsed"* was derived from the code being
   REPLACED, and a reimplementation is free to set `total = lines().count()`,
   satisfy A17, and never parse a thing. The row is therefore *parse every body
   line*, not *refuse `skip + take > total`*.
5. **The report carries the book path, both digests, `skip` and `take`.**

**AND THE REPOSITORY ENDS WITH THREE READERS, NOT TWO.** The design's §5.4 puts a
book-binding reader in `replay_check`, which shares no module with `main.rs`.
Revision 2's *"two readers"* concession undercounted, and collapsing it to two is
the horn that killed O4.

**THE STRONGEST SURVIVING ATTACK, for the ADR line. It is round 3's, not round
1's, because round 1's had a remedy and this one takes it away.**

> The recommendation's whole answer to *"two readers can agree on the bytes and
> disagree on what they mean"* was a differential test — and the matchserver has
> **no library target** (`main.rs:11-18` declares the modules; there is no
> `src/lib.rs`), so no test can reach the reader, and **no gate would run one if
> it could**: gate 3 is `cargo test --workspace` and the matchserver is a detached
> workspace, gate 16 drives `run_match.sh`, which runs `cargo build` and never
> `cargo test`. The mitigation named a test that no target could hold and no gate
> would run. What survives is weaker and must be said as such: a refusal-
> containment suite inside gate 16 can show that the matchserver refuses
> everything the arena refuses **on the fixtures somebody thought to write**, and
> nothing mechanical closes the gap between that set and the set of files the two
> readers would read differently. **O2 is chosen with that gap open**, because
> every alternative pays 24 681 shipped lines, or dies on O4's dilemma, or needs
> an engine to decide whether an opening is admissible — and none of the
> seventeen refusals does.

**THAT ATTACK IS NOT ANSWERED. It is the cost of the recommendation**, and an ADR
line that quotes it is recording a cost rather than a mitigation — which is what
D-424 asks of an option that wins on balance rather than on merit.

---

## 5. WHAT THIS MATRIX COST, RECORDED BECAUSE IT IS THE FINDING

Three revisions, three DECISION-RED-TEAM rounds, **and the option never moved**.
Round 1 declined to kill O2. Revision 2 left it on two facts that were both
wrong. Round 2 killed what revision 2 moved to. Revision 3 returned to O2.
Round 3 found the recommendation's mitigation unwritable but the option intact.

**THE THREE DEFECTS WERE ALL ONE DEFECT**: a claim asserted at the scope where it
was convenient rather than derived at the scope where it is true — a table
transcribed from a review's prose; a grep scoped to `src` that missed the answer
in `tests`; a line count scoped to `src` that counted `bin`. **D-568's standing
law was written for the first of these and the second and third happened after
it**, which says the law needs the check attached: *print the command with its
scope, and compare its hit count to the prose's before believing either.*
