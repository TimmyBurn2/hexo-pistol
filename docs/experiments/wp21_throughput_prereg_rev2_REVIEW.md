# REVIEW-design (fresh context) — `wp21_throughput_prereg.md` revision 2

**NAMED REVISION.** `git stash create` object
`fce50bc5b00baab9066f1e7bdf10c48c025b7755`, branch `overnight2-stopped`,
HEAD `2b94f043db2c7f8fe64c9abc6d5b9414ff93a11c`.

**DOES IT STILL MATCH THE WORKING TREE?** YES.
`git diff fce50bc5b00baab9066f1e7bdf10c48c025b7755 -- docs/experiments/wp21_throughput_prereg.md`
is **empty**. The working-tree file was read and is the file judged below.

**REVIEWER CONSTRAINT HONOURED.** No `cargo` invocation of any kind. Everything
below is `/usr/bin/grep`, `git grep`, `git show`, `git log`, `python3` over text,
`sha256sum`, `lscpu`, and reading files.

**WHAT I READ.** `CLAUDE.md`; `docs/process.md`;
`docs/experiments/wp21_prereg.md` revision 3 (working tree);
`docs/experiments/arc3_ledger.md`; `docs/decisions.md` (D-539, D-540, D-556
through the tail, especially D-558, D-560, D-561, D-562, D-570, D-572, D-573,
D-574); `docs/book_v2_ledger.md`; `artifacts/arc3_leverB_41_count.txt`;
`/home/tom/pistol-runs/wp20pilot-artifacts/{corpus_v1.txt,capture_v1.txt,report_v1.txt}`
and `artifacts/wp20pilot_RUN_2cd4f79_v1.txt`;
`crates/pistol-arena/src/{capture.rs,capture_file.rs,exchange.rs,labels.rs,labels_file.rs,bin/arena.rs}`;
`crates/pistol-search/src/{heuristics.rs,ordering.rs,search.rs,position.rs,tt/mod.rs}`;
`crates/pistol-engine/src/instance.rs`; `crates/pistol-core/src/{board.rs,turn.rs}`;
`configs/instrument_v0.toml`; `tools/determinism.sh`; `tools/ci.sh`; `.gitignore`.

## VERDICT: **FAIL**

Two BLOCKING findings and twelve MAJOR. **The most important attack — attack 1,
whether the cache's key can return a wrong answer — FAILED: I could not find a
single channel, and I looked hard. The cache's key is sound.** The document
fails on the things around it: an ADR it cites that does not exist, a registered
command the shipped binary refuses, a gate citation that is wrong and a gate
coverage claim that is overstated, a receipt that does not contain the rows
attributed to it, an internal contradiction its own measurement settles against
it, three registered rules with no mechanism, and a cost case that no longer
closes after its own amendment.

---

# BLOCKING

## B1 — **D-576 DOES NOT EXIST. The document asserts it four times and QUOTES it once.**

Quotes, all from the reviewed file:

- preamble: *"The architect ruled on lever B before this document was reviewed
  (arc III §R, R1; landed as **D-576**)"*
- §2.1: *"A fold that has never merged a pair … is not a trade this study takes.
  **Landed as D-576.**"*
- §4.5: *"**A failure of either still abandons the lever**, per §4.4's own
  consequence, and **D-576 says so in its own words**: the cache may not be
  repaired into passing."*

`/usr/bin/grep -c "^D-576" docs/decisions.md` returns **0**. The ADR log's last
line is **D-574** (`docs/decisions.md:1216`); there is no D-575 either. The
arc's own ledger agrees the block has not landed —
`docs/experiments/arc3_ledger.md` §0: `| §R rulings block | not started | — |`.
`docs/decisions.md` is **not** among the working tree's modified files
(`git status --short`), so it is not pending in the tree either.

Why it is wrong: hard rule 10 makes the ADR line the record of the decision, and
this document reports that record as already made. Worse, §4.5 attributes
*words* to it — *"D-576 says so in its own words"* — for a line that has never
been written by anyone. A registration cannot stand on a citation to a document
that does not exist, and a successor reading §4.5 will go looking for a
constraint that is nowhere. `wp21_prereg.md` revision 3 has taken the same
citation into its §1 table and its preamble, so the error is already propagating.

Fix: land the ruling as a real D-line at the next free number (D-575) before
either registration is reviewed as governing, and replace every "landed as
D-576" with that number — or, if the ruling has not been written, say
*"to be landed as D-n"* and delete the fabricated quotation in §4.5 (the
constraint it sources is already stated by §4.4 in its own words, so nothing is
lost).

## B2 — **§3.2's registered command is REFUSED by the shipped binary, and no dry run was recorded.**

§3.2 registers, as the literal command every setting runs:

> `arena --capture <the report> --label-nodes 400000 --out <distinct path>`

`crates/pistol-arena/src/bin/arena.rs:39-85` matches argument words
**positionally**, and the only non-census capture arm is
`arena.rs:51`:

```
["--capture", source, "--out", out, "--label-nodes", nodes] => (
```

There is no permutation arm. The catch-all at `arena.rs:79-85` returns
`"--capture, --out and --label-nodes, **each in that order**"` and exits
nonzero. The registered command has `--label-nodes` before `--out` and is
therefore refused **before any game**.

