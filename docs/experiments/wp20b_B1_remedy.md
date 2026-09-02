# WP-2.0b — B1's remedy, written where the mutation set can be checked against it

Round 3's BLOCKING **B1** (`wp20b_impl_REVIEW_r3.md` §2) is not a wrong answer in
the engine. It is that the package's own account of where the identity columns
could be exchanged was **false**, that the mutation set was registered to match
that account rather than the code, and that the criterion round 2 specified —
one assertion over an IN-TREE row against an externally derived referent — was
never written.

This document carries the three things the remedy owes: the **receipt** that
enumerates the sites, the **derivation** of the fixture the new assertion
replays, and the **mutants** registered at each site.

---

## 1. THE RECEIPT — the sites, enumerated mechanically rather than described

**D-568's standing law, in its own words**: *"a mutation set is specified against
CALL SITES ENUMERATED BY A `git grep` RECEIPT recorded in the mutation document,
never against prose, and a comment asserting a property about call sites is not
evidence of that property."* B1 is the case that earns it — `census.rs` said no
exchangeable call-site assignment remained, and two sites were assigning the
fields one by one three lines below.

Taken with `/usr/bin/grep` per CLAUDE.md's Environment rule (the agent shell's
`grep` is wrapped, D-265), sorted `LC_ALL=C`:

```
$ /usr/bin/grep -rn "key: keys\.\|key_pos: keys\." --include=*.rs crates/ | LC_ALL=C sort
crates/pistol-search/src/pvs.rs:763:            key: keys.key,
crates/pistol-search/src/pvs.rs:764:            key_pos: keys.key_pos,
crates/pistol-search/src/search.rs:813:        key: keys.key,
crates/pistol-search/src/search.rs:814:        key_pos: keys.key_pos,
```

**TWO PUSH SITES, AND THE PATTERN IS BROADENED ONCE SO THE RECEIPT CANNOT MISS A
THIRD.** A site that built a row without going through a local named `keys`
would not appear above, so the enumeration is re-taken over every construction
of the row type and of the key pair:

```
$ /usr/bin/grep -rn "TriggerObservation {" --include=*.rs crates/ | LC_ALL=C sort
crates/pistol-arena/src/bin/stub_engine.rs:177:            pistol_cli::report::census_line(&TriggerObservation {
crates/pistol-cli/tests/report_tests.rs:198:fn firing(defender: Option<TriggerAnswer>) -> TriggerObservation {
crates/pistol-cli/tests/report_tests.rs:199:    TriggerObservation {
crates/pistol-search/src/census.rs:53:pub struct TriggerObservation {
crates/pistol-search/src/pvs.rs:762:        census.push(crate::census::TriggerObservation {
crates/pistol-search/src/search.rs:812:    rows.push(crate::census::TriggerObservation {

$ /usr/bin/grep -rn "CensusKeys {" --include=*.rs crates/ | LC_ALL=C sort
crates/pistol-search/src/census.rs:15:pub struct CensusKeys {
crates/pistol-search/src/census.rs:25:impl CensusKeys {
crates/pistol-search/src/census.rs:30:    pub fn at(state: &pistol_core::GameState) -> CensusKeys {
crates/pistol-search/src/census.rs:33:        CensusKeys {
```

**THE CLASSIFICATION, so the two rows this remedy does NOT attack are refused
out loud rather than by silence.**

| site | class | mutant |
|---|---|---|
| `census.rs:33` — `CensusKeys::at`'s constructor | the DERIVATION | **M27** (already registered) |
| `pvs.rs:762` — the IN-TREE firing's push | a PUSH SITE | **M30**, **M32** |
| `search.rs:812` — the ROOT firing's push | a PUSH SITE | **M31**, **M33** |
| `stub_engine.rs:177` | a scripted FIXTURE, not a firing: it builds a row from `canonical_key` of a synthetic one-stone board and a from-scratch key, and no search reaches it | M20 (its own behaviour) |
| `report_tests.rs:198-199` | a test's own fixture builder | — |
| `census.rs:15`, `:25`, `:53` | type and `impl` declarations, matched by the pattern and not sites | — |

