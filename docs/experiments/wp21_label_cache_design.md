# WP-2.1 lever B — the label cache. DESIGN, revision 5.

> **ONE LINE.** `arena --capture` asks the engine at every asked prefix of every
> game, and the pilot MEASURED **742 asks over 347 distinct questions**. This
> package memoises, within one capture run, the answer to a question that run
> has already asked — keyed on **the `position` line's exact bytes** — so a hit
> reproduces the record's own bytes and the cache's soundness is hard rule 4 as
> CI gate 9 already enforces it (`tools/ci.sh:109-110`; the total is `:21`). It
> drops no record, changes no field, and must be BYTE-IDENTICAL to the pass that
> exists today.

**GOVERNING**: `docs/experiments/wp21_throughput_prereg.md` revision 3 §2, §2.0,
§4.2, §4.4, §4.5; `docs/experiments/wp21_prereg.md` revision 4 §4 and §5; D-576,
D-581, D-584, D-586. This document designs only what those name. Its revision
history is in `docs/experiments/arc3_ledger.md`, not here. **`file:line`
citations are at `adb2012`**, the tree before IMPL.

---

## 1. WHAT CHANGES

| # | site | change |
|---|---|---|
| 1 | `crates/pistol-arena/src/label_cache.rs`, new | `pub enum LabelCache { Off, On }`; `pub struct CaptureCounts { asks, records, hits, key_pos_collisions, key_full_collisions, fold_ms }`; a crate-private `Memo` holding the `BTreeMap<String, (String, String)>`, the two coarser-key sets and the counts. Its own file because a symmetry fold is not *"the capture ask"*, `capture.rs`'s registered subject in `docs/rule9_justifications.md` |
| 2 | `crates/pistol-arena/src/capture.rs`, `run` (`:326`) | takes a `LabelCache` MODE, builds the `Memo` inside the `with_seats` closure (`:338`) so the map's type is in no signature and no caller can share one; returns `(Vec<CaptureRecord>, CaptureCounts)` |
| 2a | `run`, first statement | **X1b**: a census-on sink with `LabelCache::On` is refused naming both, before `one_engine` (§3) |
| 2b | the prefix loop (`:341`), before the lookup | **X3**: the stray-line guard, moved here from `ask` (`:241-246`) so a hit pays it (§3) |
| 2c | the same loop, at the call to `ask` (`:343`) | **the LOOKUP**: a `position` in the memo takes the stored pair and makes no ask; a miss increments `asks` **here and nowhere else**, then asks |
| 2d | the same loop, after `normalise` (`:356`) | **the INSERT**: the post-`normalise` pair under `position`, and the miss's two coarser keys into their sets, bumping a counter for each key already present (§2.4) |
| 3 | `crates/pistol-arena/src/passes.rs` (`:82-96`) | threads the mode in and prints the counts line (§2.5) |
| 4 | `crates/pistol-arena/src/bin/arena.rs`, the `match words` (`:39-86`) | five reachable spellings of a capture line ending in `--label-cache`: `… --label-nodes n` (`:51`) and `… --census` (`:59-74`) exist; `… --label-cache` is new and legal; `… --census --label-cache` and `… --label-cache --census` are ONE or-pattern arm, **X1**, refused naming both words. Everything else falls to the catch-all (`:79-85`), which names neither |
| 5 | `crates/pistol-arena/src/usage.rs` | the word, that its absence means off, what the counts line means |
| 6 | `crates/pistol-arena/src/bin/stub_engine.rs` | `Behave::StrayAfterNewGame(n)`, spelled `stray_after_newgame <n>`: honest, and after answering the `go` that follows its n-th `newgame` it writes one unsolicited `info` line. Play sends one `newgame` per spawn and the capture one per ask, so at `n >= 2` play never reaches it — necessarily, since a play-pass stray forfeits (`exchange.rs:34`) and the report is captured with the config that played it. No engine in the tree writes an unsolicited line, so X3 has no test without this |
| 7 | `tools/cold_label_check.py` | the ten-sampled-record floor `wp21_prereg.md` §4 registers: fewer than ten sampled records in a class is a VOID (exit 2), not a pass. A named constant, printed in the void message. `--partition` landed at `f1acc57` |
| 8 | `crates/pistol-arena/tests/label_cache_tests.rs`, new; `tests/cold_label_check_tests.rs` | §5's rows; T8 beside the checker's existing cases |
| 9 | `docs/rule9_justifications.md`; `tools/governing_citation_check.sh` | `capture.rs`'s entry gains the lookup and insert as part of the ask decision; the `--proposes` entries go when their files exist |

