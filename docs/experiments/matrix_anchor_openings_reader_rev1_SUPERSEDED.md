# OPTION MATRIX — where the anchor's opening READER lives

**THE DECISION.** The local matchserver must read a slice of a committed opening
book and hand the referee a legal opening. `crates/pistol-arena/src/openings.rs`
already is that reader, with eight refusals. The matchserver is a **detached**
cargo workspace depending only on `pistol-core`, `serde`, `serde_json` and
`toml`; `pistol-arena` pulls in `pistol-engine`, `pistol-eval`, `pistol-search`
and `pistol-solver`.

**WHY IT IS A MATRIX AND NOT A CHOICE.** REVIEW-design's MAJOR 4 raised it under
CLAUDE.md's own rule: *"A named decision with more than one viable option is
settled by an OPTION MATRIX … an option adopted without a matrix … is the same
breach as silent architecture drift."* Revision 1 of the design reproduced two of
the eight refusals and never named the alternative, so nothing recorded which
omissions were considered and which were not seen. **That is the defect this
document exists to close, and it is closed by enumerating the eight, not by
picking a side.**

---

## 1. THE EIGHT REFUSALS, ENUMERATED FROM THE CODE

Read from `crates/pistol-arena/src/openings.rs` at the line ranges named, so the
comparison below is against a list rather than against a memory of one.