This is not a typo finding, it is a methodology finding. `docs/process.md`,
**Dry-run discipline**, is binding *"exactly as CLAUDE.md text would"*:

> A pre-registration's literal commands are exercised before its review passes,
> on an input of the SAME KIND as the registered workload … **The
> pre-registration records the dry-run input and its output.**

The document records **no dry run at all** — not for lever A's command, not for
§4.1's count, not for §4.4's `cmp -s` pair. The proof that the rule was not
followed is that the one command the document prints does not run. Fifteen
processes would have failed at once, at the top of a study whose whole output is
a wall-clock number.

Fix: correct the flag order, run the dry run the rule requires on a same-kind
input (a small report that is not the registered `0..2` one), and record its
input and output in the document.

---

# MAJOR

## M1 — **`tools/determinism.sh` is gate 9, not gate 6 — and the gate does not cover the sweep's seat or its budget, so the soundness claim is weaker than stated.**

§2: *"the same binary, the same config, the same `position` and the same `go`,
after a `newgame`, return the same bytes — hard rule 4, and
`tools/determinism.sh` is CI gate 6."*

`tools/ci.sh:104-105`:

```
step "gate 9/19: cross-process determinism"
gate "determinism" tools/determinism.sh
```

Gate 6 is `tools/config_check.sh` (`tools/ci.sh:92-93`). The arc's own ledger
(F-1.5) has already recorded this and states the correction is *"held as a
post-review amendment"*. **I report it anyway because the file under review
still says gate 6 and because the ledger's proposed replacement does not close
the second half of the finding**, which is new:

The reduction §2 claims is *"the cache's soundness reduces to a law this project
already gates"*. Gate 9 does not gate that law **at the sweep's seat**:

- `tools/determinism.sh`'s `SEATS` array runs `configs/gate_v0.toml`,
  `gate_staged_v0.toml`, `gate_staged_heuristics_v0.toml`,
  `gate_staged_solver_v0.toml`, `gate_staged_snk_v0.toml`. It never runs
  `configs/instrument_v0.toml`, which `wp21_prereg.md` §1 registers as the
  sweep's config for both seats.
- Its budgets are `depth_turns 4` and `nodes 200000` (`BUDGETS`), never the
  sweep's `nodes 400000`.
- The limb that actually tests the cache's premise — C vs D, *"one process per
  position"* against *"all of them in one session"* with `newgame` before each —
  runs at `LAYOUT_BUDGET="nodes 200000"` only, on the gate fixtures.

So the honest statement is: gate 9's C-vs-D limb gates the *shape* of the
property on four seats at two budgets, none of them the sweep's. That is good
evidence and it should be cited; it is not the "already gated" that §2 claims.
The empirical safety net for the sweep's own seat is §4.4, and §2 should say so
rather than claiming a gate it does not have.

Fix: cite gate 9 by number, quote its C-vs-D limb, and state plainly that the
gate runs at four gate seats and not at `instrument_v0.toml` / `nodes 400000` —
which is exactly why §4.4's byte-identity comparison, not the gate, is the
criterion.

## M2 — **§1 and §2 CONTRADICT each other about cross-game transpositions, and the measurement says §1 is wrong.**

§1: *"the pilot's 742/347 = **2.1383** MEASURED **adds the cross-game
transpositions on top**."*

§2: *"there is **not one cross-game transposition** in the corpus to fold, and
the whole 2.1383 duplication is the two-seat replay of identical prefixes."*

Both cannot be true. I re-derived the multiplicity structure from
`corpus_v1.txt` myself:

```
multiplicity distribution of the position line over 742 records:
  345 position lines occur exactly  2 times
    2 position lines occur exactly 26 times
  345*2 + 2*26 = 742;  345 + 2 = 347
```

§2 is right and §1 is wrong. The excess over the structural 2.0 is **not**
transpositions: it is the two shortest prefixes — `position start` and
`position start moves 0,0` — which are **byte-identical across all thirteen
openings** (game rule 3 puts turn 1 at the origin WLOG), and so are asked 26
times each rather than twice. That is cross-*game* sharing of an identical
spelling, which is a different mechanism with a different scaling law from a
transposition, and §1's sentence would send a successor looking for a fold that
is not there.

§1 was not touched by revision 2 (`git diff HEAD` shows §1 unchanged); revision
2 added §2's measured refutation next to it and left the refuted sentence
standing. This is D-423's *"a claim the document makes twice is a defect
waiting"* materialising: the ratio is asserted in §1, §2, §2.1, §4.1 and §4.5,
and the one copy nobody re-derived is the wrong one.

Fix: delete §1's *"adds the cross-game transpositions on top"* and point §1 at
§4.1, which owns the number.

## M3 — **The receipt does not contain the rows §2 and §4.1 attribute to it, and its own key line names revision 1's key.**

`artifacts/arc3_leverB_41_count.txt`, in full, reports four counted rows:

```
key: the replayed position's sorted (cell, player) list, wp21_throughput_prereg.md §2
asked prefixes          742
distinct sorted-stone   347
distinct key_pos column 347
distinct key_full column 347
```