**NOTHING ELSE.** No format version moves (D-572), no column, no manifest field,
`capture_sha256` untouched. The capture file carries no trace of the mode by
design — that is what §4.4 compares — so **the only record that a run was cached
is the printed counts line and the run log's verbatim command**.

---

## 2. THE MECHANISM

**2.1 The key is the question.** The key is the `String` `position_line` builds
for the prefix, the bytes the record's `position` field carries. A hit returns
the pair an identical `position`, after an identical `newgame`, under the one
`go` (`:333`, outside both loops), on the same process, already produced. It
folds nothing — not a symmetry, not a transposition, not a re-spelling of a turn,
which `Turn::pair` makes canonical by type — so the memo has no equivalence to
get wrong; every coarser key buys its saving with a class of wrong answer (D-576,
`matrix_label_cache_key.md`). A `BTreeMap`: no hasher, one string compare where
a hit replaces a ~885 ms search. **The strongest attack**: gate 9's five seats do
not include `configs/instrument_v0.toml`, none of its budgets is `nodes 400000`,
and no limb asks one position twice at the same `go` in one process — which is a
hit. §4.4's byte-identity run at the sweep's seat and budget takes that shape
~6 600 times a tranche, and that is what closes it.

**2.2 The memo is a mode.** `run` is `pub`, so an invariant about its callers
cannot be checked by reading it. The registration's invariant — built inside
`run`, dropped with it, never shared — is made structural: the mode goes in, the
counts come out, the map never crosses the signature. Within one `run` the
binary, config and `go` are fixed, so `position` is the only argument that varies
and the key is complete.

**2.3 The memo holds the post-`normalise` pair**, exactly the strings the record
carries, so a hit reproduces them by construction and the hit path cannot reach
`normalise`. The registration's §4.2 says *"the pair the engine returned"*, the
raw one; the two are equal through a pure function, and the normalised side is
fixed here so both paths build the record at one place from strings normalised
once. The divergence is recorded here and corrected in the registration's next
revision.

**2.4 The two counters.** On a MISS the memo also takes the prefix's stone list —
turn `i`'s cells under player `i mod 2`, built from the turns with no `GameState`
and no fallible step, `asked_prefixes` having proved the game legal — sorted (the
transposition fold) and through `pistol_core::canonical_form` (the symmetry
fold). `key_pos_collisions` counts misses whose sorted list an earlier miss
already had; `key_full_collisions` the same for the canonical form. Exact list
compares, never digests; the names are the corpus columns whose EQUIVALENCES they
fold, not their zobrist values. A stone-set collision is a canonical one too, so
`key_full >= key_pos`. **They decide nothing**: the cache keys on `position`
alone, so a defective counter misreports a number and cannot mislabel a record.
**Cost**: one sort and twelve images over at most 79 stones per miss, ESTIMATED
microseconds against a miss's search and MEASURED on every run as `fold_ms`, the
summed wall of the folds; memory under 20 MB ESTIMATED per tranche. **What
`key_full` will show before it runs** — the book's own prefixes fold at `k = 2`,
per tranche (`artifacts/arc3_opening_prefix_fold.txt`, D-584):

```
tranche  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16   sum
merges  42 51 48 55 47 48 50 44 53 48 51 60 45 56 48 46   792
```