**AND THE ENUMERATION IS TAKEN A THIRD TIME OVER CONSUMPTION, because a receipt
over CONSTRUCTIONS alone answers only half the question** — a column can also be
exchanged where it is read:

```
$ /usr/bin/grep -rn "\.key\b" --include=*.rs crates/pistol-engine/src crates/pistol-cli/src crates/pistol-arena/src | LC_ALL=C sort
crates/pistol-arena/src/labels.rs:202:        key_pos: state.key().to_string(),
crates/pistol-cli/src/corpus/openings.rs:161:        if let Some(seen) = keys.insert(state.key(), stones.clone())
crates/pistol-cli/src/corpus/openings.rs:167:                state.key(),
crates/pistol-cli/src/random_openings/mod.rs:281:    if replayed.key() != state.key() {
crates/pistol-cli/src/report.rs:145:        row.key,
crates/pistol-engine/src/budget.rs:58:            return Err(EngineError::config(self.key(), "must be at least 1, got 0"));
crates/pistol-engine/src/budget.rs:64:                self.key(),
crates/pistol-engine/src/instance.rs:275:                        budget.key(),

$ /usr/bin/grep -rn "key_pos" crates/pistol-cli/src/ | LC_ALL=C sort
                                                            (no output)
```

**ONE census row consumer, and it consumes ONE column.** `report.rs:145` prints
`row.key` into the wire line; `key_pos` reaches no CLI source at all, which is
the design's *"NOT on the wire"* clause read off the code rather than off the
prose. **A single-column read has no pair to exchange**, so the consumption side
adds no push site — and its own removal is already M1's mutant. The three
`budget.key()` and `self.key()` hits are an unrelated `Budget::key`, and the
`labels.rs` / `corpus` hits are the corpus pipeline's own `GameState::key`, not
the census's.

---

## 2. THE DERIVATION — where the fixture in the new test comes from

The new assertion replays the position an IN-TREE firing happens on. **A census
row does not carry its position**, so the position was read off the firing site
itself, once, with instrumentation that is not part of the tree.

Worktree `git worktree add --detach <dir> d83ac01`, then in
`crates/pistol-search/src/pvs.rs`, immediately after
`let keys = crate::census::CensusKeys::at(state);` inside the census closure:

```rust
if std::env::var_os("PISTOL_DERIVE_INTREE").is_some() && from_root > 0 {
    let cells: Vec<String> = state
        .played()
        .map(|(cell, _)| format!("{cell}"))
        .collect();
    eprintln!("INTREE t={} n={} cells={:?}", from_root, cells.len(), cells);
}
```

driven by an example that runs the identity suite's own seat and workload —
`configs/gate_staged_solver_v0.toml`'s shape, `BENCH_B15_FIRST`, `go nodes 1500`
— printed:

```
INTREE t=2 n=19 cells=["0,0", "-1,1", "1,0", "0,1", "0,2", "-1,0", "1,-1", "0,-1",
                       "1,-2", "0,-2", "0,3", "-1,-1", "1,1", "-1,2", "-1,3",
                       "-1,4", "4,-2", "-2,3", "-3,3"]
ROW t=0 key=8adc7560f93b697753dc430a63ea6e8e key_pos=f5dfa2e6971c841a77648b0a57ef0317
ROW t=2 key=03017106e7fbac162e2edc7afc0b4dfc key_pos=506195b918c7cef3ee11c283cd5893ac
```

**THE DERIVATION REVISION IS `d83ac01`, AND THE SEARCH PATH HAS NOT MOVED SINCE.**
The remedy's own diff touches `crates/pistol-search/src/census.rs` in a `///`
comment and adds tests; `pvs.rs`, `search.rs` and everything the search reads are
byte-identical to `d83ac01`, and the release binary built after the remedy hashes
to `7a7a2347a7af56a103d0fa512bb6838361d258e3a146ac71ac9903c2c6ec6a6e` — the same
digest the closure receipts name. So the position read at `d83ac01` is the
position the shipped search still fires at.

**THE WORKTREE WAS RESET AND THE INSTRUMENTATION IS IN NO TREE.** What survives
is the nineteen-stone fixture in
`crates/pistol-search/tests/census_identity_tests.rs`, and **the test does not
trust it**: it replays those stones, computes both identities from `pistol-core`
alone, and requires an in-tree row to carry them. A fixture that stopped being
the firing position fails the test rather than weakening it.