| # | refusal | site | reachable on `pistol-core` alone? |
|---|---|---|---|
| R1 | the fixture's in-band `# body_sha256` disagrees with the body | `:60-68` (D-147/D-148) | **yes** — a digest over bytes |
| R2 | a blank line or a comment inside the body | `:156-170` | **yes** |
| R3 | a line that is not `start moves …` | `:180-187` | **yes** |
| R4 | an opening the RULES refuse — illegal, or already decided | `:190-191`, via `PositionSpec::replay` | **yes** — `GameState::make_turn` for the first, `state.outcome()` for the second |
| R5 | two openings equal up to a lattice symmetry | `:200-222` (D-137, rule 6's distinct-n) | **yes** — `pistol_core::canonical_form` |
| R6 | a file mixing turn counts | `:225-245` | **yes** |
| R7 | `skip + take > total` | `:98-108` | **yes** |
| R8 | a `turn_cap` that leaves no room for an engine move | `:86-95` | **yes** |

**THE COLUMN THAT DECIDES THE MATRIX IS THE LAST ONE, AND IT IS ALL YES.** Every
refusal is expressible over `pistol-core` plus a digest. The only thing
`pistol-engine` supplies is `PositionSpec`, whose `Start` arm is a ten-line
`make_turn` loop plus a won-position check (`crates/pistol-engine/src/position.rs:78-88`,
`:68-73`). **So reuse buys code sharing, not capability** — which is a smaller
claim than "the reader already exists" makes it sound, and it is the claim the
options below are weighed on.

---

## 2. THE OPTIONS

### A — depend on `pistol-arena` and call `openings::load`

- **Buys**: all eight refusals, already reviewed, already tested, already
  driven by `crates/pistol-arena/tests/`; `Opening::moves` as `Vec<Turn>` and
  `openings::replayed`'s play-order recovery for free; one reader in the tree, so
  a refusal added later reaches both callers.
- **Costs**: the matchserver stops being a harness over `pistol-core`. Its
  dependency closure gains `pistol-arena → pistol-engine → {pistol-eval,
  pistol-search, pistol-solver}` — the whole engine, in a binary whose stated
  purpose (`Cargo.toml`'s own comment) is *"a comparison harness, not shipped
  engine code"*.
- **Failure mode**: a change to the arena's opening semantics — made for the
  arena's reasons, reviewed against the arena's obligations — silently changes
  what an anchor plays. The two callers have different requirements and one
  reviewer would be reviewing for whichever they had in mind.
- **Second failure mode**: `openings::load` takes `turn_cap` and applies R8 with
  the ARENA's turn accounting. The matchserver's cap means the same thing today;
  nothing pins that it will.

### B — reimplement a minimal reader in the matchserver, on `pistol-core` alone

- **Buys**: the dependency stays four crates; the harness stays what its own
  `Cargo.toml` says it is; the anchor's opening semantics are reviewed against
  the ANCHOR's obligations.
- **Costs**: eight refusals to write and test, and §1's table is the list they
  are checked against. **Roughly 80 lines** — ESTIMATED, from the arena's own
  `:44-245` less the parts `PositionSpec` supplies.
- **Failure mode**: **THE ONE THAT ALREADY HAPPENED.** Revision 1 of the design
  reproduced two of eight and nobody could tell omission from oversight. The
  mitigation is not diligence, it is §1's table: a REVIEW-impl reviewer checks
  eight rows and a missing one is a finding rather than a judgement call.
- **Second failure mode**: the two readers drift, and a book that loads in the
  arena is refused by the anchor or the reverse. **Bounded by R1**: both verify
  the same in-band digest, so they cannot disagree about which bytes they read.

### C — extract a `pistol-openings` crate on `pistol-core`, used by both

- **Buys**: one reader, no heavy dependency, and the extraction is where the
  eight refusals get their own crate-level tests.
- **Costs**: a refactor of `pistol-arena` — moving code that is under test and
  cited by a landed pre-registration (`wp21_prereg.md` §2 binds the generator,
  and `openings::load` is what a tranche's book slice goes through). **A
  refactor of the sweep's own reader, on the eve of the sweep.**
- **Failure mode**: the sweep is the largest run this project has scheduled and
  its opening reader would be freshly moved. A defect introduced by the move is
  a defect in 3,487 openings' worth of corpus, found late.
- **Note**: C is the right answer LATER. It is the wrong answer this week, and
  the reason is scheduling rather than design.

---

## 3. THE NUMBERS, EACH MARKED

| claim | value | mark |
|---|---|---|
| matchserver's direct dependencies today | 4 (`pistol-core`, `serde`, `serde_json`, `toml`) | **MEASURED** — `tools/sealbot/matchserver/Cargo.toml` |
| crates option A adds to the closure | 4 (`pistol-arena`, `pistol-engine`, `pistol-eval`, `pistol-search`, `pistol-solver` less those already present) | **MEASURED** — the `Cargo.toml` chain |
| refusals reachable on `pistol-core` alone | 8 of 8 | **MEASURED** — §1's table, read from the code |
| option B's new code | ~80 lines | **ESTIMATED** |

**THERE IS DELIBERATELY NO BUILD-TIME ROW, AND D-291 IS WHY THE ABSENCE IS
STATED RATHER THAN LEFT.** A clean-build comparison would be the obvious proxy
for "dependency weight", and measuring option A's side of it requires **editing
`Cargo.toml` to add a dependency this matrix has not yet chosen** — which is not
a measurement taken in seconds but a speculative implementation of the option
under attack. **And it would not move the decision**: the argument against A is
not that the build is slow, it is that a harness whose own `Cargo.toml` says it
is *"a comparison harness, not shipped engine code"* would link the whole engine,
and that a change made for the arena's reasons would silently change what an
anchor plays. A build-time number would decorate that argument, not test it.
**What IS measured is the thing the argument rests on**: the dependency closure,
read from the `Cargo.toml` chain.

---

## 4. RECOMMENDATION

**OPTION B**, with §1's table promoted into the design as the checklist a
REVIEW-impl reviewer answers row by row.

**The argument in one line**: reuse's headline benefit is capability, and §1's
last column says the capability is not what is at stake — every refusal is
reachable on `pistol-core`, so option A is paying the whole engine's dependency
weight for code sharing between two callers whose requirements differ.

**What would flip it.** A ninth refusal that is NOT reachable on `pistol-core` —
anything needing an engine, an eval or a search to decide whether an opening is
admissible. None of the eight is. If the anchor ever wants openings filtered by
an engine's judgement, this matrix is re-taken and A wins immediately.

**What the recommendation costs and this document does not hide**: two readers of
one file format in one repository, which is the two-documents-one-claim shape
D-423 names. The mitigation is R1 — both verify the same in-band digest over the
same bytes — and the honest statement is that the mitigation bounds the drift
rather than preventing it.
