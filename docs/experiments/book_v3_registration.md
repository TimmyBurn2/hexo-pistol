# `book_v3` — premises, size registration, and the classification finding that stopped the package once

**STATUS: R1 COMPLETE, R2 IN PROGRESS. The R1 stop is LIFTED.** This document
stopped the package at R1 on the classification conflict in §11 and asked for an
operator ruling. The operator returned the decision to this session, and it was
settled the way CLAUDE.md's Process section requires a named decision with more
than one viable option to be settled: an OPTION MATRIX
(`docs/experiments/matrix_book_v3_storage.md`) attacked by a fresh-context
DECISION-RED-TEAM before selection. The selection is **Option D**, recorded at
**D-647**, which supersedes D-645.

**CORRECTION, because this document said otherwise at `8f8c306` and the sentence
is now false.** Revision 1 stated *"No book was generated, no range was claimed,
no opening was drawn."* That was true when committed and is no longer: a
candidate book of 8500 openings has since been generated, to measure generation
cost and canonical overlap. **No RANGE has been claimed and no governed run has
been taken** — those halves still hold — but a book has been generated, and the
red team was right to name the contradiction rather than let a committed
document and the tree disagree.

**This document is deliberately NOT in `tools/governing_citation_check.sh`'s
`GOVERNING` list.** That list names documents that GOVERN a run. This one
governs nothing, because no run was licensed. Adding it is part of a resume,
not part of this stop.

Revision this work was done at: `0a430d2e3e98430d2cd98d28765e79939c9b0f51`.

## Premises (R0)

Quoted per D-477. Every quotation below was taken with `/usr/bin/grep` or
`sed` against tracked bytes at the revision above.

### P1 — `enum BookVersion`, its variants, and every match site

