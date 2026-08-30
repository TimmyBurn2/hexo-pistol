# WP-1.9 — eval window-map storage: SCOPE MEMO (the premise gate)

**Verdict: PREMISE STOP before design. No design was written, no code was
touched.** The registered scope is stale in one place, self-contradictory in
another, incomplete in the dispatch's own enumeration, and carries one licence
clause that is FALSE at this revision. Four rulings are owed and each is one ADR
line. This is the WP-1.5c disposition the dispatch itself names: a premise
failure here is a success of the process.

Revision this memo is taken at: `722ab8b` (`docs/decisions.md` at the revision
carrying D-494/D-495, appended by this session per §0.1).

This memo contains **no measured numbers of its own** (D-483). Every figure below
is either a quotation from the record or a count of call sites in the tree,
reproduced with `/usr/bin/grep` per the Environment rule (D-265).

---

## 1. The sources the dispatch names, quoted at `file:line`

The dispatch's premise rule names three. There is a fourth — **D-258**
(`docs/decisions.md:557`), which amends D-249 for this package specifically and
registers six obligations on its measurement. It is quoted under F4 below,
because its absence from the dispatch's enumeration is itself a finding.

### 1.1 D-225 — `docs/decisions.md:492`

> WP-1.5a IS LICENSED ON THE RECORD RATHER THAN ON A PROFILE THRESHOLD, AND ITS
> OPTION MATRIX GAINS AN OPTION AND LOSES A CONSTRAINT.

The two clauses that bind this package:

> THE OPTION THE EARLIER LIST OMITTED: packing `Window` (`{ axis: Axis, start:
> Coord { q: i16, r: i16 } }`) into an order-preserving `u64` key, `axis << 32 |
> (q + 32768) << 16 | (r + 32768)`, which is EXACTLY the derived `(axis, q, r)`
> lexicographic order so `BTreeMap` iteration order and `PartialEq` are
> untouched […]

> THE CONSTRAINT THAT DOES NOT APPLY: `self.windows` is used in exactly three
> operations — `entry()` in `apply`, `entry()` in `undo`, `get()` in `delta` —
> and is **NEVER ITERATED**, so its ordering is unobservable and an unordered or
> hashed store cannot influence move choice […] Flips if `self.windows` ever
> gains an iterating caller, which would put the determinism law back in front
> of every unordered option.

### 1.2 D-249 — `docs/decisions.md:540`

> AMENDS D-225 — WP-1.5a IS THE `pistol-solver` THREAT GENERATOR, AND THE EVAL
> WINDOW-MAP STORAGE CHANGE D-225 CALLED BY THAT NAME IS RENUMBERED **WP-1.9**,
> LICENSED AND NOT SCHEDULED.

> LOGGED, NOT DISPATCHED: the threat core stays on the critical path and WP-1.9
> opens when the operator schedules it.

### 1.3 The ROADMAP entry — `docs/ROADMAP.md:290-298`

> **WP-1.9 — eval window-map storage** (docs/decisions.md D-225, renumbered by
> D-249). Replace `pistol-eval`'s `BTreeMap<Window, Counts>` with the storage
> shape WP-1.5a's matrix selects. LICENSED, NOT SCHEDULED: it owes its own
> option matrix, its own pre-registration and its own `tools/bench_delta.sh`
> run, and it does not sit on the threat core's critical path. PRIORITY: after
> WP-1.5b and after WP-1.10 (D-289), because its recorded stake is a table-only
> reading that D-258 forbids quoting as a whole-engine bracket. Unlike WP-1.5a
> its bracket IS a whole-engine one, because `pistol-eval` is linked by the
> shipped binary and `pistol-solver` is not.

---

## 2. The code sites those three bind, at this revision

| What the record names | Where it actually is |
|---|---|
| `BTreeMap<Window, Counts>`, the incumbent store | `crates/pistol-eval/src/handcrafted.rs:86` |
| the whole carried state | `crates/pistol-eval/src/handcrafted.rs:81-95` |
| `entry()` in `apply` | `crates/pistol-eval/src/handcrafted.rs:130` |
| `entry()` in `undo` | `crates/pistol-eval/src/handcrafted.rs:151` |
| `get()` in `delta` | `crates/pistol-eval/src/handcrafted.rs:226` |
| `Window`, the key, and its derived `Ord` | `crates/pistol-core/src/window.rs` (`#[derive(… PartialOrd, Ord, Hash)]`) |
| `windows_through`, the per-stone enumeration | `crates/pistol-core/src/window.rs` |
| the `Eval` trait and the `delta` contract | `crates/pistol-eval/src/eval.rs` |
| the rebuild-from-scratch oracle | `crates/pistol-eval/tests/common/reference.rs:11` (`value_from_scratch`) |

