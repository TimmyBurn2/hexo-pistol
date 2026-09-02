# OPTION MATRIX — where the anchor's opening READER lives. REVISION 2.

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

### 1.1 THE DECISIVE COLUMN, CORRECTED

Revision 1 said *"8 of 8 reachable on `pistol-core` alone"* and the whole
recommendation turned on it. **Fifteen of seventeen are. A2 is not, and it is the
one that matters**: the digest is computed by
`pistol_cli::sha256::sha256_hex` — **`pistol-cli`, not `pistol-core`**
(`crates/pistol-cli/src/sha256.rs`, 206 lines). `pistol-core` has no SHA-256
(`/usr/bin/grep -rn "sha256\|Sha256" crates/pistol-core/src` is empty) and
neither does the matchserver (same grep over its `src`, empty — `run_match.sh:91`
shells out to `sha256sum`).

**SO REIMPLEMENTING MEANS REIMPLEMENTING A DIGEST TOO, or shelling out to
`sha256sum` from a Rust binary that today shells out to nothing.** That is a
materially bigger claim than revision 1 made, and it moves the recommendation.

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
| refusals re-expressible on `pistol-core` alone | **15 of 17** | **MEASURED** — §1's table, derived |
| the exception | **A2, which needs SHA-256** | **MEASURED** |

**STILL NO BUILD-TIME ROW, and the reason is unchanged**: measuring option A's
side needs a `Cargo.toml` edit adding the dependency this matrix has not chosen,
which is a speculative implementation rather than a measurement in seconds. The
argument against A is not build time.

---

## 3. THE OPTIONS

Labelled **O1–O5** so they cannot be confused with the design's own refusal
numbering, which revision 1's `R1–R8` collided with.

### O1 — depend on `pistol-arena`, call `openings::load`

- **Buys**: all seventeen refusals and both scope rules S1/S2, already reviewed,
  already tested; `Opening::moves` and `replayed`'s play-order recovery.
- **Costs**: **+7 packages** (MEASURED). The harness stops being what its own
  `Cargo.toml` calls it — *"a comparison harness, not shipped engine code"* — and
  links the whole engine.
- **Failure mode**: a change made for the arena's reasons, reviewed against the
  arena's obligations, silently changes what an anchor plays. `load` also takes
  `turn_cap` and applies A16 with the ARENA's turn accounting; nothing pins that
  the two stay the same.

### O2 — reimplement a reader inside the matchserver

- **Buys**: the dependency stays at 22.
- **Costs**: seventeen refusals plus S1 and S2 **plus a SHA-256** — revision 1's
  *"~80 lines"* excluded A2 and is wrong. **ESTIMATED ~280 lines** with a digest,
  or ~80 plus a shell-out the binary has no precedent for.
- **Failure mode**: **the one that already happened twice** — an enumeration
  transcribed rather than derived, and nobody able to tell omission from
  oversight. S2 is the sharpest case: a reader that validates only its window
  passes every row of §1 and is still weaker.

### O3 — extract a shared crate

- **Costs**: revision 1 called this *"a refactor"*; it is a **RE-LAYERING**.
  `openings.rs` uses `pistol_cli::sha256` and `pistol_engine::PositionSpec`, so
  extracting it means deciding where a digest and a position replayer live too.
- **Failure mode**: revision 1 rejected it on the sweep's schedule. **The sweep is
  HELD by an operator instruction and has not started**, so that ground is gone.
  The remaining cost is certain rather than speculative: the sweep's governing
  revision moves, which reopens `wp21_prereg.md`'s review.

### O4 — pre-materialise the openings into the matchserver's config

**A workspace-side generator reads the book with the reader that already
exists**, validates the slice against all seventeen refusals and both scope
rules, and writes the anchor's config with the openings in it verbatim, plus the
book path, its `body_sha256`, `skip`, `take`, and the config's own digest. **The
matchserver reads no book.** It parses `start moves …` with `Turn::from_str` and
replays with `make_turn`, refusing what the rules refuse (A10, A11, A12) — which
it must do anyway, because §3 of the design already requires the play order to
come from `make_turn`.

- **Buys**: **ONE reader of the book format in the repository.** No new
  dependency (22 stays 22). No second SHA-256. A2, A6, A7, A13, A14, A17, S1 and
  S2 are performed **once**, by the reader that is already reviewed and tested.
- **Precedent, in this project, for this exact shape**: `tools/wp21_tranche_config.py`
  writes the sweep's per-tranche configs from registered values and prints each
  one's sha256, *"because a config silently rewritten under a live run is a
  document that drifted from the run reading it (D-199)"*.
- **Costs**: a generator plus its coverage test (`docs/process.md`'s tools/ rule:
  a script producing a recorded artefact carries a test driving the shipped
  script). The config grows fifty lines.
- **Failure mode**: a hand-edited config. **Mitigated the way the precedent
  mitigates it** — the generator prints the config's digest, the pre-registration
  records it, and re-running the generator reproduces it byte for byte. And the
  matchserver still replays through the rules, so an illegal edit is refused at
  load.
- **Second failure mode**: the binding from game to BOOK LINE becomes a property
  of the generator's receipt rather than of a run-time check. The design's §5.4
  binding then reads against the config's own list, and the book binding is
  re-derivable rather than re-checked.

### O5 — `pistol-arena` as a dev-dependency only

**NOT VIABLE and named so the field is not padded.** The reader is needed by the
shipped `run_match.sh` path, not by tests. A dev-dependency does not reach it.

---

## 4. RECOMMENDATION

**OPTION O4.** And it changed from revision 1's O2 because **two derived facts
changed**, not because a newer option looked better:

1. **A2 needs a SHA-256 the matchserver does not have and `pistol-core` does not
   provide.** O2's cost was understated by an entire digest implementation.
2. **S2 exists**: the arena validates the WHOLE file before cutting the window.
   Any option that puts a reader in the matchserver either reproduces that or is
   a weaker reader of identical bytes — and O4 is the only option that does not
   have to reproduce it, because it does not read the book at all.

**WHAT WOULD FLIP IT.** If the anchor ever needs openings the matchserver selects
at run time — a resumable window, a different slice per seat — O4's
pre-materialisation stops working and O1 wins immediately, because O2's cost
would then include everything O4 avoided.

**THE STRONGEST SURVIVING ATTACK, recorded because the ADR line must quote one.**
The red team's, against revision 1's O2, and it survives against O4 in a changed
form:

> Two readers agreeing on the bytes and disagreeing on what the bytes mean is the
> entire failure class, and a digest does not touch it: `pistol-core` exports two
> ways to build a `Turn` from one book token that disagree — `Turn::from_str`
> refuses an uncanonical `a/b` spelling by name while `Turn::pair` silently
> canonicalises it, and `pistol_client.rs:169` already calls the canonicalising
> one. **O4 does not escape this.** It removes the second reader of the BOOK, but
> the matchserver still parses `start moves …` out of its config, so a token the
> generator accepted and the matchserver reads differently is still possible.
> **The honest bound is not a digest and not a checklist: it is a test that runs
> both parsers over the committed books and asserts the same verdict and the same
> `Vec<Turn>`.** O4 owes that test exactly as O2 would have.

**AND THE CONCESSION REVISION 1 MISASSIGNED.** It offered *"two readers of one
file format"* as O2's unique cost. **Every option leaves two**: the design's own
§5.4 puts a book-binding check in `replay_check`, and §5.1 pins that to
`pistol-core` alone. O4's advantage is not that it leaves one reader; it is that
the second reader reads a **config it was handed**, not a book format it must
independently agree about.