`git grep -n 'enum BookVersion'` returns **one** site:

    crates/pistol-cli/src/random_openings/mod.rs:32:pub enum BookVersion {

**Two variants**, `V1` and `V2`. The enum's own doc says what it is for:

> Which book a generation run writes.
>
> A CLOSED SET, and the closure is the point: the file name is the tool's and
> never the operator's, so a book that could be written under any name is a
> book whose pin means nothing. A new book is a new variant here plus the
> header its own [`document::render`] arm states, which is a reviewed change
> and not a config edit.

`git grep -n 'BookVersion::'` returns **14 code and test sites**, in five
files, plus one in `docs/decisions.md` (D-645, appended by this package's §0):

| file | sites | what they are |
|---|---|---|
| `crates/pistol-cli/src/random_openings/mod.rs` | 4 | `file_name()` at 45-46, `label()` at 53-54 |
| `crates/pistol-cli/src/random_openings/document.rs` | 2 | preamble selection at 95-96 |
| `crates/pistol-cli/tests/random_openings_cli_tests.rs` | 3 | 267, 291, 293 |
| `crates/pistol-cli/tests/random_openings_config_tests.rs` | 2 | 201, 218 |
| `crates/pistol-cli/tests/random_openings_document_tests.rs` | 3 | 40, 261, 262 |

Bare `BookVersion` mentions across the tree: **26**.

**THE FINDING P1 EXISTS TO PRODUCE.** `BookVersion` lives in **pistol-cli**,
and **pistol-arena does not reference it at any site**. It is a WRITE-SIDE
name: it selects an output file name and a header preamble at generation time.
It resolves nothing at load time, because nothing loads through it. The
dispatcher's Scope line — *"Diff surface: pistol-arena (BookVersion::V3 and its
loader)"* — names the wrong crate and a mechanism that does not exist. See §11.

### P2 — `book_v2_registration.md`: the generation command, the seed, the §4 rule

The generator command and the seed live in `configs/random_openings_v2.toml`,
which states itself as *"the ONLY home of those five values"*:

>     cargo run -p pistol-cli --bin random-openings -- \
>       --config configs/random_openings_v2.toml \
>       --out-dir crates/pistol-cli/tests/fixtures

> `book = "v2"`, `k_stones = 5`, `n_openings = 4500`, `max_radius = 5`,
> `seed = 20260830`

**§4, THE SIZE DECISION RULE, verbatim:**

> Let **`P`** be the smallest pair cap, taken from the measured sweep, at which
> the WP-1.5d configuration (`elo0 = 0`, `elo1 = 15.0`, `alpha = beta = 0.05`,
> bucket shape = D-491's governed pentanomial `30/75/277/68/50`) reaches **power
> ≥ 0.90** under `truth = elo1`.
>
> Then:
>
> ```
> n_openings = ceil_to_500( P + 500 )
> ```

Storage: the book is `crates/pistol-cli/tests/fixtures/random_openings_v2.txt`,
a **committed fixture** — the config says so in as many words: *"The book they
produced is a committed fixture, and a test regenerates it from this document
and compares the bytes, so editing anything here without regenerating the book
is a red test."* Its digest today is
`829361a9ae61d0d4369b5291bfc893133fa8160867f11cc638b11f432b6cc29a`; v1's is
`895a05edb53a0a8d89c262bb058e3bc3dd24d446405d375458aaf067e2f076e7`.

There is **no manifest** for either book. The in-tree `manifest_row` mechanism
(`capture_file.rs:106`, `census_file.rs:131`, `labels_file.rs:139`) exists for
captures, censuses and corpora — never for a book.

### P3 — D-568 and the v2 usage ledger

`docs/book_v2_ledger.md` records three rows over the 4500-opening book:

| `openings_skip` | `openings_take` | range | consumed by |
|---|---|---|---|
| 0 | 13 | `0..12` | the WP-2.0 label-pipeline PILOT |
| 13 | 3487 | `13..3499` | the WP-2.1 PRODUCTION LABEL SWEEP |
| 3500 | 1000 | `3500..4499` | **RESERVED — NEVER LABELLED** (D-568) |

D-568's own words, quoted in the ADR from the architect:

> book_v2 holdout of 1,000 openings reserved for governed runs, never labelled;
> sweep takes the remaining 3,500; anchors may use book_v1.

**The reservation is whole and this package did not touch it.** The ledger also
records that WP-2.2 Phase 1's row for `3500..3899` *"stood for two commits"* and
was **withdrawn** because the run was not launched, so *"the range is therefore
unspent and the reservation is whole."*

**AND THE LEDGER CARRIES A NUMBER BV3-4 CONTRADICTS**, quoted because it is the
measured precedent for what disjointness between two of these books actually
is:

> It does not say that no individual position appears in both — both books draw
> independently from one finite pool, and the measured overlap (1 identical
> line, 10 positions up to symmetry, against expectations of 0.59 and 7.04) is
> what chance gives.

That overlap is pinned by
`the_two_books_overlap_only_as_far_as_independent_drawing_makes_them` in
`crates/pistol-cli/tests/random_openings_document_tests.rs`.

### P4 — D-505, v1 retired for governed use

> D-505: **`book_v2` IS SCHEDULED INTO THE STAGE-3 DETECTOR'S §0, AND THE SPENT
> 2000-OPENING BOOK IS RETIRED FOR GOVERNED USE.** [...] **fresh ranges by
> construction**, so no slice can collide with one `random_openings_v1.txt`
> already spent; and a **size registered with its grounds, covering the SPRT's
> worst-case n** — a book sized to the expected n is a book that runs out
> mid-run, which is the failure this line exists to prevent.

D-518 is where it fired: *"`random_openings_v1.txt` IS RETIRED FOR GOVERNED USE
WITH ITS BYTES UNTOUCHED."*

### P5 — the two placeholder keys, resolved

The dispatch spells these `D-56x` and `D-56q`. Neither is a line the log holds.
Resolved in D-644 rather than left dangling, on the precedent of D-565, D-568,
D-569 and D-611:

- **`D-56x` is D-562**, the usage ruling. `docs/experiments/wp21_DISPATCH.md:198`
  says so in the tree: *"**`D-56x` in the dispatch is D-562**, appended to
  `docs/decisions.md` this session:"*
- **`D-56q` is D-568**, the holdout ruling, which took that number in its own
  first sentence: *"TAKING THE NUMBER THE DISPATCH SPELLS `D-56q`."*

D-568 states the standing reason this matters: *"a dispatch citing a key no ADR
log holds sends a successor grepping for a decision that does not exist."*

### P6 — `sprt_power.rs`, its CLI, and the Phase 1 pentanomial

CLI keys (`parse`, lines 59-67): `--pairs --elo0 --elo1 --alpha --beta --truth
--runs --seed --buckets`. `--buckets` takes five comma-separated counts.

`artifacts/wp22_phase1_quiet/POWER.txt`, digest
`26ca6f3095421340764dfb98478f700a110a717a68ed09630d7d93340db909e8`, states its
own invocation:

> Invocation: --buckets 2,3,8,7,4 --runs 20000 --seed 1, varying --pairs /
> --elo1 / --truth. The buckets are the committed dry run's own pentanomial.

**The pentanomial is `2,3,8,7,4` — 24 pairs in total.** That is a thin base for
a sizing decision and is recorded here as a named limitation, not smoothed over.

### P7 — the canonical-form API the corpus dedup uses

One function, in pistol-core:

    crates/pistol-core/src/symmetry.rs:166:pub fn canonical_form(stones: &[(Coord, Player)]) -> Vec<(Coord, Player)>

`key_full` is that function's output, rendered — `crates/pistol-arena/src/labels.rs:203`:

    key_full: render_key_full(&canonical_form(&stones)),

and the field is documented at `crates/pistol-arena/src/labels_file.rs:28`:

> `canonical_form` over the stones: the same position up to BOTH.

The same function is already used by the arena's own loader
(`crates/pistol-arena/src/openings.rs`) for `refuse_symmetry_duplicates`, and by
the generator for its dedup — so **BV3-4's internal-distinctness requirement is
already enforced twice** in the shipped path, once at generation and once at
load.

### P8 — which CI gates read a book or the arena handshake

| gate | reads |
|---|---|
| 3 `cargo test --workspace --locked` | the book pinning tests: byte-for-byte regeneration, canonical dedup, the v1/v2 overlap pin |
| 6 `config validation` | `configs/random_openings_*.toml` via `validate_random_openings_config` |
| 15 `arena self-match smoke` | an arena config's openings slice |
| 16 `sealbot anchor platform suite` | `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` directly (`run_tests.sh:390`) |

**The handshake does NOT advertise book versions.** `crates/pistol-arena/src/identity.rs`
is engine identity — *"what it says about itself, and what it is by content"* —
and carries no book field. So R2's conditional golden refresh does not arise:
the golden transcripts (`instrument_golden_v1.txt`, `golden_boards_v1.txt`,
`golden_games_v1.txt`) are engine and core artifacts and a new book does not
touch them.

### P9 — hard rule 8

> **Artifacts.** Nets, books, match logs, bench outputs are never committed; a
> committed manifest may sha-index them.

**And the rule does not classify this object — see §11.**

## R1 — the sizing measurement

Instrument: `crates/pistol-arena/examples/sprt_power.rs` at
`0a430d2e3e98430d2cd98d28765e79939c9b0f51`, release build, run rather than only
computed (D-628 §7, BV3-1).

Bounds: `elo0 0`, `elo1 10`, `alpha = beta = 0.05`, `--buckets 2,3,8,7,4`,
`--runs 20000`, `--seed 1`, `--truth 10`.

**Instrument cross-check.** The two caps POWER.txt already carries reproduce
exactly: 4000 gives `h1 13940 (0.6970)` and 8000 gives `h1 18090 (0.9045)`,
digit for digit against the Phase 1 artifact.

**The sweep** (`artifacts/book_v3/POWER_v3.txt`, digest
`ea77d4416e8d2f39c89d7a4d7aa7319cfc78e5c8022cb524c49628fe612d3dbf`):

| pair cap | power (`h1`, truth = 10) |
|---|---|
| 4000 | 0.6970 |
| 4500 | 0.7462 |
| 5000 | 0.7867 |
| 5233 | 0.8045 |
| 5500 | 0.8187 |
| 6000 | 0.8451 |
| 6500 | 0.8635 |
| 7000 | 0.8794 |
| 7250 | 0.8863 |
| 7500 | 0.8927 |
| 7750 | 0.8992 |
| **7800** | **0.9001** |
| 7850 | 0.9020 |
| 7900 | 0.9022 |
| 7950 | 0.9036 |
| 8000 | 0.9045 |

Alpha side at the top of the grid: 7750 `h0 0.9004`, 8000 `h0 0.9053`.

- **`P` = 7800 pairs — MEASURED.** The smallest cap in the swept grid reaching
  power ≥ 0.90; 7750 answers 0.8992 and is below.
- **Floor = 5233 pairs — DERIVED.** `ln(19)² / t1²` with
  `t1 = 10 · ln(10)/800 · √2 = 0.040704`, which is the instrument's own printed
  `tilted_t`. This reproduces `training_pipeline_2026-09.md` §5's figure exactly.
  **The floor does not bind**, the measured number being larger.
- **`pairs_v3` = max(7800, 5233) = 7800 — MEASURED.**
- **`openings_v3` = ceil_to_500(7800 + 500) = 8500 — DERIVED**, via P2's §4 rule.

**THE DERIVED SIZE IS INSENSITIVE TO GRID REFINEMENT, WHICH IS WORTH MORE THAN
ANY SINGLE GRID POINT.** `ceil_to_500(P + 500)` returns **8500 for every `P` in
`(7500, 8000]`**, and the 0.90 crossing is bracketed inside `(7750, 7800]`. So a
finer sweep cannot move `openings_v3`, and the number does not depend on where
in that band the crossing is pinned.

BV3-1's fork — *"instrument says more than 2x that, STOP"* against an expected
8,000 pairs / ~8,500 openings — **does not fire**: 7800 and 8500 are the
expected order.

**Cost, with its seat named (D-479, BV3-7).** At cap 8000 the instrument
measures `mean_pairs 3102.1` — the expected pairs a run actually plays, not the
cap. The `~4.5 h` of the handoff is an SPRT RUN cost at the 2.046 s/opening
instrument seat, not a generation cost. **Generation cost was not measured at
R1, because no generation had been run then**; it was measured at R2 and is
0.036 s — see §12. (Corrected: this sentence said generation cost "was not
measured" full stop, which R2 made false. REVIEW-impl m-9.)

## §11 — THE BLOCKING FINDING

**`book_v3` is not a book under hard rule 8. It is a fixture under hard rule 7,
by this repository's own written test — and BV3-3, BV3-5(c) and D-645 are built
on the other classification.**

D-151 settles the class, and says it was written down for exactly this kind of
question:

> `openings_v1.txt` is a FIXTURE under CLAUDE.md rule 7, not a book under rule 8,
> and the distinction is written down here because it is exactly the kind rule 10
> exists for. [...] What makes it a fixture is what is in it: [...] No engine
> judgement, no eval, no search result, no solved value. A book is a store of the
> engine's OWN conclusions and is regenerated whenever the engine changes [...]
> a balance-filtered book WOULD be a book.

D-175 applies that test to the seeded random generator — this generator, the one
BV3-2 says v3 uses unchanged but for the seed:

> That arithmetic is also what keeps this file a FIXTURE under rule 7 rather than
> a book under rule 8, by D-151's own test: no engine judgement went into it, and
> D-151 says in as many words that a balance-filtered book WOULD be a book. The
> filter question reopens the day k grows past five, and it reopens as a rule-8
> question and not only as a curation one.

`k_stones` stays 5 under BV3-2, and there is no balance filter. So v3 is v1's and
v2's class. **A CITATION IS STRUCK HERE RATHER THAN QUIETLY DROPPED.** Revision 1 argued
that `tools/artifact_check.sh` (gate 5) *"enforces rule 8 by name pattern and
content signature and passes today with both books committed"*, and offered its
green as evidence for the fixture class. **It is not evidence.** Gate 5 carries
content signatures for `arena_report` and `baseline_snapshot` and for nothing
else; it cannot recognise an opening book at all, so its green is equally
compatible with both books being rule-8 artefacts wrongly committed — which is
verbatim D-203's recorded history, *"a report committed as `report.txt` sailed
past them"*. The classification rests on D-151 and D-175, which is enough.

**Four things follow, and each is a cost BV3-3 pays without saying so.**

1. **The digest protection BV3-3 asks for already exists — as ONE RUN-TIME
   REFUSAL AND TWO GATES, and a manifest would be a weaker fourth.**
   *(Corrected after the DECISION-RED-TEAM: revision 1 called all three
   "refusals". F4 and F5 fail CI; they refuse nothing at run time. D-645 asks for
   a refusal "with a named error", and only the first of the three answers it.)*
   - **In-band.** Every book carries `# body_sha256 ` in its own header;
     `openings::load` recomputes the body digest on **every load** and refuses
     with the named `ArenaError::OpeningsDigest { path, claimed, found }` (hard
     rule 3 satisfied).
   - **Out-of-band.** The digests are pinned as source constants —
     `RANDOM_OPENINGS_V1_SHA256` and `RANDOM_OPENINGS_V2_SHA256`
     (`crates/pistol-cli/tests/random_openings_document_tests.rs:13,17`) — and
     that test names its own reason: *"The out-of-band pin, the convention
     `tactical_v0.txt` and the corpus fixtures already use (CLAUDE.md rule 7).
     It catches strictly more than the in-band digest: an edit that rewrote the
     body AND its `# body_sha256` line is self-consistent and is caught only
     here."* **That comment is a fourth in-tree statement that these books are
     rule-7 fixtures**, and it is the sha-indexing rule 8 would have wanted,
     already present.
   - **By regeneration.** `a_v2_run_reproduces_the_committed_v2_book_byte_for_byte`
     (`random_openings_cli_tests.rs:285`) and its v1 sibling
     `random_openings_binary_writes_the_committed_book_byte_for_byte` execute the
     config's own regeneration instruction and compare digests, in gate 3.

   A manifest adds a fourth path and is the weakest of them: a stale manifest
   cannot detect a book edited in place, where the out-of-band source pin catches
   even a self-consistent rewrite of body and header together.
2. **`BookVersion` cannot carry the behaviour BV3-3 assigns it.** It is write-side
   only (P1) and pistol-arena has zero references to it. "V3 resolves through the
   manifest" has no site to live at.
3. **An uncommitted v3 loses the pinning test v1 and v2 both have.** BV3-2 requires
   that regeneration from the committed seed reproduce the digest. For v1 and v2
   that is a STANDING GATE-3 TEST over committed bytes. Under BV3-3 it degrades to
   a one-time receipt, and gate 3 can no longer see the book at all.
4. **BV3-4's zero-overlap demand is stricter than any book here has met, and the
   filter that would achieve it may itself be the flip D-151 names.** v2 against v1
   is 10 positions up to symmetry, recorded as *"what chance gives"* and pinned by
   a test (P3). Reaching 0 requires a rejection filter. Filtering against the
   corpus `key_full` set is a curation step whose criterion is derived from
   **engine play** — the corpus is what the engine reached in labelled games — and
   D-151's flip clause is *"the moment a curation step consults an engine."*
   Whether that flip fires is precisely a rule-8 question, and it must be answered
   BEFORE the generator runs, not after.

**Why this is a stop and not a fork taken.** BV3-9 waived the OPTION MATRIX and
the DECISION-RED-TEAM on the ground of *"zero free parameters"*. P1 falsifies that
ground: where v3's bytes live, and which digest path governs them, is a named
decision with more than one viable option and different failure modes — which
CLAUDE.md's Process section says is settled by an OPTION MATRIX attacked by a
fresh-context DECISION-RED-TEAM **before** selection, and calls an option adopted
without one *"the same breach as silent architecture drift."*

It is also not an OPERATOR OVERRULE: that instrument is for when the CODE is done
and a document blocks it. No code was written.

**D-645 is in the append-only log and cannot be edited.** It was appended
verbatim per §0.2 before this finding was reached. Whatever the ruling, it is
recorded by a superseding line, not by a correction to D-645.

## Receipts

| artifact | digest |
|---|---|
| `artifacts/book_v3/POWER_v3.txt` | `ea77d4416e8d2f39c89d7a4d7aa7319cfc78e5c8022cb524c49628fe612d3dbf` |
| `artifacts/wp22_phase1_quiet/POWER.txt` (input) | `26ca6f3095421340764dfb98478f700a110a717a68ed09630d7d93340db909e8` |
| `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` | `895a05edb53a0a8d89c262bb058e3bc3dd24d446405d375458aaf067e2f076e7` |
| `crates/pistol-cli/tests/fixtures/random_openings_v2.txt` | `829361a9ae61d0d4369b5291bfc893133fa8160867f11cc638b11f432b6cc29a` |

Artifacts are gitignored (`.gitignore:19:/artifacts/`) and are not committed,
per hard rule 8.

## §12 — R2, the book itself

Built at `84daf0d`+ under D-647's Option D. **The generator is unchanged**: the
filter runs after generation, which is what keeps this package inside the
dispatch's "seed and version tag" limit.

### The fixed point, and why there is no truncation

`n_openings = 8538` is chosen so that **exactly** 8500 survive the filter. The
builder REFUSES any other survivor count and names the `n_openings` that would
land on it, rather than truncating — a book shortened in silence is a sample size
nobody chose (rule 6). So the header's two numbers differ by exactly the
rejection count:

```
build_book_v3: 6490 distinct canonical forms excluded
build_book_v3: drew 8538, rejected 38 already held, 8500 survive
```

6490 is |v1 ∪ v2| = 2000 + 4500 − 10, the 10 being the v1/v2 overlap the
committed pin records.

### Receipts

**BV3-2 — regeneration from the committed seed reproduces the digest.** Two
fresh processes, byte-identical:
`9453763625c83a8a95d31bf5ee62e5ec6db91552d0fe81a604b190067e32f240`. Pinned
out-of-band as `RANDOM_OPENINGS_V3_SHA256` and re-derived in-process by
`random_openings_v3_is_what_this_build_produces`.

**BV3-4 — disjointness, four counts, both terms named (D-479).** From
`tools/book_v3_disjointness.sh`, which shares no code with the generator:

```
book_v3_disjointness: input 895a05ed…f076e7  …/random_openings_v1.txt
book_v3_disjointness: input 829361a9…6cc29a  …/random_openings_v2.txt
book_v3_disjointness: input 94537636…32f240  …/random_openings_v3.txt
book_v3_disjointness: input 00f61780…f35968  …/deduped_manifest.txt
book_v3_disjointness: control: corpus5 ^ v2: 3487 of 3487 (the renderings agree)
book_v3_disjointness: v3 vs v1: 0 of 8500
book_v3_disjointness: v3 vs v2: 0 of 8500
book_v3_disjointness: v3 vs corpus: 0 of 8500
book_v3_disjointness: v3 internal: 0 of 8500
```

**The control line is not decoration.** This script renders a key and compares it
against a column the ARENA wrote. Had the two renderings drifted apart, every
corpus comparison would find nothing and report `0 of 8500` — a pass produced by
comparing two vocabularies. The control fails the run as a VOID if any five-stone
corpus key is not a `book_v2` opening.

**BV3-5 — loadability, three artifacts.**

(a) The arena loads v3 and names it by content, from `smoke_report.txt`:

```
openings_file crates/pistol-cli/tests/fixtures/random_openings_v3.txt
openings_body_sha256 7df96afb355336c780579067668619d29a5bf709b516ddaa12f3b6f7450acd31
openings_take 4 of 8500
```

(b) A four-opening paired smoke match completed, distinct-n reported:

```
counts n 8 distinct_n 4 wins_a 4 capped 0 losses_a 4 forfeits 0 voids 0 decided 8
verdict inconclusive_degenerate
```

The verdict is the knowably-correct one for a self-match and is asserted rather
than admired: two identical deterministic engines score every pair 1-1.

(c) The digest refusal fires on a one-byte-altered copy, and the named error is
`OpeningsDigest` at exit 2 — **not** a manifest mismatch, because D-647 selected
the in-band header digest the arena already verifies on every load:

```
arena: OpeningsDigest: artifacts/book_v3/tampered_v3.txt: the header claims
body_sha256 7df96afb...450acd31 and the body hashes to ea8e75dd...11064c4;
the file has been edited since it was pinned
```

**BV3-7 — generation cost, seat named (D-479).** Generating 8500 openings costs
**0.036 s** on this workstation, release build. Five timed runs by the red team
on the same seat: 0.0344–0.0346 s. The handoff's "~4.5 h" is the SPRT RUN cost at
the 2.046 s/opening instrument seat and is not generation.

### What is NOT claimed

No strength claim, of any kind (hard rule 6). The smoke match's verdict is
degenerate by construction and measures nothing. No range of `book_v3` is
consumed: `docs/book_v3_ledger.md` records the book as whole, and records the
smoke match explicitly as a non-consuming loadability check so that a successor
may still draw `0..3` for a governed run.

## §13 — R4, the fix round

REVIEW-impl ran fresh-context against `c443720` and returned **FAIL**, with the
engineering sound and every binding number reproduced independently — the digest,
the four counts, the rejection arithmetic and the digest refusal were all
re-derived by the reviewer — but with the shipped artifact's own header stating
two things untrue of it. Report: `artifacts/book_v3/REVIEW_IMPL.md`. This is the
package's **one** fix round (D-481).

### What was wrong, and what it cost to fix

**M-1 (MAJOR) — the preamble named the wrong file for its own pin.** It said
`RANDOM_OPENINGS_V3_SHA256` and `random_openings_v3_is_what_this_build_produces`
live in `random_openings_document_tests.rs`. They live in
`random_openings_v3_tests.rs`. The sentence was v1's and v2's, copied — TRUE
there, false here, and §11 leans on exactly that out-of-band pin as the mechanism
that "catches strictly more than the in-band digest", so a successor would have
grepped the named file and found nothing.

**M-2 (MODERATE) — the preamble described a truncating filter the code refuses to
be.** It said "keep the first `openings` survivors in generation order". The
builder REFUSES a survivor count that is not the size asked for. The paragraph
also contradicted itself: under truncation the gap between the two header numbers
would be rejections *plus* discards, so its next sentence was true only under the
non-truncating code. **The matrix's own selected-option text said "truncate to
8500" too** — the implementation took the safer reading, and
`matrix_book_v3_storage.md` now follows the code and says so.

Both were fixed in one regeneration. **The file digest moved and the body digest
did not**: the preamble sits above `# body_sha256`, which covers only the body, so
`7df96afb…450acd31` is unchanged and **every load-side receipt in §12 stands
without being re-taken** — the smoke report and the `OpeningsDigest` refusal are
still the run they claim to be. Only `RANDOM_OPENINGS_V3_SHA256` and §12's file
digest moved, to `9453763625c83a8a95d31bf5ee62e5ec6db91552d0fe81a604b190067e32f240`.

### What else was closed

- **M-3** — the receipt identified none of its four inputs. It now prints each by
  sha256 before any count. The corpus was the sharp case: a 43 MB file outside the
  repository with no in-tree pin, so `v3 vs corpus` named one term and left the
  other unidentified, in a package whose own lesson (D-479) is the opposite. **An
  unplanned cross-check falls out of it**: the corpus digest the script prints,
  `00f61780…caf35968`, equals the value D-636 recorded when the corpus moved.
- **M-4** — the rebuild command the config documents was executed by no test; the
  suite re-implemented the composition instead. Two tests now drive the SHIPPED
  example: one asserts it writes the committed bytes, one drops an `--against` and
  asserts it refuses and writes nothing.
- **M-5, A SEVENTH MUTANT THAT SURVIVED** — `rejected` reached only a `println!`,
  so a drifted counter would have the tool announce one number while the book it
  wrote in the same run implied another. `the_filter_rejects_exactly_the_gap_the_header_states`
  pins it at 38 and at `n_openings − openings`.
- **m-1, m-2** — statements that could die under `set -e` with exit 1, which the
  script's usage block defines as "the answer is no", for environmental reasons;
  and a count the instrument could not produce was spelled `fail` where item 12
  makes it a VOID. Both corrected, plus one dead line removed.
- **m-3, m-4, m-6, m-7, m-8, m-9, m-10** — claims stated more strongly than the
  code supports, corrected rather than defended: the `SurvivorCount` suggestion is
  a STEP toward the size and not a guarantee; "shares no code with the generator"
  becomes "shares no draw, no filter and no rendering" (it does share
  `canonical_form`, and must, since that is the identity D-644 is stated over);
  `Unreadable`'s doc now covers its use on a draw index; `unwrap_or(i64::MAX)`
  becomes this crate's own `unreachable!` convention; §11's "generation cost was
  not measured" is corrected as now-false; and the v3 config no longer inherits
  v1's "ONLY home" claim unqualified, because the book's SIZE is the builder's
  required `--openings` and the schema never sees it.

### What was NOT changed, and why

The reviewer's own analysis is recorded where it clears the design rather than
only where it faults it: the control is sound in both directions and exit 2 is
the right class for its failure; `random_openings_v3_is_what_this_build_produces`
is a real check and not agreement by construction; no test passes if the filter
is removed; the script is clean against checklist item 11; and **given the
control, `v3 vs corpus: 0` is implied by `v3 vs v2: 0`** — D-647's proof,
mechanized. One observation is carried forward rather than fixed: D-647 states
`v1 ∩ v3 = 9`, `v2 ∩ v3 = 29`, `corpus₅ ∩ v3 = 23` for the CANDIDATE draw, where
the shipped book's values are 0/0/0. The log is append-only and the surrounding
clause supplies the context; it is noted here for a successor rather than
rewritten.
