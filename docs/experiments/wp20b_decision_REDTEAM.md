# WP-2.0b — DECISION-RED-TEAM on the two option matrices of `wp20b_design.md` revision 2

**Revision attacked:** `9c4f702edc25df8c0f5d021bacf188ed10f40374`, a `git stash create`
object holding uncommitted work on top of HEAD `a56449b` (`dev`). It is NOT a commit on
any branch and does NOT match HEAD.

**Does it still match the tree?** YES, for every tracked byte. At the end of this review
`git diff 9c4f702 --stat` prints nothing. `git status --porcelain` shows the same four
paths it showed at the start (`docs/decisions.md`, `docs/experiments/wp20_dispatches.md`
modified; `docs/experiments/wp20b_design.md`, `docs/experiments/wp21_DISPATCH.md` added)
plus **one untracked file that appeared during this review and is not mine**:
`docs/experiments/wp20b_design_rev2_REVIEW.md`. That is a concurrent REVIEW-design
session's output. It changes nothing about the revision attacked here, and this report
was written without reading it.

**Scope.** §2 (DECISION 1, the identity form) and §3 (DECISION 2, the token's location).
§1's three premise findings were read first, because F2 sets §2's option field and hard
rule 1 sets §3's. Neither selection has been taken.

**What I read.** `docs/experiments/wp20b_design.md` in full; `CLAUDE.md`;
`docs/experiments/wp20_dispatches.md` §"WP-2.0b v2"; `docs/experiments/wp21_DISPATCH.md`;
`docs/experiments/wp20s_design.md` §8 and its key definitions; `docs/decisions.md`
D-6, D-8, D-88, D-291, D-423, D-424, D-512, D-516, D-518, D-527, D-530, D-534, D-537,
D-539, D-542, D-544, D-551, D-553, D-558, D-559, D-560, D-561, D-562, D-563 (D-401 not
read). Source: `crates/pistol-core/src/{symmetry.rs, zobrist.rs, state.rs, board.rs}`,
`crates/pistol-search/src/{census.rs, pvs.rs, search.rs, info.rs, lib.rs}`,
`crates/pistol-search/examples/trigger_census.rs`,
`crates/pistol-cli/src/{budget_token.rs, protocol.rs}`,
`crates/pistol-cli/tests/workspace_shape_tests.rs`,
`crates/pistol-arena/src/{labels.rs, capture.rs}`,
`crates/pistol-engine/src/{config.rs, position.rs}`, all 38 files in `configs/`.

**What I could not check.** Whether the concurrent REVIEW-design found the same things
(not read, by design). The corpus-class-versus-labelling-seat gap in my own throughput
measurements: I measured on `bench_positions_v1.txt` and `bench_solver_positions_v1.txt`,
which are bench fixtures, not the sweep's own corpus positions — my ratios are brackets,
not the sweep's number. I did not run `tools/ci.sh`; no code changed.

**Measurements taken for this review** (in a detached `git worktree` at `9c4f702` under
`/home/tom/.cache/`, own `CARGO_TARGET_DIR`, removed afterwards; `--release`):

| tag | what | result |
|---|---|---|
| R-CHECK | `canonical_form` and the C2 fold over 2 000 random positions × all 12 of `Symmetry::ALL` (24 000 pairs), plus play-order permutation and colour swap | both G-invariant, order-invariant, colour-swap distinct — **no counterexample** |
| R-COST | the C2 fold at 80 stones, 200 000 reps | **22.99 µs/call** |
| R-FIRE-A | `trigger_census --nodes 400000 --cap 2048 --gate on`, trigger-rich fixture, 3 entries | firings **95 / 97 / 102**; proofs 0/0/4; wall **147.99 s** |
| R-FIRE-B | the same on the corpus/regression fixture | firings **134 / 159 / 107**; proofs 0/0/0; wall **52.75 s** |
| R-OFF-A | the same trigger-rich entries, `--gate off` | wall **4.11 s**, `search_nodes 400 384` each |
| R-OFF-B | the same corpus entries, `--gate off` | wall **3.63 s**, `search_nodes 400 384` each |

---

## VERDICTS

### §2 — DECISION 1, the identity form: **STANDS WITH FINDINGS**

**I would still select C2.** I could not find an option that reaches D-537's count more
cheaply or more correctly, and C2's correctness claims are true — verified, not conceded
(R-CHECK, and R11 below). But the matrix decides on an axis whose arithmetic is wrong by
~3× in the recommendation's own favour (R1), and **both** of the two reasons it gives for
C2 over C1 are weaker than stated — one of them rests on a property with zero uses in the
workspace (R2). A fifth option the matrix did not consider is cheaper on the cost row it
does price and is not eliminated anywhere in the document (R3). None of that moves the
selection; all of it has to be in the record before the ADR line is written.

### §3 — DECISION 2, where the token lives: **STANDS WITH FINDINGS**

**I would still select T1, and T3 only once its sink is named.** T1+T3 is the right place
for the token. But T3 as specified is inoperative: the arena's capture reader *discards
every `info` line* and has no census sink, so a `--capture --census` run can set the
token, receive every row, write a corpus with zero census bytes, and exit 0 (R7,
BLOCKING). And the T2 elimination is argued against an option WP-2.1 did not name — its
words are *"the pipeline's experiment config"*, which is the **arena's** config, not the
engine's TOML the matrix prices (R9).

---

## FINDINGS

### BLOCKING

#### R1 — §2's artifact arithmetic is wrong by ~3×, in the recommendation's favour, and the "six times the wire" claim that carries C2 over C1 is really ~2.9×

The matrix's `bytes on the wire per row` row gives `32 hex` for C2 and `~600 B` for C1 —
those are the **identity field's** bytes. The `census artifact for the full sweep` row
then converts them as if they were the **row's** bytes: 2.6·10⁷ × ~96 B = ~2.5 GB for C2,
2.6·10⁷ × 600 B = ~15 GB for C1. The two conversions use different bases (C2's adds ~64 B
for the rest of the row; C1's adds nothing), and neither base is right, because §4's own
field list is ~250 B of names and counts that **both options pay identically**.

Counted from §4's field order with plausible values (13 counts, `cover minimal`,
`cover_count 12`, a 32-hex key, one newline):

```
minimum row (all counts single digit, cover none, defender '-')   281 B
typical row                                                        291 B
C1's row = the same line with 32 hex replaced by the design's own
  ~600 B stone list                                                859 B
```

So over the design's own 2.6·10⁷ rows: **C2 ≈ 7.6 GB, C1 ≈ 22.3 GB**, and the ratio is
**2.95×, not 6×**. Using my MEASURED firing rate instead (R4: 95–159/ask, mean ≈ 116),
256 372 asks gives ≈ 3.0·10⁷ rows and **C2 ≈ 8.7 GB, C1 ≈ 25.5 GB**.

Why this is BLOCKING rather than a tidy-up: the wire-bytes row is one of only two reasons
§2 gives for C2 over C1 (*"C1 costs six times the wire"*), and the multiple is wrong by a
factor of two. The other reason is R2. The selection survives — 8.7 GB against 25.5 GB is
still a real difference and C1 has costs C2 does not — but the ADR line may not quote a
number this document produced.

**Corollary the matrix should own.** The identity form contributes 32 B of a 291 B C2 row
and ~570 B of an 859 B C1 row: on C2's side the axis the matrix decides on is **89 %
determined by a variable the matrix does not vary** — the row format. §4 already pins
field order with a report test, so a header-plus-positional-values line is compatible with
the D-88 precedent and would cut the sweep artifact from ~8.7 GB to ~2.5 GB. That is not
this matrix's decision, and it is exactly the decision whose absence makes this matrix's
cost axis unreadable.

#### R2 — the second reason for C2 over C1 rests on a `Copy` that nothing in the workspace uses

§2: C1 *"breaks `Copy` on a struct whose `Copy` the premise memo already flagged as
load-bearing"*, quoting §P2.4: *"the struct derives `Copy` … so today it cannot hold a
`Vec` or a `String` without a shape change"*.

That quote says what the derive **permits**. It does not say anything is using it. One
`git grep` over every mention of `TriggerObservation` outside its own module gives eleven
sites, and **not one of them requires `Copy`**:

- `crates/pistol-search/src/pvs.rs:746` and `search.rs:767` — `Vec::push` of a
  freshly-constructed value; a move suffices.
- `pvs.rs:739-741`, `search.rs:760-762` — by-value parameters; a move suffices.
- `pvs.rs:76`, `search.rs:80`, `search.rs:216` — `Option<Vec<…>>` fields and a drain.
- `search.rs:311` reuses `attacker_answer` — that is **`TriggerAnswer`**'s `Copy` (a
  two-field POD), not `TriggerObservation`'s, and C1 would not touch it.