`self.windows` is a private field; `/usr/bin/grep -rn "windows" crates/pistol-eval/src/`
returns its declaration and exactly the three operations D-225 names. **No fourth
call site exists.** That half of D-225 reproduces.

---

## 3. The findings

### F1 — BLOCKING (scheduling). D-494 amends D-471, but D-471 is not what put this package behind WP-1.10.

`D-493` (`docs/decisions.md:1054`) is the newest line in the log and was written
one commit before this session. It rules:

> ROADMAP's WP-1.9 (eval window-map storage) remains licensed-not-scheduled
> **behind WP-1.10**. […] Flips if the architect rules that D-471's "WP-1.9"
> meant the eval storage package, **in which case D-471's ordering sentence is
> the thing that needs amending, not the ROADMAP**.

D-494 (this session's §0.1 append, verbatim from the dispatch) amends exactly
that: D-471's ordering sentence. **But D-471 was never the source of the
WP-1.10 precedence.** That precedence is registered independently, in two
places, with two different stated reasons:

- `docs/ROADMAP.md:294-296` — WP-1.9 "PRIORITY: after WP-1.5b and after WP-1.10
  (D-289), because its recorded stake is a table-only reading that D-258 forbids
  quoting as a whole-engine bracket."
- `docs/ROADMAP.md:320-323` — WP-1.10 "PRIORITY: after WP-1.5b, and BEFORE
  WP-1.9 — these scripts are the instrument every Stage-1 strength claim is read
  through, and the defect that opened the package was a gate reporting a pass
  for a binary that was never built."

So D-494 fires D-493's flip only halfway: it schedules the package without
displacing the ordering that D-493, ROADMAP:294-296, ROADMAP:320-323 and D-289
all carry. Proceeding would be silent architecture drift under rule 10 — the
exact defect D-249 and D-473 exist to have fixed once already.

**Scoped honestly, because I checked rather than assumed:** WP-1.10's debt is
NOT a technical blocker for this package. Its coverage item (D-289,
`docs/decisions.md:623`) names five UNDRIVEN gate scripts — `config_check.sh`,
`determinism.sh`, `movetime_check.sh`, `perft_check.sh`, `search_oracle_check.sh`
— "undriven" meaning no test suite executes the script, not that the gate does
not run. All five run in `tools/ci.sh` and their output is citable. And Track E's
three legs do not depend on the two that look relevant:

- gate-off byte-identity does **not** route through `search_oracle_check.sh`.
  The WP-1.5d(A) precedent (D-484, `docs/decisions.md:1036`) is a direct
  two-binary comparison — both built `--release --locked` in their own detached
  worktrees, over all 115 positions of `tactical_staged_v0.txt` at both
  determinism budgets, 805 output lines and 115 bestmoves compared.
- the bench bracket routes through `tools/bench_delta.sh`, which is on D-289's
  **DRIVEN** list.

So F1 is an ordering/ADR conflict, not an instrument failure, and I am not
dressing it up as one. It still needs the architect, because a session cannot
overrule a registered precedence reaffirmed by the log's own newest line.

### F2 — BLOCKING (scope staleness). The ROADMAP entry names a selection that does not exist, and contradicts itself in the same sentence.

`docs/ROADMAP.md:291-292` says to replace the incumbent "with the storage shape
**WP-1.5a's matrix selects**". D-249 made WP-1.5a the `pistol-solver` threat
generator and moved the eval storage change out of it. **WP-1.5a selects no eval
storage shape, and no eval-storage option matrix exists in this tree** — no file
under `docs/experiments/matrix_*.md` covers `Window`/`Counts` storage, and the
`DESIGN.md` whose §5.4.3/§5.4.5 the WP-1.5a prereg cites
(`docs/experiments/wp15a_prereg.md:671`) is not in the tree at any path (`find`
returns nothing). It lived in a removed worktree — a D-469 export failure of the
same class as WP-1.8c's four review reports.

Two sentences later the same entry says the package "owes its own option
matrix". The entry therefore both inherits a selection and owes one. The
resolution is almost certainly "owes its own" — but it is not mine to make,
because it decides whether this package must author and red-team a matrix before
any design is written.

**And the dispatch's development round has no matrix step in it.** It runs scope
memo → design → REVIEW-design → impl → REVIEW-impl → track → closure. CLAUDE.md
is unambiguous that this is not optional:

> An option adopted without a matrix, or a matrix never attacked, is the same
> breach as silent architecture drift.

The storage shape is a named decision with more than one viable option — D-225
names at least three (packed-`u64` key over the incumbent `BTreeMap`, open
addressing, and the incumbent) — so the matrix law binds and the round as
dispatched cannot satisfy it.

### F3 — BLOCKING (a licence clause that is false at HEAD). `self.windows` IS reached by an iterating operation, and D-225's own flip condition has arguably already fired.

D-225 licenses the whole unordered-store option field on the claim that the map
is "**NEVER ITERATED**". At this revision that is false, by one site D-225 did
not count:

`crates/pistol-eval/src/handcrafted.rs:81` — `#[derive(Debug, Clone, PartialEq, Eq)]`
on `HandcraftedV0`, over the whole carried state including `windows`.
`BTreeMap`'s `PartialEq` iterates both maps.

It is not decorative. Two committed tests are exactly this equality, and they are
the two the dispatch's own test list asks for:

- `crates/pistol-eval/tests/eval_incremental_tests.rs:140` —
  `assert_eq!(eval, fresh, "take-back order must not matter")`
- `crates/pistol-eval/tests/eval_delta_tests.rs:407` —
  `assert_eq!(eval, before, "a probe left a trace in the carried state")`

and the trait makes it a contract, at `crates/pistol-eval/src/eval.rs`:

> "Indistinguishable" means OBSERVATIONAL equivalence through
> `Eval::apply`/`Eval::undo`/`Eval::value`; **a backend whose whole state is
> comparable pins it as equality** (D-214).

**What survives and what does not.** D-225's *determinism* conclusion survives
intact: equality is a test-only path, nothing on it reaches move choice, so
rule 4 is not in danger and D-32 still does not reach this map. What does NOT
survive is the use of that clause as a blanket licence for "an unordered or
hashed store": any replacement owes **canonical, order-independent equality**.

**The sharpest form of it is tombstones, and the incumbent test is already
aimed at exactly this.** `eval_incremental_tests.rs:118-140` unwinds a position
twice — once in reverse (`:119`) and once in a rotated order no search would use
(`:137`, `stones.iter().skip(3).chain(stones.iter().take(3))`) — and both times
asserts equality against a **fresh** eval, with the first assertion spelling out
its target: *"an unwound eval must be indistinguishable from a fresh one,
emptied windows included"* (`:129-130`). That is pinning `undo`'s removal of an
emptied entry at `handcrafted.rs:168-172` (*"An emptied window leaves no entry
behind"*). An open-addressing store retains **tombstones** where the incumbent
`BTreeMap` retains nothing, so a `#[derive(PartialEq)]` over its raw slot array
compares a fully-unwound table UNEQUAL to a fresh one — and `assert_ne!(full,
fresh)` at `:116` leans on the same comparison from the other side. The failure
is loud, which is the good case; the hazard is the "repair" that replaces whole-
state equality with a `p1_score` comparison, at which point both tests and the
`Eval` trait's equality clause go vacuous together and the incremental contract
is pinned by nothing.

That is a live design constraint with a named failure mode, and it belongs in
the matrix's failure-mode column before an option is selected, not discovered
during impl.

Whether this trips D-225's stated flip ("Flips if `self.windows` ever gains an
iterating caller, which would put the determinism law back in front of every
unordered option") is the architect's call. My reading: the flip's *reason* —
the determinism law — is not engaged, so the flip should be recorded as
NARROWED rather than fired, and the narrowing is the canonical-equality
obligation above.

### F4 — BLOCKING (the dispatch's enumeration of registered scope is incomplete). D-258 is a fourth binding source and it is not in the dispatch.

The dispatch's premise rule says the registered scope "lives in D-225 as amended
by D-249 and the ROADMAP entry". It does not. **D-258** (`docs/decisions.md:557`)
amends D-249 specifically for this package:

> AMENDS D-249's RECORDED STAKE FOR WP-1.9 — THE `k = 4.4-4.9` FIGURE IT CARRIES
> IS A TABLE-ONLY READING AND IS NOT USABLE AS A BRACKET, so the whole-engine
> 1.36x-1.62x it predicts must not be quoted as one.

and then registers six obligations under "WHAT WP-1.9 MUST DO, registered here so
it is not rediscovered". A session that read only the dispatch's three sources
would not have seen them.

**And one of the six has no referent in `pistol-eval`.** D-258 requires WP-1.9 to
"carry all ten maintained sets and not six". There are no ten maintained sets in
this crate. `HandcraftedV0` holds `weights`, one `BTreeMap<Window, Counts>`
(`handcrafted.rs:86`) and one `p1_score`. The ten are `pistol-solver`'s, and they
are countable: `crates/pistol-solver/src/sets.rs:7` `CLASS_COUNT = 5`,
`sets.rs:141` `sets: [Vec<Window>; CLASS_COUNT]`, `state.rs:30`
`sets: [WindowSets; 2]` — five classes on two sides. D-258's obligation list was
transplanted from `docs/experiments/wp15a_prereg.md:676-687`, which is the
**solver's** storage measurement, and one item came across without a referent.

The other five transplant cleanly and are good law for this package (measure at
the shipped structure and not an isolated table; colour plies through
`GameState`, never `i % 2`; hoist store construction out of the timed region;
keep the comparand in the same run; check the instrument against something
outside it). They should be carried. The tenth-sets item should be struck by ADR
rather than satisfied, because it cannot be satisfied here.

---

## 4. Recorded, not blocking

**R1 — the dispatch's Scope paragraph describes work that is already landed.**
It asks the design to decide "incremental update and UNDO paths" and a
"rebuild-from-scratch path as the internal oracle" as though they were new. All
three exist and are accepted: `apply`/`undo` are the landed incremental contract
(`handcrafted.rs:125-177`), `delta` is D-220/D-214-accepted at a PASS inside its
registered bracket, and the rebuild oracle is
`crates/pistol-eval/tests/common/reference.rs:11`. The dispatch's own premise
rule resolves this — "This dispatch asserts NO mechanism", registered scope
governs — so registered scope wins and the package is a **container swap**
behind an unchanged contract. Recorded so the design does not re-derive
machinery that is already there and already tested.

**R2 — the two-track rule is already decided by registered scope.** A container
swap behind an unchanged contract is bit-identical by construction, so the claim
is Track E and D-495's harness verification is what makes it bind. Worth noting
that `tools/bench_delta.sh:389-399` already asserts per-position node identity
between the two binaries under both budgets in every rep, and fails by name if it
does not hold — so the registered bench instrument is *itself* a partial
equivalence harness, and the bracket run and the identity proof are not
independent evidence. The design should say so rather than let the agreement read
as corroboration it is not (the D-220 lesson, stated in D-220's own text about
time-to-depth).

**R3 — the carried-forward bench defect does NOT bite this package. Checked, not
assumed.** The stopped stage-Q session left a finding that the shared rule-5
bench command block silently substitutes the empty board for a refused entry. It
is fixed (D-475, `docs/decisions.md:1018`), and independently
`tools/bench_delta.sh` never had it: it refuses error lines by name at
`tools/bench_delta.sh:363-364`, asserts one totals line per position at
`:374-375`, and asserts node identity at `:389-399`.

**R4 — filename collision.** `docs/experiments/wp19_design.md` is already the
stage-Q design, which D-473 re-designated WP-1.5c. This package's documents need
distinct names; this memo uses `wp19_storage_*`.

---

## 5. What the architect is asked to rule — four lines, and the package runs

1. **Ordering.** Does the eval-storage WP-1.9 run now, ahead of WP-1.10, or does
   WP-1.10 run first? If now: the ruling must displace `ROADMAP:294-296` and
   `ROADMAP:320-323` by name, not only D-471, and say where WP-1.10 goes. Note
   that the dispatch's Stage-1 arc-closure list (1.5b, 1.5c, 1.5d, 1.6, 1.7,
   1.8, 1.9) would record Stage 1 complete with **three** Stage-1 packages still
   open, and names only one of them. Stage 1 is `docs/ROADMAP.md:30-327`; the
   open packages inside it are **WP-1.4** (`ROADMAP.md:70-73`, the D-95
   movetime-ceiling fix — no CLOSED marker, and D-478 restates it as "LICENSED,
   NOT SCHEDULED"; on HeXO "the server owns the clock and hard-clamps the call,
   so this is a forfeit risk rather than a known limitation"), **WP-1.9** (this
   package) and **WP-1.10** (`ROADMAP.md:300-323`). WP-1.10 additionally holds a
   registered precedence over this one, so routing it to "final cleanup" inverts
   a precondition rather than deferring a debt.
2. **Matrix.** Confirm the package owes its own option matrix and a fresh-context
   DECISION-RED-TEAM before any option is selected, and strike
   `ROADMAP:291-292`'s "the storage shape WP-1.5a's matrix selects" as stale.
   The development round then gains a matrix step it does not currently have.
3. **D-225's flip.** Fired or narrowed? Recommended: narrowed — the determinism
   law is not engaged, and the operative obligation is that any replacement store
   carries canonical, order-independent equality, with
   `eval_incremental_tests.rs:140` and `eval_delta_tests.rs:407` as the tests
   that must keep meaning what they say.
4. **D-258.** Confirm it as a binding source for this package, carry its five
   applicable obligations, and strike "carry all ten maintained sets and not six"
   as having no referent in `pistol-eval`.

Nothing in the eventual design depends on the answers to 3 and 4 in a way that
would be wasted work; 1 and 2 gate whether the design may be written at all.

## 6. State left behind

No design written. No code touched — `git diff --stat` over `crates/ tools/
configs/ Cargo.*` is empty for this session. Landed: D-494 and D-495 (§0.1,
verbatim), this memo, and the STOP summary at
`sessions/WP-1.9/2026-08-30-WP19-STORAGE-PREMISE-STOP.md`.