§4.1 prints a code block introduced by *"Receipt
`artifacts/arc3_leverB_41_count.txt`:"* whose first count row is
**`distinct raw move lists (§2's key)    347`** — a row the receipt does not
carry. §2 cites the same receipt for *"distinct **raw move lists**, distinct
**`key_seq`**, distinct `key_pos` and distinct `key_full`"* — two rows the
receipt does not carry. And §2 and §4.1 do not even name the same four columns
as each other (§2: move lists / `key_seq` / `key_pos` / `key_full`; §4.1: move
lists / sorted stones / `key_pos` / `key_full`), while both say "all four".

Separately, the receipt's own `key:` line names **revision 1's** key — the
sorted `(cell, player)` list — and points at *"§2"*, which revision 2 has
replaced. A reader following the citation lands on a receipt that measured the
key this document has abandoned.

The numbers are not wrong — I re-derived all five and every one is 347 (see
WHAT SURVIVED, S3) — but CLAUDE.md's closure rule is that a claim *"cites the
gate's own log output, never a wrapper's"*, and a receipt quoted for rows it
does not hold is the same defect one step further along.

Fix: the post-review re-take (§4.1) must emit all the rows the document quotes,
under revision 2's key, with a `key:` line naming that key — and §2 must quote
the same four columns §4.1 does, or better, stop quoting them and point at §4.1.

## M4 — **The registered key is one of the four arguments §2's own soundness statement names, and the invariant that makes that safe is not registered.**

§2 opens: *"A LABEL IS A PURE FUNCTION OF **(position, budget, binary, config)**"*.
§4.2 then registers the key as *"the `position` line's exact bytes"* — one
argument of four. That is safe only under an invariant the document never
states: the memo must not outlive one `capture::run` invocation, in which `go`
is loop-invariant (`capture.rs:333` computes `label_go_line` once outside the
game loop) and the binary and config are fixed by the process.

§4.2 says *"A memo in `capture::run`"*, which gestures at the scope but does not
register it as a constraint the implementation must satisfy. An implementer who
lifts the memo to a `Capture` struct reused across two `--label-nodes` values,
or a future `capture::run` that varies `go`, silently makes the key wrong — and
§4.3 says exactly what that costs: *"nothing downstream would notice"*.

Fix: register the key as the pair (`position` line, `go` line), or register the
lifetime invariant explicitly — *the memo is constructed inside `capture::run`
and dropped with it; it is never a field of a longer-lived value* — as a named
obligation the cache's own review checks.

## M5 — **No cache OFF switch is registered, yet §4.4's referent and §4.2's census refusal both require one.**

§4.4 requires *"tranche one's own report … captured **uncached** and captured
cached"*, and §4.4's first bullet makes the uncached capture the corpus of
record. §4.2 requires `--census` to remain a usable mode for the census run
`wp21_prereg.md`'s preamble defers to Phase 4. Both require the cache to be
switchable. The document registers **no switch**: no flag name, no config key,
and no default.

Hard rule 1 is explicit that a tunable's default *"lives in exactly one schema
place"* and that there is *"NO code-side default for any tunable"*. As written
the package could equally be implemented as an unconditional memo in
`capture::run` — which would make §4.4's referent unobtainable and `--census`
permanently refused — and nothing in the registration would have been violated.

Fix: register the switch by name, register its default, and register that the
default is **off** (which also gives M6 its mechanism).

## M6 — **"No tranche runs cached until the comparison returns byte-identity" has no mechanism and no named checker.**

§4.4, second bullet, is a rule about run ORDER across sixteen tranches launched
in waves by an operator. The document says what must be true and never says who
establishes it or how a violation would be caught. There is no gate, no
refusal, no flag default, and no run-log obligation — the wave schedule is
explicitly *"derived in the run log"* rather than registered. `wp21_prereg.md`
§6.1 repeats the rule and likewise names no checker.

The failure it guards is unrecoverable in the direction that matters: a cached
tranche taken before the comparison returns produces a corpus whose correctness
was assumed, and §4.3 says nothing downstream would notice.

Fix: give it a mechanism. The cheapest is M5's default — the cache flag is off
unless passed — plus a registered run-log obligation: the `cmp -s` exit line and
its timestamp appear in the run log **before** the first tranche block whose
command carries the cache flag, and the closure states who read it.

## M7 — **C3 cannot be evaluated from any output the study produces.**

§3.4 and §3.6 register: *"**Any N whose realised seconds-per-label exceeds
`hang_timeout_ms`/1000 = 120 s for a single label is REFUSED whatever its
throughput**"*, and §3.5's table gives C3 as *"no single label's wall exceeds
120 s at any N"*, marked *"can it fail? yes"*.

Nothing measures a single label's wall:

- `capture::normalise` (`capture.rs:66-96`) **strips** ` nps <n> time <n>` from
  the totals line before it is written. I confirmed empirically:
  `/usr/bin/grep -c " time " capture_v1.txt` returns **0** — the pilot's
  742-record capture carries no timing field at all.