- `crates/pistol-search/examples/trigger_census.rs:215-241` iterates
  `take_trigger_census()` by value.

So the cost row `TriggerObservation: Copy` → **broken** is a cost on a property with zero
uses. The C1 penalty that is real — an allocation and a `String` build **inside the census
closure**, per firing, on the search path — is nowhere in the matrix. The conclusion
(C2 beats C1) survives; the reason given for it does not.

#### R7 — T3 can turn the census on and produce nothing, silently: `arena --capture` discards every `info` line and the design names no sink

`crates/pistol-arena/src/capture.rs:165-179`, `classify`:

```rust
if line.starts_with(&format!("{} ", pistol_cli::report::INFO_PREFIX)) {
    return Step::Ignore;
}
```

and `ask` at `:227`, `Step::Ignore => continue`. The capture loop reads until `bestmove`
and keeps **only** the totals line. `info census …` starts with `info `, so under T3 as
specified the engine emits every row and the capture pass throws all of them away — no
error, no refusal, exit 0, and a corpus with zero census bytes.

§6's diff row says only *"the `--capture` flag that emits the token, and the census rows
written beside the capture"*. Where "beside" is, what writes it, and what change to
`classify`/`ask` carries the rows past `Step::Ignore` are not stated. `classify`'s own doc
comment says it is *"a pure function so the refusals INVARIANT 9 promises can be pinned"*
— so the change lands on a seam WP-2.0 pinned with tests, and the diff table does not list
it.