Each tranche's count is read against its own floor, and what that reading
decides is **D-586**, which amends D-581's flip clause to this per-tranche
floor-relative form with a number for *"materially"*, corrects the matrix's
*"no extra search"*, and withdraws *"settles D-562(2)"* — the counters measure a
fold's yield and adjudicate nothing between keys.

**2.5 The counts line.** One stdout line beside the manifest row:
`arena: label cache on: asks A records R hits H key_pos_collisions P key_full_collisions F fold_ms M`,
or `arena: label cache off: asks A records R`. **`asks` is a counter incremented
immediately before the call to `ask`, never derived from the records or the
memo**: derived as `tools/label_cache_count.py` derives the same name, it would be
a function of the capture file, which is identical cached or not, and T2 would go
green on a cache that never ran. `hits` is `records - asks`.

---

## 3. THE REFUSALS

| # | refusal | the defect it excludes |
|---|---|---|
| **X1** | `--label-cache` with `--census`, either order, refused at the argument parse naming both words, before `outpath::claim` (`:89`) so no file is left behind | a hit performs no search and emits no census row: a cached census capture writes fewer rows than positions asked, indistinguishable from a quiet search |
| **X1b** | the same refusal as `run`'s first statement | the CLI arm is another crate; `run` is `pub` and the combination is spellable at the seam where the damage is done — a guard in one file with its consequence in another, the shape `capture.rs`'s rule-9 entry names |
| **X3** | `channel.unsolicited()` at every prefix, before the lookup, moved out of `ask` | a guard that stops running on the ~53% of prefixes that hit: a stray in the pipe at a hit is read as the answer to a LATER miss, or never |

X3's residual: nothing checks the pipe after the final prefix of the final game,
in either pass; the hoist restores parity and does not close that. The census-row
guard inside `ask` is untouched and moot — X1b makes a cached run never a census
run at the seam. There is no *"cache disagrees"* refusal: a self-check that
re-asked would cost what it saves, and §4.4 is the external referent.

---

## 4. THE COLD CHECK

`wp21_prereg.md` §4 owns the criterion and the hit/miss derivation; row 7 is this
package's part, the floor that turns an under-filled class into a VOID. On a hit
the check reads the memo, not the key: the hit's `position` is byte-identical to
its miss's, so a memo that returned another position's entry fails at the first
sampled hit.

---

## 5. THE TESTS

`crates/pistol-arena/tests/label_cache_tests.rs`, driving the shipped `arena` and
stub unless a row says otherwise.

| # | what it pins |
|---|---|
| T1 | a cached capture is **byte-identical** to an uncached one, over the fixture below |
| T2 | the counts line reads `asks < records` cached and `asks == records` uncached. Two games must share a prefix: the arena plays every opening from both colours, and with one stub on both seats the two games are identical, so every prefix of the second is a hit |
| T3 | `--label-cache` with `--census`, **in both orders**, is refused naming both words and leaves no output file |
| T3b | `capture::run` given a census-on sink and `LabelCache::On` returns the refusal naming both, before any engine is spawned — a library-level test over a report the arena wrote |
| T4 | a stray line at a prefix the cached run treats as a HIT is refused **at that prefix in both runs**: row 6's stub with `n` = the asked-prefix count of game 0, read off an honest play of the same opening, so the stray follows game 0's last answer and game 1's first prefix, a hit, finds it |
| T5 | over a one-opening report both counters are zero — every miss has a distinct stone count; over the fixture below `key_pos_collisions >= 1`, `key_full_collisions >= 2`, `key_full >= key_pos` |
| T6 | T1 over a report holding a forfeit and a rule-4 win |
| T7 | `--label-cache` twice, or anywhere but last, is refused |
| T8 | in `cold_label_check_tests.rs`: a capture cut to nine HIT records is a VOID at `--partition hits`; cut to ten, it passes |