**WHY THE CHEAPER ROUTES DO NOT WORK, recorded so a successor does not re-walk
them.** The principal-variation nodes are not firing sites — MEASURED at 1,500,
4,000 and 8,000 nodes, no PV prefix of `BENCH_B15_FIRST` matches any census row's
identity in either column — so the referent cannot be taken from the answer the
search returns. Enumerating the positions two turns from the root is not
feasible: the legal region is the union of radius-8 balls (CLAUDE.md rule 5) and
a turn is a PAIR.

---

## 3. THE ASSERTION — and why it is not another correlate

`an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on`.

Every in-tree check that existed before it compared a row's two columns **to
each other** (`assert_ne!(row.key, row.key_pos)`). An EXCHANGE of the columns
preserves that — exchanging two distinct values leaves them distinct — which is
how the option-A identity design F2 forbids reached every in-tree census row with
**73 tests across 8 suites green**.

The referent here is derived outside the search:

1. from a position **replayed stone by stone** in the test, not from anything the
   search returned. **This is the load-bearing limb** — an in-tree row's `key` is
   compared to a value the search had no hand in computing.
2. **Through the mirror**, which is what makes the compared value one only a
   SYMMETRY-FOLDED identity can equal: the asserted referent is `canonical_key`
   of the twelve symmetry images of those stones, and the test first requires the
   twelve to agree. `GameState::key` is not symmetry-invariant, so a column
   carrying it equals no image's canonical key. **What limb 2 adds honestly**:
   the twelve-image agreement is a property of `canonical_key` itself — already
   pinned in `pistol-core` by M3 — so it certifies the census only in the sense
   that this test fails loudly if the fold ever stops folding. The
   census-specific claim rests on limb 1.
3. The test **refuses a fixture whose two derivations coincide** (`assert_ne!`),
   because a position that cannot tell the columns apart cannot fail the mutant
   either.

The test then requires an IN-TREE row (`turns_from_root > 0`) whose `key` is that
value, and asserts that same row's `key_pos` is the replayed position's
`GameState::key`. An exchange at the in-tree push site leaves no such row.

---

## 4. THE MUTANTS, and the test each one must die at

Registered in `artifacts/mutants.py` — one per push site per defect class, as
round 3 required (*"a registered mutant at each of the two push sites, since a
mutant registered only inside `CensusKeys::at` certifies nothing about the sites
that consume it"*).

| mutant | site | what it does | registered dying test |
|---|---|---|---|
| **M27** | `census.rs:33` | the two derivations EXCHANGED in the constructor | `each_identity_column_is_the_derivation_it_is_named_for` |
| **M30** | `pvs.rs:763-764` | the two columns EXCHANGED at the IN-TREE push | `an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on` |
| **M31** | `search.rs:813-814` | the two columns EXCHANGED at the ROOT push | `every_census_row_carries_the_two_columns_that_function_produces` |
| **M32** CALL-REMOVED | `pvs.rs:763` | the canonical derivation never reaches the IN-TREE row (`key` takes `keys.key_pos`) | `an_in_tree_rows_identity_is_the_canonical_key_of_the_position_it_fired_on` |
| **M33** CALL-REMOVED | `search.rs:813` | the same at the ROOT row | `every_census_row_carries_the_two_columns_that_function_produces` |

MUTATION_RESULT_PLACEHOLDER

---

## 5. THE FALSE SENTENCE, DELETED

`crates/pistol-search/src/census.rs` claimed, as the reason the type exists:

> *"With the derivation here there is no call-site assignment left to exchange: a
> site hands over a state and takes back both columns, and any exchange has to
> happen inside `CensusKeys::at`, where a test pins each field against an
> independently computed referent."*

Both halves were false, and §1's receipt is why. It is replaced by what is true:
deriving both columns in one place removes the DUPLICATED derivation and not the
exchange, each firing site still names the two fields, each is attacked by a
mutant of its own, and what kills them is the in-tree assertion above. The same
half-sentence in `census_identity_tests.rs` — *"there is no call site left to
exchange them at"* — is deleted with it.