Why BLOCKING and not MAJOR: this is **F3's failure mode arriving by a second route, from
inside the package written to prevent it**. F3 stops the sweep because a switch nobody set
means zero rows; R7 means the switch can be set, honoured by the engine, and still give
zero rows. §7's invariants and §8's tests cover neither — test 5 pins *token absent ⇒ zero
bytes* and there is no test pinning *token present on an armed seat ⇒ rows reach the
capture's output*. Under D-553's own reasoning this is the call-removed mutant nobody
listed: the census writer can be correct and its call from the arena never made.

**Consequently §3's claim that "T3 is T1 plus the caller" is false.** T3 is T1 plus a
caller **and a sink**, and the matrix prices the caller.

#### R8 — F3's headline is false as written: three committed configs arm the solver, and the design and D-563 both state the opposite in bold

§1 F3's section title is *"THE CENSUS CANNOT FIRE AT ALL UNDER ANY COMMITTED CONFIG"* and
its body states *"**The gate is `false` in every committed config**"*. D-563, in this same
revision, carries the same claim in its own title.

`git grep -n "^on_search_path = true" 9c4f702 -- configs/ | LC_ALL=C sort`:

```
configs/bench_wp18c_solver_on.toml:45:on_search_path = true
configs/gate_staged_solver_v0.toml:47:on_search_path = true
configs/play_staged_solver_v0.toml:75:on_search_path = true
```

Three of eighteen configs that carry the key set it `true`. F3's **narrow** claim — that
`configs/instrument_v0.toml:113`, the config both pilot seats ran and the one D-560's cost
model is extrapolated from, has the gate off — is TRUE, and the sweep-blocking conclusion
survives intact. What does not survive is the universal.

Why BLOCKING: this is not a wording slip, because two of §8's and §9's obligations turn on
it.

1. **§9's bench guard is vacuous on a gate-off seat.** *"one registered nps spot-check
   ON-token vs OFF at 50 000 nodes — and this is where §2's ESTIMATED per-firing cost
   becomes MEASURED"*. On any of the fifteen gate-off configs the census never fires, the
   closure is never entered, and ON-token and OFF-token are the same run to the byte. The
   guard passes and measures nothing. That is D-527's named defect class verbatim — *"a
   criterion that is a property the named defect class PRESERVES"* — and the design does
   not name the seat the guard runs on. `configs/bench_wp18c_solver_on.toml` is the right
   seat and exists; the document must say so.
2. **Half of §8's test table can pass vacuously for the same reason.** Tests 1, 7, 8 and
   12 all assert something *about a row*. On a gate-off engine there are no rows, and an
   assertion over an empty row set is satisfied. The design names the seat for none of
   them. Test 9 (gate off ⇒ no row) is the only one whose seat is unambiguous.

The correction is one sentence plus a named config, and it must land before D-563 is
committed, because a successor reading D-563's title will conclude that no committed
config can produce a census row — which would remove the only seat on which this package's
own tests and its bench guard can mean anything.

### MAJOR

#### R3 — the option field is incomplete: a fifth option (D′) is cheaper on the cost row the matrix prices, and is eliminated nowhere

Four options are listed. Here is the fifth, and it is not a strawman:

> **D′ — the minimum image key.** `key = min over Symmetry::ALL of
> from_scratch_key(g·board, to_move, phase)`, i.e. fold `cell_key` over each of the twelve
> images and take the least `Key128`. `Key128` already derives `Ord`.