**The fixture**, built by the test and checked by it with `pistol-core` before
use: three five-turn openings X, Y, Z — Y's four-turn prefix a transposition of
X's (equal sorted stone lists, different lines), Z's a lattice image of X's (equal
`canonical_form`, different lists), the three whole openings of distinct
canonical form, as the book loader requires. It is the only fixture on which a
wrong-key mutant is observable. **T2 is the only row a dead cache fails**: a cache
parsed and dropped yields the uncached bytes.

---

## 6. THE MUTATION SET

Against call sites enumerated by a `git grep` receipt in the mutation document
(D-568); the sites do not exist yet, so **the receipt is owed at IMPL**.

| site | mutant | dies at |
|---|---|---|
| 2c, lookup | removed — every prefix asks | T2, cached arm |
| 2c, lookup | inverted — a miss taken as a hit | T1, the first record |
| 2c, lookup | keyed on the stone list | T5: Y's prefix becomes a hit, `key_pos_collisions` reads 0 |
| 2c, lookup | keyed on the canonical form | T5: Z's prefix becomes a hit, `key_full_collisions` reads 1; and T1, a `bestmove` in the wrong frame |
| 2d, insert | removed | T2 |
| 2d, insert | stores the raw pair | T1: ` nps` in a hit's record |
| 2, mode | consulted nowhere — `On` behaves as `Off` | T2 |
| `asks` | reports `records` | T2, **cached** arm |
| `asks` | derived from the records or the memo | T2, cached arm |
| X1 | the arm removed | T3: the combination falls to the catch-all, which also refuses and exits 2 but names neither word |
| X1 | one alternative of the or-pattern removed | T3, that order |
| X1b | removed | T3b |
| X3 | the hoisted guard removed | T4: the cached run exits 0 where the uncached refuses |
| counters | `key_pos_collisions` removed | T5, the collision fixture |
| counters | `key_full_collisions` removed | T5, the collision fixture |
| counters | inverted — non-collisions counted | T5, the one-opening fixture |
| row 6 | the stray never written | T4 |
| row 7 | the floor removed, or set to zero | T8, the nine-record case |

---

## 7. THE ONE REJECTED FINDING

Round 1: the design never mentions `no_tab`, which the registration names as a
candidate cache defect. `no_tab` runs in `run` on the constructed record (`:359`),
after `ask` and outside it; a hit skips `ask`, not record construction, so it
cannot skip the guard. This package does not touch it and adds no mutant for it.

---

## 8. OBLIGATIONS

| obligation | discharged by |
|---|---|
| REVIEW-design, fresh context, against this revision whole | round 4 of five (D-585) |
| IMPL; REVIEW-impl, fresh context, not the implementer; RED-TEAM on the cache path | subagents. The red team's inputs: a forfeit, a rule-4 win, the T5 fixture, a stray line, `--census`, a doubled word |
| the mutation receipt with its `git grep` enumeration | at IMPL, its own document |
| the `tools/` coverage rule | T8 drives the shipped checker; §5 the shipped `arena` |
| `wp21_throughput_prereg.md` §7.1 limb 4 cannot fail (`label_cache_count.py` reads a file identical either way) | its next revision reads the printed `asks`, and corrects §4.2's raw-pair wording and §2's three counter claims |
| an ADR line carrying the strongest surviving attack | at closure |

---

## 9. WHAT THIS DESIGN DOES NOT DO

1. **Persist across processes** — the memo dies with `run`, structurally.
2. **Fold transpositions or symmetries** — each a class of wrong answer, worth 42
   to 60 searches a tranche at `k = 2` (§2.4).
3. **Change any criterion of `wp21_prereg.md` §4** — it supplies T-A's floor.
4. **Settle D-562(2)** (§2.4).
5. **Make `search_nodes` a measure of work**: under the cache ~53% of records
   carry a `search_nodes` for a search not run (registration §4.2); summing that
   column over-counts by the duplication factor.