- `arena` prints one line per capture run
  (`arena: captured 742 position(s) from 26 game(s) at go nodes 400000`) and no
  per-label timing; `artifacts/wp20pilot_RUN_2cd4f79_v1.txt` shows the whole
  pass reported as `capture1 seconds=657`.

So the only per-label number obtainable is `wall / records`, the **mean**, which
is what §3.3 defines. A setting whose worst label took 110 s would violate C3
undetectably; the only detectable case is the boundary itself, where
`channel.receive`'s watchdog refuses the run and the setting fails on exit
status rather than on C3. A criterion whose registered consequence can never
fire on the evidence the run produces is not a criterion.

Fix: either restate C3 over the quantity the instrument yields (*"the realised
mean seconds-per-label at any N stays below X, where X leaves margin M against
the 120 s watchdog"*, with X and M registered), or register an instrument that
produces per-label wall — e.g. run the setting's capture with the engine's
`time` field preserved in a scratch, non-corpus transcript.

## M8 — **§4.4's registered consequence pre-attributes a disagreement to the ENGINE and forecloses the likelier cause.**

§4.4: *"the disagreement is recorded as a finding about `newgame`'s isolation —
because under §2's argument a disagreement means state crossed an ask, which is a
defect in the ENGINE and not in the cache."*

That inference is invalid. §2's argument establishes that the *key* is sound —
that two asks spelled identically must answer identically. It says nothing about
whether the *implementation* stores, looks up and returns the right entry. A
byte difference between the uncached and the cached pass is at least as likely to
be a defect in a brand-new memo (wrong entry returned, records emitted out of
order, an entry mutated after insertion, a hit that skips the `no_tab` guard) as
a defect in `newgame`, which four determinism-gate seats already exercise on
every CI run.

Registering the diagnosis in advance is the after-the-numbers move inverted: the
verdict is written before the evidence, and it points away from the new code.

Fix: register the consequence as *"lever B is abandoned; the disagreement is
reported as a finding and the two candidate causes — the cache implementation
and `newgame`'s isolation — are separated by re-running the uncached pass twice
before either is named."* The pilot already shows the uncached pass is
self-identical (`capture-determinism exit=0`, two identical `sha256`s in
`wp20pilot_RUN_2cd4f79_v1.txt:46-47`), so that separation costs one 11-minute
run.

## M9 — **F-1.2 survives the amendment: on THIS sweep lever B's registered verification costs about what lever B saves, and the document replaces the number with prose.**

§5 states the counterfactual precisely — *"lever B's ESTIMATED saving is **3.26
h of the 7.15 h eight-way wall**"*, *"48.95 h to 22.89 h"* — and then states the
applicable figure as prose: *"the ESTIMATED saving falls to **a fraction of a
wave**"*. Every input needed to compute it is in the two documents:

```
uncached, N=8      2 waves x 12 869 s               = 25 738 s = 7.15 h
cached wave 2      12 869 - 11 017 + 5 152 = 7 004 s   (wp21_prereg.md §3)
realised, N=8      12 869 + 7 004          = 19 873 s = 5.52 h
ESTIMATED SAVING ON THIS SWEEP                        = 1.63 h
```

against §5's own registered costs: **~1.43 h** for the cached re-capture of
tranche one (§5, ESTIMATED), plus **~1.1 h** for lever A, plus ~16 min for the
pilot pair. And §4.4's second bullet puts the 1.43 h on the critical path by
construction — the comparison must *return* before wave two runs cached, so the
cached re-capture sits between the waves unless it displaces a wave-one slot,
which degrades wave one instead.

So the amended lever is, on this sweep, roughly a wash on its own and net
negative once the study that justifies it is counted. That is F-1.2's finding
after the remedy, and the document does not re-run F-1.2's comparison against
the amended numbers — it states the favourable number to four significant
figures and the applicable one as "a fraction".

The justification offered instead is *"**THE CACHE'S VALUE IS MOSTLY IN THE
SWEEPS AFTER THIS ONE**"*. There is no registered sweep after this one:
`docs/book_v2_ledger.md:41-43` shows `0..12` consumed by the pilot, `13..3499`
taken by this registration, and `3500..4499` **RESERVED FOR GOVERNED RUNS —
NEVER LABELLED**. The book has no unclaimed range for a later label sweep to
use, so the value is being deferred to a run nothing registers.

Fix: compute the applicable saving (1.63 h ESTIMATED) in §5 beside the 3.26 h,
state the net against §5's own costs, and either name the future sweep and its
range or drop the appeal to it. This is a real decision — the lever may still be
worth taking for the verified capability — but it should be taken against the
number, not against prose.

## M10 — **§4.1's remedy is a replication with the same instrument, which `docs/process.md` says is not the answer for a cheap instrument under doubt; and the instrument has no named revision and no durable receipt.**

The document is candid about the defect: *"This count was taken **before** the
first fresh-context review … which is precisely the defect that stopped the prior
arc"*. The registered remedy is *"the count is RE-TAKEN after this revision
passes review"*.

Three problems.

**(a) It is a replication, not a second instrument.** `docs/process.md`:

> Where the run is cheap, doubt about the instrument is answered by REPLICATION
> and by a SECOND INSTRUMENT whose agreement criterion is registered before
> either runs … AND IT NAMES THE STAGE UNDER DOUBT, and says how the second
> instrument does not share it.

The re-take runs the same script over the same sha-pinned input
(`493f4fa8…`), so it is deterministic and will return the same five numbers by
construction. It can fail only if someone edits the script between now and then.
It cannot change any reading, which is the definition CLAUDE.md gives of prose
that constrains nothing. **As registered, the remedy is a fig leaf.**

**(b) The instrument has no governing revision.** `docs/process.md`, *Instrument
governing revision*: *"An artefact that produces a registered number — a
`tools/` script, a **scratchpad harness**, or a command block the document prints
— is named in the pre-registration WITH ITS REVISION"*, and *"living there is
not what makes the rule apply."* The receipt names `scratchpad count_key.py`;
the document names no instrument at all, no revision, and no sha. The
scratchpad will not survive the session.

**(c) The receipt is not durable and is not anchored.** `artifacts/` is
gitignored (`.gitignore:19`, confirmed by `git check-ignore -v`), so
`artifacts/arc3_leverB_41_count.txt` is uncommitted. Hard rule 8 permits that —
*"a committed manifest may sha-index them"* — but nothing sha-indexes this one,
so §4.1's evidence has no anchored referent.

Fix that discharges all three at once: register a SECOND INSTRUMENT with an
agreement criterion before the re-take — a `tools/` script (which then carries
the coverage rule's test) and one independent derivation — name both with their
sha256, and sha-anchor the re-take's receipt in a committed manifest. **I have
in fact executed the second instrument as part of this review (S3 below) and it
agrees exactly**, which the document may cite; but it should be registered, not
inherited from a reviewer.

## M11 — **The KEY was selected without an OPTION MATRIX and without a DECISION-RED-TEAM.**

CLAUDE.md, Process: *"A named decision with more than one viable option is
settled by an OPTION MATRIX — options, costs, failure modes, recommendation —
attacked by a fresh-context DECISION-RED-TEAM subagent BEFORE selection … An
option adopted without a matrix, or a matrix never attacked, is the same breach
as silent architecture drift."*

The cache key is exactly such a decision, and the document itself enumerates
four viable options: revision 1's sorted `(cell, player)` list, R1/D-570's
canonical identity (§2.1), a 128-bit zobrist (§2), and the `position` line
(selected). §2 and §2.1 give costs and failure modes for each — the prose is
most of a matrix — but it is a matrix in narrative form, never labelled as one,
never routed to a fresh-context DECISION-RED-TEAM before selection, and already
consumed as settled by `wp21_prereg.md` §1's seat table (*"label cache | **ON**,
keyed on the `position` line's exact bytes"*).

This review is a review of a registration, not a decision-red-team run before
selection, and revision 2 explicitly frames it as the first review the document
has ever had — i.e. after selection.

Fix: promote §2/§2.1 into a labelled OPTION MATRIX with each numeric claim marked
MEASURED or ESTIMATED, route it to a DECISION-RED-TEAM, and record the strongest
surviving attack in the ADR line B1 asks for.

## M12 — **§5's "~171 s per process at N=1" is a units error over a count that is itself wrong, and the true value is derivable in milliseconds (D-291).**

§5's cost row: *"lever A, 5 settings x 3 reps, **~171 s per process at N=1** and
rising with N"*.

171 is not a time. It is `3 x 57.0769` — the pilot's records-per-opening
(`wp21_prereg.md` §3) scaled to three openings, i.e. an estimated **record
count**. At the pilot's MEASURED 0.8854 s/label, 171 records is ~152 s, not 171 s.

And the estimate is unnecessary. The exact record count for openings `0..2` is
in the pilot's own committed corpus. Counting games 0-5 of `corpus_v1.txt`:

```
records for openings 0..2 (games 0-5)   152
ESTIMATED wall at 0.8854 s/label        134.6 s
```

`docs/process.md` and CLAUDE.md's D-291: *"an estimate that could have been
measured in seconds is a finding."* This one could have been measured in
milliseconds, from a file the document already cites by sha.

Fix: replace with the MEASURED 152 records and the derived ~135 s, and re-derive
the ~1.1 h aggregate from it.

---

# minor

**m1 — C1 and C2 name the wrong artifact.** §3.5: *"every process's **corpus
file** … is byte-identical"* and *"every process's record count equals the
report's own"*. The registered command is `arena --capture`, which writes a
**capture** (`capture_file.rs`, five TAB fields, `CAPTURE_FORMAT_VERSION`). A
corpus is pass 3's output (`arena --labels --report`), which the study does not
run. Byte-identity is still the right check on the right file; the name is wrong.

**m2 — "No state crosses an ask" is literally false, and the document relies on
the exception elsewhere.** §2's claim is contradicted by
`crates/pistol-engine/src/instance.rs:112-119`, whose own comment says of the
trigger-census collector: *"`new_game`'s `clear` does not touch it"*; the same
holds for `Searcher::census_folds` (`search.rs:83`), which `clear` does not
reset. Neither can move an answer with census OFF, and §4.2's `--census` refusal
is precisely the acknowledgement that census and the cache do not compose — so
the document knows about the exception and states the absolute anyway. Say *"no
state that can move an answer crosses an ask, and the one state that does cross
is the census collector, which §4.2 refuses to combine with the cache."*

**m3 — §4.2's BTreeMap rationale misreads rule 4.** *"IT IS AN ORDERED MAP AND
NOT A HASH MAP, because hard rule 4 forbids an unseeded hash order on a choice
path."* A `HashMap` used only for point lookups never exposes an iteration
order and is not a choice path; rule 4 also explicitly admits *"fixed-seed
hasher"*. The choice is fine and cheap — the reason given is not the reason.

**m4 — the canonicalisation is `Turn`'s, not `position_line`'s.** §2: *"not even
a re-spelling of one turn, because `exchange::position_line` canonicalises every
pair token before it is written."* `exchange::position_line`
(`exchange.rs:154-161`) just calls `turn.to_string()`. The canonicality is
`Turn::Pair`'s type invariant (`turn.rs:90`, *"`first < second`"*) plus
`FromStr`'s refusal (`turn.rs:213-215`, *"An uncanonical pair is refused rather
than reordered"*). The conclusion holds; the attribution does not.

**m5 — §3.3's machine-seconds algebra assumes every process runs the full wall.**
*"(wall from first start to last exit) x N"* multiplies the **maximum** process
wall by N. Where processes finish unevenly this over-states machine-seconds, and
it over-states it most at the settings with the highest variance. It cancels in
the reported statistic (§3.3 correctly reduces to `wall / records`) so no
conclusion moves, but the derivation as written is not exact and should say so.

**m6 — under the cache the corpus's `search_nodes` column no longer sums to
machine work.** 53% of records will carry node counts for searches that were not
run. Hard rule 6 requires per-side compute in strength claims; this sweep makes
none (`wp21_prereg.md` §7.2), and §6 reports records/distinct/coverage rather
than node sums, so nothing breaks today. Register it anyway — a later consumer
summing that column would over-count compute by the duplication factor.

**m7 — the study's own play pass has no registered instrument.** §3.1 registers
the workload (*"Openings `0..2` … played once into ONE report"*) but names no
config for pass 1, so the report's `hang_timeout_ms` — which C3 and §3.4 both
read as 120 s — is assumed from the sweep's seat rather than registered here.
`docs/process.md`'s instrument-governing-revision rule binds this too.

**m8 — D-423.** The 742 / 347 / 2.1383 / 0.5323 block is stated in §1, §2, §2.1,
§4.1 and §4.5. §4.1 owns it; the other four should point there. M2 is what
happens when they do not.

---

# WHAT SURVIVED ATTACK

**S1 — the isolation chain in §2 is exactly as described, verified line by
line.** `capture.rs:247`: `for line in [pistol_cli::protocol::NEW_GAME, position, go]`
— `newgame` really does precede every `position`/`go`.
`instance.rs:85-88`: `new_game` sets `self.state = GameState::new_game()` and
calls `self.searcher.clear()`. `search.rs:263-272`: `clear` calls
`table.clear()`, `heuristics.clear()` and `solver.reset()`.
`tt/mod.rs:108-112`: `Table::clear` fills every bucket with `EMPTY` **and resets
`generation` to 0 and `used` to 0** — I checked specifically for a generation
counter surviving, and it does not.
`heuristics.rs:60-65`: killers, pair killers, history and countermove all
cleared. Every claim §2 makes about this chain is true.

**S2 — attack 1(c): I found NO channel by which two identical `position` lines
can answer differently within one process.** This was the attack the dispatch
called most important, and it failed. What I checked and rejected:

- *Clock.* `ordering.rs:38-52` reads `Instant::now()` only when `deadline` is
  `Some`, and `search.rs:301-307` passes a fallback/deadline only under
  `Stop::Deadline`. The sweep's budget is `nodes 400000` → `Stop::Nodes`
  (`instance.rs:266-281`), so the ordering pass performs, in `ordering.rs`'s own
  words, *"zero clock reads"*. `search.rs:300`'s `started` feeds only `nps` and
  `time_ms`, which `capture::normalise` strips (`capture.rs:66-96`) — and which
  are absent from the pilot's capture file, confirmed by grep.
- *Hash iteration order / allocation addresses.* `Board` is a
  `BTreeMap<Coord, Player>` (`board.rs:52-55`), so `Board::stones()` is sorted.
  `Position::reset_to` (`position.rs:55-71`) rebuilds eval and `ThreatState`
  over that sorted order from a state freshly replayed from the `position` line,
  so two identical lines produce identical internal layouts. `heuristics`'
  maps are only ever probed by key, never iterated. Nothing on a choice path
  iterates an unseeded hash.
- *A counter surviving `clear`.* Two survive — `census` and `census_folds`
  (`search.rs:71-83`) — and neither is read by the search, neither is printed,
  and §4.2 refuses census with the cache. Recorded as m2, not as a channel.
- *The solver.* `instrument_v0.toml:113` has `on_search_path = false`, so
  `solver_wiring` returns `None` (`instance.rs:186-188`) and the field is `None`
  by construction.
- *TT aging.* `new_generation` (`tt/mod.rs:116-118`) increments from the 0 that
  `clear` just set, so every ask searches at generation 1.

**S3 — §4.1's count re-derives exactly, under a second instrument I wrote
myself.** Over `corpus_v1.txt` (`sha256sum` confirms `493f4fa8b6fb…873af9c4f`),
742 body records, all 16 fields:

```
distinct position lines (revision 2's key)   347
distinct raw move lists                      347
distinct key_seq                             347
distinct key_pos                             347
distinct key_full (sorted cell:player)       347
ratio  742/347                            2.138329
hits   742-347 = 395   hit rate           0.532345
1 - 1/ratio                               0.532345
```

Every figure in §4.1's block is correct to the digits printed, the arithmetic
relating 2.1383 to 0.5323 is exact, and the inference that all-four-equal implies
no transposition and no symmetric pair is valid (`key_pos` is a function of the
move list; equal distinct counts make it injective on the observed set).

**S4 — the code claim about `played()` is right at the cited lines, and the
enumeration is complete.** `heuristics.rs:191-193` is `last_stone`, reading
`state.played().last()`; `:155` is `&& let Some(at) = last_stone(state)` inside
the `gates.countermove` arm; `:89` is `for played in state.played()` inside
`record_cutoff`, reached under `gates.killers`. `git grep -n "played()" --
'crates/*' | LC_ALL=C sort` gives 23 hits; the only ones in production search
code are `heuristics.rs:89` and `:192` (`:412` is inside `#[cfg(test)]`), the rest
being `pistol-core`/`pistol-cli` book and test code. **So the claim that revision
1's argument was FALSE is correct, and I found no play-order-dependent read it
missed.** `configs/instrument_v0.toml:79-81` does set all three gates `false`,
exactly as quoted.

**S5 — the 1.5 threshold really was registered before the count.** Revision 1
(committing §4.1's *"If the ratio is below **1.5**, lever B is dropped"*) was
committed at `57c660c`, **2026-09-02 17:27:17 +0200**; the receipt is dated
**2026-09-02T17:43:30+02:00**. Sixteen minutes, in the right order. The
after-the-numbers-threshold attack fails.

**S6 — `0.8854` is MEASURED and correctly derived.**
`artifacts/wp20pilot_RUN_2cd4f79_v1.txt:39` reads `capture1 seconds=657` for the
742-record capture (`:35`), and 657/742 = 0.88544. The replication at `:44`
(`capture2 seconds=658`) and the byte-identity at `:45-47` are on the record too.

**S7 — every other arithmetic claim in §1, §4.4 and §5 checks out.**
11 017/12 869 = 85.6% ✓. 3.26 h: `2 x (12 869 - 0.5323 x 11 017)` = 14 012 s
against 25 738 s, difference 11 726 s = 3.26 h ✓. 48.95 h → 22.89 h ✓. §5's
"~1.43 h" for a cached tranche capture = `0.4677 x 11 017` = 5 152 s ✓. §4.4's
pilot pair "~11 min + ~5 min" = 657 s and 307 s ✓.

**S8 — lever A's field {1, 2, 4, 8, 16} is sound and the box is as claimed.**
`lscpu`: `AMD Ryzen 7 3700X`, `Core(s) per socket: 8`, `Thread(s) per core: 2`,
`CPU(s): 16`. The divides-16 argument is correct, the N=12 exclusion is correct
(twelve then four leaves two thirds idle for the second wave), and the
consequence — *"if the sweep's concurrency changes, the TRANCHE COUNT changes
with it"* — is registered as a rule with a named reopening, not left as a note.

**S9 — §3.4's decision rule is genuinely pre-registered and free of
after-the-numbers freedom.** Highest median throughput; ties inside 5% to the
smaller N; must divide the tranche count; refused above the watchdog; SMT note
if 16 wins. Three reps in rotated setting order, with the `wp20b_perf_guard.sh`
thermal-ramp lesson cited. §3.5's separation of DESIGN ELEMENTS from CRITERIA is
the right distinction and is the kind of thing that stops a closure reporting
four criteria met when two were never at risk. §3.6 gives every criterion a
registered consequence, as `docs/process.md` requires, and C1's consequence
(**the sweep stops, not the study**) is the correct escalation.

**S10 — C4 is a real external referent and it survives the obvious attack on
it.** The pilot's 0.8854 was measured by a different run of a different
document, so it does not share the suspect stage. I attacked it (see A3) and
the attack failed.

**S11 — §4.4 remains an EXTERNAL referent after the amendment, and the
amendment fits the referent to the budget in the legitimate direction.** The
uncached pass is *"the pass that exists today"* and shares no code with the
cache — that is what makes it external, and moving *which report* it runs on does
not change that. The amendment reduces the cost of the **referent** (by reusing
work the sweep must do anyway) without reducing the coverage of the **check**:
the cached side is still a full tranche-sized capture, still compared with
`cmp -s`, still on 742 records as well. Fitting the check to the budget would
have been shrinking the comparison to a sample; §4.3 explicitly refuses that.
This is the right shape.

**S12 — §2.1's rejection of R1's canonical key is correct and well-argued.**
D-570's identity folds symmetries; a hit under it returns a `bestmove` in the
wrong frame, and node counts are not symmetry-invariant because the tie-break is
lexicographic (`instrument_v0.toml:99`, `tie_break = "lexicographic"`; D-137).
The measured half is right too: my S3 count confirms `key_pos == key_full == 347`,
so the extra fold merges nothing on this population. Declining a coarser key that
buys a measured zero in exchange for a class of wrong answer is the correct
trade, and §2's *"a lost saving, never a wrong answer, which is the direction an
error in a cache must fall"* is the right principle stated the right way round.

**S13 — the D-562(2) reasoning holds.** *"A cache drops no record"* is true of
§4.2's design, and D-562(2)'s objection is specifically to dedup at capture
destroying the duplication factor. The cache preserves every record and its
`position` field, so the factor survives. §2's stronger claim — that the
three-key question is *"unreachable from here"* because the cache only ever
compares a key with itself — is also correct.

**S14 — §3.1's citations are accurate.** `docs/book_v2_ledger.md:41` does record
`0..12` consumed by the pilot, and D-539 does say, in the architect's quoted
words, *"the pilot carries no census and **is not corpus**"*. §6's three
exclusions are all correct: `capture::run` does walk one channel by construction
(`capture.rs:338-364`), and D-558(2)'s one-engine refusal is at
`capture.rs:144-162`.

---

# ATTACKS I ATTEMPTED AND REJECTED

**A1 — "A within-process channel makes the cache unsound."** The dispatch's
priority-one attack, pursued through the transposition table's generation
counter, the census collector, `Instant::now()` on the ordering path, hash
iteration order in the heuristic tables, `Board::stones()`'s iteration, the
eval's incremental apply/undo, `ThreatState`'s rebuild order, and the solver.
**Rejected in full** — see S2 for each. The one thing that does survive `clear`
(the census collector) cannot move an answer and is refused in combination with
the cache anyway. The key is sound. Note that even if I had missed something,
§4.4's byte-identity comparison catches exactly this failure by construction: an
uncached pass's second ask would differ from its first, and the cached pass would
return the first, producing a diff. The safety net is real; it just is not the
gate §2 claims it is (M1).

**A2 — "The document's enumeration of play-order-dependent reads is
incomplete."** `git grep -n "played()"` over `crates/` returns 23 hits; I
classified every one. Only `heuristics.rs:89` and `:192` are production search
reads, and both are cited. **Rejected** — the enumeration is complete.

**A3 — "C4 will fire spuriously, because §3.1's workload is a different
composition from the pilot's, so the study voids itself for a reason that is not
the box."** This looked strong: the pilot's 26 games include mate-in-N positions
that terminate in 5 to 16 757 nodes rather than 400 384, and openings `0..2` are
a different mix. **Measured, and rejected.** Mean `search_nodes` over all 742
pilot records is **342 359**; over games 0-5 (openings `0..2`) it is **343 788** —
a ratio of **1.0042**. Per-opening means range 239 328 to 400 384, but the
three-opening subset lands within half a percent of the whole. C4's ±20% band has
about forty times the margin the composition difference needs. The criterion is
safe.

**A4 — "The pilot's 2.1383 will not generalise to a 218-opening tranche, so the
projected saving is overstated."** Modelling the structure S3 revealed — two
prefixes shared by all G games, the rest paired by seat — gives, for `G` games at
28.538 records/game, `ratio = T / (2 + T/2 - G)`, which **rises** to ~2.151 as G
grows. So a 218-opening tranche duplicates slightly *more* than the pilot, not
less, and the projection is conservative. **Rejected.** (Related: `wp21_prereg.md`
§3's distinct estimate of 5 819 per tranche scales the pilot's per-opening
distinct count and so double-counts the two shared prefixes; the structural model
gives 5 787, 0.5% lower. The direction again favours the estimate being safe, so
this is not a finding.)

**A5 — "The 1.5 threshold was set or moved after the count was seen."**
Rejected on timestamps — see S5.

**A6 — "`position_line` can spell one turn two ways, so the finest-key claim is
false."** Rejected: `Turn::Pair` is canonical by type invariant and `FromStr`
refuses an uncanonical pair, so no report can carry a non-canonical spelling to
`position_line`. The claim holds (the attribution does not — m4).

**A7 — "The position line is not strictly finer than `key_seq`, because
`key_seq` is move-order sensitive too."** Rejected: `key_seq` is
`canonical_sequence`, i.e. the move sequence **up to a symmetry**, so equality of
position lines implies equality of `key_seq` but not conversely. §2's "strictly
finer than `key_seq`, `key_pos` and `key_full` alike" is correct.

**A8 — "§4.4's amended referent stops being external because it is now the
sweep's own work."** Rejected — see S11. What makes a referent external is that
it does not share the suspect stage, not that it was produced for the check.

**A9 — "Anything in the document still asserts the 3.26 h saving as if it
applied."** Partially rejected. §5 does state 3.26 h and 48.95→22.89 h, but the
very next paragraph qualifies them, and §4.4 mentions 3.26 h only as the figure
the old referent lost against. So the document does not *assert* the
inapplicable number — it just never computes the applicable one. That is M9, and
it is a weaker finding than the one I went looking for.