D′ reaches exactly the same equivalence as C2 — it is constant on symmetry orbits and (up
to a hash collision) separates them — so it reaches **the same count**, which is the ONE
criterion §2 registered. And it is cheaper on the very row the matrix prices C1 and C2 on:
C2 does 12 `apply` sweeps **plus 12 sorts of ≤ 80 elements plus 12 `Vec` allocations**
(`symmetry.rs:141-147`, `transform` allocates and `sort_unstable`s per image, and
`canonical_form` keeps the running best); D′ does 12 `apply` sweeps, 12 × 80 `cell_key`
calls, **zero sorts and zero allocations**, and can carry side-to-move and phase for free
by XORing `context_key` in. MEASURED, C2's per-firing cost is 22.99 µs at 80 stones
(R-COST) — dominated by the sorts and the allocations, which D′ does not do.

**C2 still wins, and the reason is not in the document.** D′ is a *fourth notion of
sameness*: its representative is "the position whose key is least", which is not
`canonical_form`'s representative, so nothing D′ produces can be pinned against
`canonical_form`'s already-reviewed semantics, and §2's own stated hazard (*"not a fourth
notion of sameness"*) applies to D′ with full force. C2 inherits `canonical_form`'s pinned
meaning, and that is worth more than the microseconds. The matrix states this hazard as a
property of C2 and never as the argument that kills its cheapest rival — because it never
names the rival.

The two shapes the dispatch's attack line asked about, for completeness, both fail and
should be recorded as failing:

- **canonicalise once per game, not per firing** — impossible. The minimising symmetry
  `g*` is not stable under adding stones, so a `g*` fixed at the root is wrong at every
  descendant. There is no incremental route: `stones_key` is carried incrementally
  (`board.rs:131`) but the key of `g·board` is not derivable from the key of `board`.
- **carry `key_pos` and fold symmetry offline at count time** — impossible. `key_pos` is a
  128-bit hash; no offline pass can recover the stones to transform them. It works only if
  the row also carries the stone list, which is C1.
- **carry BOTH `key_pos` and a symmetry tag** — costs strictly more than C2 (it still
  needs `canonical_form` to produce the tag) and buys only human readability of the
  *symmetry*, not of the position. Dominated.

#### R4 — §2's volume basis is measurable in one command, and this review measured it: "order 10²" is right, at 95–159, and the extrapolation is sound but was not taken

§2 marks firings-per-ask at the sweep budget **ESTIMATED at order 10²** from D-530's
MEASURED 26 firings at `nodes 50000`, and says *"tranche one must measure rather than
assume"*.

That measurement did not need tranche one. `crates/pistol-search/examples/trigger_census.rs`
exists at this revision, takes `--nodes`, `--cap` and `--gate on`, constructs its own
armed `SolverWiring` independent of any config, and prints firings per entry. Two commands
and four minutes on this machine (R-FIRE-A/B):

```
nodes 400000, cap 2048, gate on
  trigger-rich fixture:  95, 97, 102 firings
  corpus fixture:       134, 159, 107 firings
```

**The estimate is correct** — 95–159, mean ≈ 116 — so this is a D-291 marking finding, not
an arithmetic one. It is MAJOR rather than MINOR because of what else the same two commands
show, in R5 and R6, which the design needed and does not have.

#### R5 — the design knows F3's arming question changes D-560's price and never gives the magnitude; MEASURED it is 14.5×–36×, which turns a 63-hour sweep into 900–2 300 hours

F3's limb (2) and D-563's limb (2) both say only that the gate-on seat *"is a different
sweep at a different price, re-derived and not scaled"*, citing the mean 156 313 nodes
against a 50 000 budget. Same fixtures, same binary, same `nodes 400000` budget, gate on
versus gate off (R-FIRE vs R-OFF):

| fixture | gate off | gate on | ratio | `search_nodes` gate on |
|---|---|---|---|---|
| corpus/regression, 3 entries | 3.63 s | 52.75 s | **14.5×** | 3 138 / 40 162 / 13 882 |
| trigger-rich, 3 entries | 4.11 s | 147.99 s | **36.0×** | 15 619 / 5 464 / 24 614 |

Gate off, every entry spends its whole budget on the search: `search_nodes 400 384`. Gate
on, the search gets **0.8 %–10 %** of it and the solver absorbs the rest.

Two things follow that no document in this repository states:

1. **D-560's ~63 h book ceiling becomes ~900 h at the mild end and ~2 300 h at the
   trigger-rich end** — 38 to 95 days, single-threaded. That is what makes F3's question a
   STOP rather than a preference, and it is the number the operator needs to rule on it.
2. **The labels do not merely "come from a solver-consulting engine" (F3 limb 1) — they
   come from a search that saw 1–10 % of its nodes.** F3 argues from the *provenance
   column*; the stronger and simpler statement is that the label's own depth collapses.

This is MAJOR against §2 and §3 only indirectly — neither recommendation changes — but the
design's §10.1 hands F3 to the operator as *"the one question that must be answered"*
while withholding the only quantity that would let them answer it, and CLAUDE.md's own
D-291 clause is about exactly that: an estimate that could have been measured, in a
paragraph whose whole job is to size a decision.

#### R6 — the matrix prices the whole row volume against a criterion that counts a ~1 % subset of it, and the row-set decision is never opened

D-537 counts **win-proving firings on disjoint positions**. §2 prices 2.6·10⁷ **firings**.
The two differ by the proof rate and again by the disjointness fold, and the design never
says so.

MEASURED at the sweep's own budget (R-FIRE-A/B): across six positions from the two
hardest fixtures in the workspace, `proofs` came back **0, 0, 4, 0, 0, 0** — four proofs in
654 firings, ~0.6 %. And `proofs` is not the numerator either: D-530 records that on
`g001-t42-p2` *"a governed search from it at the bench cap finds six proofs and every one
is a proven LOSS"*, so win-proofs are a subset of an already-tiny count.

The consequence for the matrix: an option that writes a row only where it can enter
D-537's count — attacker-proved firings, keyed by C2 — reaches the registered criterion
**identically** and cuts the artifact from ~8.7 GB to well under 100 MB. That option is
almost certainly wrong, and the reason is good: the census's other and older purpose
(D-516's *"what FRACTION of the present trigger's firings does a predicate keep"*) needs
the denominator, so the unfiltered row set is load-bearing for the detector's option field
even though it is dead weight for D-537's clock.

But the row-set decision is the one that dominates the cost axis on which C2 was chosen
over C1 — filter to proved firings and C1 costs ~250 MB and the wire argument for C2
evaporates entirely. A matrix that decides the identity form on bytes, while an
undiscussed orthogonal decision moves those bytes by two orders of magnitude, has not
priced its own recommendation. §10 should carry the row-set question with its reason for
being answered "all firings", and does not.

#### R9 — T2 is eliminated against an option WP-2.1 did not name, and the elimination's own strongest evidence is not cited

§3's T2 row reads *"an engine config field — `[census] on = true` in the engine's TOML"*,
and the rule-1 collision is priced against *"every committed config"*.

WP-2.1's words, which §3 quotes as the tension it is answering, are: *"Census: ON via the
WP-2.0b token in **the pipeline's experiment config**"*. That is the **arena** config —
`crates/pistol-arena/src/config.rs`, the file `configs/arena_wp20_label_pilot.toml`
instantiates — not `crates/pistol-engine/src/config.rs`. There are 14 committed arena
configs and 18 committed engine configs; the two options have different costs and
different blast radii, and the matrix prices only one of them, then declares the tension
with WP-2.1 "named rather than resolved".

The elimination still holds for both variants, and my reading of rule 1 is that the
design's **reason** is right and understated. Rule 1 says *"a default lives in exactly one
schema place"*, which on its face permits a schema-side `#[serde(default)]`. It is
foreclosed here not by the rule's letter but by two landed tests the design does not cite:

```
crates/pistol-engine/tests/config_schema_tests.rs:21   "serde(default",
crates/pistol-arena/tests/config_tests.rs:186          for forbidden in ["serde(default", "impl Default for", "#[derive(Default"]
```

Both config crates have a test forbidding the string `serde(default` in the config module,
and both carry a `schema_version` that `validate.rs` refuses on mismatch. So adding a field
to either is a schema-version bump plus an edit to every committed file of that kind —
which is what the design says, arrived at by the mechanism it does not name.

MAJOR because the matrix's own stated purpose for the T2 row is to answer WP-2.1's words,
and it answers them about a different config.

#### R10 — §4's emission point is ambiguous in a way that multiplies the artifact by the iteration count, and §7/§8 pin only the internal row, not the wire line

§4: census lines are *"emitted with the per-depth `info` lines the `go` handler already
streams … before `info totals` and `bestmove`"*. §6: *"`SearchInfo` carries the run's
census rows when one was collected."*

`SearchInfo` is constructed **fresh inside the deepening loop** and handed to the reporting
callback once per completed depth (`crates/pistol-search/src/search.rs:401-416`), and the
`go` handler prints one line per callback
(`crates/pistol-cli/src/protocol.rs:169-172`). The census `Vec`, by contrast, accumulates
across the whole search and is drained once (`search.rs:216`, `take_trigger_census`). If
each per-depth `SearchInfo` carries "the run's census rows", every row is printed again at
every subsequent depth, and the wire volume is multiplied by the number of completed
iterations.

§7 invariant 3 says *"a firing has exactly one row"* — that is a statement about the
internal `Vec`, and it is already true today. Nothing in §7 or §8 says **a firing has
exactly one LINE**. D-530's 26 and my 95–159 are whole-search drains, so §2's entire
volume arithmetic silently assumes a drain semantics §4 does not state.

The count itself survives — counting distinct keys among proved rows is idempotent under
duplication — so this is MAJOR and not BLOCKING. Any per-firing rate read off the census
would not survive, and neither would R1's byte totals.

### MINOR

#### R11 — C2 is CORRECT, including on the side-to-move question, and the design does not give the argument that makes it so

Recorded as a finding because the design asserts §8 compliance without the load-bearing
step, and the step exists in-tree.

The concern is real on its face: `GameState::key` XORs `context_key(to_move, phase)`
(`state.rs:134-136`, D-8), and `canonical_form(stones)` cannot. So C2 drops two components
`key_pos` carries, and if two decision points differing only in side-to-move folded to one
key, D-537's count would be **UNDER**-stated — the opposite direction to F2's complaint,
and not discussed anywhere.

**They cannot fold, and pistol-core says why in its own words.** `GameState::key`'s doc,
`state.rs:129-133`:

> *"for an ongoing game the stone count fixes the turn, the phase and the mover together,
> so two positions this key cannot tell apart are the same position."*

And it is enforced, not merely asserted: `PositionSpec::Set`'s replay treats the stated
`to_move` and `phase` as *"**checked, not trusted**: the stone lists alone fix the whole
structure"* (`crates/pistol-engine/src/position.rs:102-104`, refusing at `:173-181`). So
no position with an inconsistent side-to-move can be constructed from any seam this engine
has. R-PARITY walked 400 successive states and found every `(p1_count, p2_count)` mapping
to exactly one `(to_move, phase)`. `canonical_form` preserves both per-colour counts
(colour is part of the sorted tuple), so C2 separates every pair `key_pos` separates.

Two further limbs of the same attack, both answered:

- **Colour swap.** `cell_key` takes the player (`zobrist.rs:87-93`), so a position and its
  colour-swap have different keys — verified over 2 000 random positions (R-CHECK), and the
  corpus's `render_key_full` spells colour for the same reason
  (`crates/pistol-arena/src/labels.rs:76-81`).
- **The XOR fold's collision class.** It is **identical to the one D-8 already accepts**:
  `Board::stones_key()` is itself a carried XOR of `cell_key` over the stones
  (`board.rs:126-133`), so C2 is `stones_key` of the canonicalised board and introduces no
  new collision family. An ordered hash over the sorted canonical list would be strictly
  stronger — it has no GF(2) subset-dependency family at all — but the birthday bound over
  3·10⁷ rows at 128 bits is ~10⁻²⁴, and adopting a stronger hash here would make the census
  key incomparable with every other key in the engine. The XOR is right.

C2's correctness is therefore established, not conceded — and the ~20 lines above are what
§2 owes and does not contain. §7 invariant 4 is stated over `canonical_form`, so it is
satisfiable without ever asking whether `canonical_form` is the right thing to be
invariant over.

#### R12 — §2's per-firing cost estimate is measurably wrong, and the bench guard registered to correct it will see 0.07 %–0.5 %

§2: *"**ESTIMATED** 12 transforms + 12 sorts of ≤ 80 stones ≈ 8·10³ ops"*, C2 being *"the
same, plus one XOR fold"*. MEASURED (R-COST): **22.99 µs per call at 80 stones**, roughly
7·10⁴ cycles — an order of magnitude above what 8·10³ ops implies in time, because the
estimate counts comparisons and ignores the twelve heap allocations `transform` makes.

Against the sweep this is harmless and should be said plainly: 3·10⁷ firings × 23 µs ≈
**13 minutes** over a 63-hour run. Against §9's guard it matters:

| seat | added cost | as a fraction |
|---|---|---|
| 50 000 nodes, 26 firings (D-530's, §9's registered budget) | 0.60 ms | ~0.5 % of a ~110 ms gate-off search, ~0.2 % of a gate-on one |
| 400 000 nodes, 116 firings | 2.7 ms | **0.30 %** of 885 ms |

§9 registers the guard as *"where §2's ESTIMATED per-firing cost becomes MEASURED"*. An
effect of 0.2 %–0.5 % is at or under the noise an IQR-gated nps bench resolves, so the
guard as written will return "no measurable difference" whether the cost is 23 µs or 230
µs. That is not a reason to drop the guard — it is a reason for its pre-registered bracket
(rule 5, owed at §9) to be an **upper bound the guard can falsify**, not a gain estimate,
and for the guard to run on `configs/bench_wp18c_solver_on.toml` (R8).

#### R13 — the `go` grammar's fourth-word refusal names the wrong word

`crates/pistol-cli/src/budget_token.rs:44-51`:

```rust
[kind, amount] => budget_of(line, kind, amount),
[_, _, extra, ..] => Err(protocol(line, format!("`{GO}` takes one budget, and `{}` follows it", quoted(extra)))),
```

The design's change adds an arm matching `[kind, amount, token]` where `token` is the
census word, keeping the catch-all for everything else. `go nodes 400000 census extra`
then falls to the catch-all, which binds `extra` to the **third** word and refuses naming
`census` — the one word on the line that is legal. Rule 3 wants the named error to name
the problem. One extra arm fixes it; the design should pin it, since §8's test 6 asserts
only that a non-token third word is refused *naming it*, which this case satisfies while
being wrong.

#### R14 — the handshake question D-88 raises and §3 does not answer

D-88 pins that *"the budget kinds the handshake advertises are derived by asking
`Budget::check_supported`, never restated"*, and that `id` lines carry *"whatever the
binary adds … so that a transcript records the instrument a claim was measured with"*.
Under T1, whether an engine honours the census token is discoverable only by sending a
`go` and reading a refusal. A driver — and, more to the point, a **transcript** — has no
record that the census was available. §3 discusses the grammar and not the handshake. The
D-88-shaped answer is an `id` line; the design should take it or state why not.

#### R15 — T3 changes `capture_sha256`, and the design does not say so

`crates/pistol-arena/src/capture.rs:103-109` digests
`capture_format` + `experiment_sha256` + `label_go <go_line>`. Under T1+T3 the census token
is part of the `go` line, so **every census-on capture has a different `capture_sha256`
from the otherwise identical census-off capture**. This is arguably correct behaviour — it
is a different instrument — but WP-2.1 registers tranches against these digests, and a
package that changes a landed identity function's inputs should say it is doing so. Note
also that `label_go_line` is built from `crate::config::BudgetSection::go_line()`, so the
token's spelling has to be threaded through the arena's **config module** even though §3's
matrix row reads `committed configs touched: none` — true of the files, not of the type.

#### R16 — §10 omits a decision a committed ADR in this same revision assigns to this package

§10 lists what the package does not decide, and item 2 defers the corpus's `key_full`
re-spelling to WP-2.0-S. But D-562 — in the same stash object — says, of its own dedup
default:

> *"the pilot's three keys AGREED — `key_seq = key_pos = key_full = 347` — so *three-key
> agreement* has never been read against a corpus where they disagree, and at the sweep's
> scale a transposition is exactly what makes `key_pos` differ from `key_seq`; **which key
> rules a disagreement is WP-2.0b's transposition question** and not this line's."*

D-562 hands "which key rules a disagreement" to WP-2.0b by name. The design neither
answers it nor lists it in §10. §2's answer to D-537 (`key_full`'s equivalence, spelled as
a `Key128`) is not obviously the answer to D-562's dedup question, because D-562(2)'s rule
is *"distinctness by the three keys agreeing"* and the interesting case is precisely when
they do not. Either §2's ruling settles it and should say so, or §10 gains an item.

#### R17 — `canonical_form` panics, and it is being put on the search path

`Symmetry::apply` calls `overflow(...)` rather than returning `None`
(`symmetry.rs:105-111`), and `transform` uses `apply`. The panic is unreachable by D-34's
own argument (some two thousand turns to reach `i16::MAX`), and it is census-only, so this
is genuinely minor. But §7 invariant 1 is *"the census cannot move a move"*, and a panic
moves every move after it by ending the search. If `canonical_key` is the pistol-core
function §2 proposes, its `# Panics` section is owed under CLAUDE.md's Code style, and the
invariant should read "cannot move a move and cannot end a search".

---

## THE STRONGEST SURVIVING ATTACK ON EACH RECOMMENDATION

Written in the form the surviving option's ADR line can quote (CLAUDE.md, Process).

### On §2's C2

> **C2 pays for a fold whose only measurement is zero.** The single number this project
> has on the symmetry fold's yield is the pilot's, and it is `key_seq = key_pos = key_full
> = 347` over 742 records (D-560) — symmetry merged **nothing**, and
> `canonical_sequence`'s own doc explains why a deterministic engine rarely produces
> mirrored lines: *"D-7's final tie-break is lexicographic by `(q, r)` and is therefore not
> symmetry-invariant"* (`crates/pistol-core/src/symmetry.rs:213-217`, D-137). Against that
> measurement, option A — `key_pos`, which F2 eliminates a priori — is free at the site,
> keeps `Copy`, keeps 32 hex, needs no new pistol-core function, carries side-to-move and
> phase, and reaches the same count. C2's answer is that the census population is
> **in-tree**, where a search tree generates symmetric transpositions by construction,
> and not the root population the 347 was measured over — which is correct, is nowhere in
> the document, and is itself a claim about a magnitude nobody has measured. **The
> recommendation therefore rests on an unmeasured belief that the in-tree fold's yield is
> materially above the root fold's measured zero.** Tranche one's first census is the
> measurement that settles it, and the two-line addition that would let it do so is to
> emit `key_pos` beside the canonical key on tranche one only, and compare the two distinct
> counts.

*(Why it does not overturn the selection: F2's asymmetry argument is sound in direction —
over-counting clears a FLOOR early, which is the failure D-537 was written to prevent, so a
fold whose yield is uncertain should be taken rather than skipped. And R6 shows the count's
numerator is ~1 % of the rows, so a fold that merges even a few positions moves a small
number by a large fraction.)*

### On §3's T1+T3

> **T1 spends the protocol's `go` grammar on a switch that, by the design's own F3, cannot
> produce a byte on the seat it was built for.** The census already has a working off-wire
> seam — `Searcher::collect_trigger_census` / `take_trigger_census`
> (`crates/pistol-search/src/search.rs:206-220`), driven by
> `crates/pistol-search/examples/trigger_census.rs`, which arms its own solver independent
> of any config and which this red team ran at the sweep's own budget to produce R-FIRE-A
> and R-FIRE-B. So the wire is a **second** output path for rows that already have one,
> and it widens a line kind D-88 pins and every future driver reads. F3 concedes that the
> labelling seat must be separately armed by an unruled operator decision before the wire
> can carry anything at all; if the census run needs its own seat regardless, the case for
> putting it on the `go` line rather than in the instrument that already collects it is
> never made. **The answer is D-562(3) — *"census ON from game one"* means the census must
> ride the corpus's own games, and the off-wire route would cost a second full sweep at
> R5's 14.5×–36× — but that is an argument from cost the matrix never makes, and it is the
> argument that decides the question.**

*(Why it does not overturn the selection: the governing dispatch's scope 2 mandates
*"census rows on the wire"*, so the wire is not this package's choice to unmake; and the
second-sweep cost is real and large. T1 is right. What is owed is the sentence saying so.)*

---

## SUMMARY TABLE

| # | class | one line |
|---|---|---|
| R1 | BLOCKING | §2's artifact arithmetic uses two different bases; C2 is ~8.7 GB not 2.5 GB and the C1/C2 ratio is 2.95× not 6× |
| R2 | BLOCKING | the second reason for C2 over C1 rests on a `TriggerObservation: Copy` that eleven sites use and none needs |
| R7 | BLOCKING | T3 has no sink: `capture.rs`'s `classify` returns `Step::Ignore` for every `info` line, so the census can be on and produce nothing, exit 0 |
| R8 | BLOCKING | F3's "the gate is false in every committed config" is false (3 of 18); §9's bench guard and four of §8's tests can pass vacuously on a gate-off seat |
| R3 | MAJOR | option D′ (min over the twelve image keys) is cheaper on the priced row and is eliminated nowhere; C2 still wins, for a reason the matrix does not give |
| R4 | MAJOR | the ESTIMATED firings-per-ask was one command away; MEASURED 95–159, so the estimate is right and the marking is the finding (D-291) |
| R5 | MAJOR | F3's arming question is handed to the operator without its magnitude; MEASURED 14.5×–36×, turning 63 h into 900–2 300 h |
| R6 | MAJOR | the cost axis prices all firings; the registered criterion counts a ~0.6 % subset, and the row-set decision that dominates the axis is never opened |
| R9 | MAJOR | T2 is eliminated as an engine-config field; WP-2.1's words name the arena's experiment config, and the landed `serde(default` bans are not cited |
| R10 | MAJOR | §4's emission point admits per-depth re-emission; §7/§8 pin one internal row, never one wire line |
| R11 | MINOR | C2 is correct including on side-to-move — verified — and the argument that makes it so (`state.rs:129-133`, `position.rs:102-104`) is absent |
| R12 | MINOR | per-firing cost MEASURED at 22.99 µs, ~10× the estimate in time; §9's guard will see 0.2–0.5 %, at or under its own resolution |
| R13 | MINOR | a fourth word after a valid token is refused naming the token, not the intruder |
| R14 | MINOR | D-88's handshake pattern is unanswered: no `id` line records that the census was available |
| R15 | MINOR | T3 changes `capture_sha256` via `label_go`, and threads the token through the arena's config type |
| R16 | MINOR | §10 omits D-562's *"which key rules a disagreement is WP-2.0b's transposition question"* |
| R17 | MINOR | `canonical_form` panics on overflow and is being placed on the search path; `# Panics` and invariant 1's wording are owed |
