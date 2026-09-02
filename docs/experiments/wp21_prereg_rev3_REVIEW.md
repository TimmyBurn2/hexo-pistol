# REVIEW — `docs/experiments/wp21_prereg.md` revision 3. FRESH CONTEXT.

**NAMED REVISION.** `git stash create` object
**`fce50bc5b00baab9066f1e7bdf10c48c025b7755`** on `overnight2-stopped`
(HEAD `2b94f04`).

**DOES IT STILL MATCH THE WORKING TREE?** **YES.**
`git diff fce50bc5b00baab9066f1e7bdf10c48c025b7755 -- docs/experiments/wp21_prereg.md`
prints nothing. So does the same command scoped to
`docs/experiments/wp21_throughput_prereg.md`, the sibling — noted because the
sibling *was* edited under this reviewer mid-session (the harness reported the
gate-9 correction landing on disk) and was then restored to `fce50bc`'s bytes.
That is `arc3_ledger.md` F-1.5's own episode, and both files match the stash as I
write.

**WHAT I READ.** `CLAUDE.md`; `docs/process.md`;
`docs/experiments/wp21_throughput_prereg.md` revision 2;
`docs/experiments/wp21_DISPATCH.md`; `docs/experiments/overnight2_ledger.md` §2;
`docs/experiments/arc3_ledger.md`; `docs/book_v2_ledger.md`;
`docs/decisions.md` (D-537, D-539, D-540, D-553, D-556, D-558, D-560, D-561,
D-562, D-563, D-568, D-570, D-572, D-573, D-574 and the tail);
`tools/wp21_tranche_config.py`;
`crates/pistol-arena/tests/wp21_tranche_config_tests.rs`;
`tools/cold_label_check.py`; `crates/pistol-arena/src/capture_file.rs`;
`crates/pistol-arena/src/capture.rs`; `crates/pistol-arena/src/outpath.rs`;
`crates/pistol-arena/src/conclusion.rs`; `crates/pistol-arena/src/replay.rs`;
`configs/instrument_v0.toml`; `configs/random_openings_v2.toml`;
`crates/pistol-cli/tests/fixtures/random_openings_v2.txt`;
`artifacts/arc3_leverB_41_count.txt`;
`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`; `docs/experiments/wp20_CLOSURE.md`;
`docs/experiments/wp20_pilot_prereg.md` (the replay invocation only).

No `cargo` was run. Every count below was taken with `/usr/bin/grep`, `git show`,
`python3` or `wc`, with the command and its scope printed beside the claim.

## VERDICT: **FAIL**

4 BLOCKING, 12 MAJOR, 7 minor. The partition is exactly right and the per-unit
arithmetic almost all reproduces — the document's arithmetic is the strongest
part of it. What fails is the frame around it: the ruling the whole revision-3
amendment rests on has no ADR line, the run has no registered commands and no dry
run, the rule that protects the corpus from an unverified cache is not recorded
anywhere a successor can read, and the void rule contradicts the governing
dispatch in a direction that silently shrinks the number the closure leads with.

---

# BLOCKING

## B1 — **D-576 does not exist.** The whole revision-3 amendment cites a decision the ADR log does not carry.

Line 13: *"**THE SWEEP RUNS WITH A LABEL CACHE** (R1, landed **D-576**)"*, and
§1's cache row: *"R1 / D-576, key per `wp21_throughput_prereg.md` §2 revision 2"*.

```
$ /usr/bin/grep -oE '\bD-[0-9]{3}\b' docs/decisions.md | sort -u | sort -t- -k2 -n | tail -5
D-570
D-571
D-572
D-573
D-574
$ /usr/bin/grep -rn 'D-57[567]' docs/decisions.md
(no output)
$ /usr/bin/grep -rln 'D-576' --include='*.md' --include='*.rs' --include='*.py' .
./docs/experiments/wp21_prereg.md
./docs/experiments/wp21_throughput_prereg.md
```

`docs/decisions.md` ends at **D-574**. The only two files in the repository that
mention D-576 are the two registrations that cite it. So the cache — which
re-derives §3's capture figures, amends T-A, adds §1's table row and adds §6.1
entire — rests on an ADR line **nobody has written**.

This is not a formatting nit. D-568's own text condemns exactly this defect in as
many words: *"a dispatch citing a key no ADR log holds sends a successor grepping
for a decision that does not exist."* The arc wrote that law for `D-56q` and then
broke it four documents later. A successor who reads §1's cache row and goes
looking for D-576 to find out what R1 actually approved finds nothing — and R1
itself is not transcribed in the tree either (`arc3_ledger.md` F-0.2 records that
the arc III dispatch is transcribed nowhere; the ledger quotes one sentence of R1
and no more).

Worse: `arc3_ledger.md` F-0.3 records that **the ruling's key was DECLINED** —
R1 named the WP-2.0b canonical identity and the registrations key on something
else. That declination is precisely the kind of non-obvious design choice hard
rule 10 requires an ADR line for, and the line that is supposed to carry it is
the one that does not exist.

**FIX.** Append the D-line (at the next free number, D-575) recording R1, the
approval, the declined key and the amendment, then re-cite it here. If the
architect's ruling is to be quoted, transcribe it — `wp21_DISPATCH.md` §5 is the
precedent for transcribing a dispatch that is otherwise locatable nowhere.

## B2 — There is no registered command block and **no dry run**, which the dispatch asks for by name and `docs/process.md` requires before this review passes.

`docs/process.md`, "Dry-run discipline":

> *"A pre-registration's literal commands are exercised before its review passes,
> on an input of the SAME KIND as the registered workload … The pre-registration
> records the dry-run input and its output."*

The governing dispatch asks for it explicitly (`wp21_DISPATCH.md` §5, the
transcribed registration clause): *"the same honesty rules apply — D-479/D-483,
exits quoted at a SHA, **dry run on a stand-in tranche**"*.

```
$ /usr/bin/grep -n "arena --\|dry run\|DRY RUN\|dry-run" docs/experiments/wp21_prereg.md
102:| **label budget** | `nodes 400000` | … a command-line argument to `arena --capture` …
256:<SWEEP_DIR>/tranche-<n>/report.txt      pass 1's report        (arena --out)
257:<SWEEP_DIR>/tranche-<n>/capture.txt     pass 2's capture       (arena --capture --out)
258:<SWEEP_DIR>/tranche-<n>/corpus.txt      pass 3's corpus        (arena --labels --out)
```

Three parenthetical flag fragments and the string "dry run" nowhere in the
document. The registration never states the invocation for any of its five
instruments: the `--label-nodes 400000` spelling, the `--workers 1` on replay,
the `--stride 200` and the `--binary`/`--engine-config` arguments to
`cold_label_check.py`, the `corpus-check` call, or the `cmp -s` of T-F. The only
place a command list exists in the tree is `overnight2_ledger.md` §2's *"what a
successor does to start it"*, which is a ledger note, is not part of this
registration, and covers only the generator.

This is BLOCKING rather than MAJOR because sixteen tranches run over days from a
resume point: two sessions reading "pass 2's capture (`arena --capture --out`)"
can and will spell the label budget, the stride and the worker count differently,
and the assembled corpus is then labelled by two instruments with nothing in the
record saying so. That is a wrong conclusion, not an inconvenience.

**FIX.** Add the three-pass command block plus the four criterion commands,
verbatim, and run them on a stand-in tranche — the pilot's own `0..12` range is
the obvious same-kind input that costs no unseen opening (the sibling's §3.1 uses
it for exactly this reason) — recording input and output as process.md requires.

## B3 — **Nothing in the registered run log distinguishes a cached tranche from an uncached one**, so §6.1's protective rule is not checkable by a successor and the closure cannot say which tranches rest on the byte-identity check.

§6.1: *"**no tranche runs cached until that comparison returns byte-identity**"*.
§1's cache row: *"Tranche one and everything in its wave run UNCACHED as §4.4's
referent."*

§5 registers what the run log holds, in full:

> *"revision, binary digests, then each pass with its exit status and its
> seconds."*

Revision, binary digests, exit statuses, seconds. **No cache state.** The cache
is a change inside `pistol-arena`, so a cached tranche and an uncached tranche
run the same `target/release/arena` and print the same `binary … sha256 …` line;
whatever selects between them (a flag? an env var? a build feature?) is never
named anywhere in the registration. §1's table row says the cache is "ON" for the
sweep and §6.1 says half the sweep runs with it off, and the document supplies no
mechanism and no receipt for either.

The consequence is not hypothetical. §6.1's rule exists because *"a cached
tranche taken on the strength of a check that has not returned is a corpus whose
correctness is being assumed"* (sibling §4.4). If the rule is violated — by a
successor resuming from the log, which §5 designates *"the resume point"* — the
log is the only place the violation could show, and it cannot. At closure, the
question *which tranches' labels depend on an unverified cache* has no answer in
any registered artifact. That makes the corpus's provenance unreadable.

**FIX.** Register (a) how a tranche is run cached vs uncached, by its literal
spelling; (b) a per-tranche `cache on|off` line in the run log block, adjacent to
the exit statuses; and (c) the §4.4 comparison's own verdict line in the log, so
"the comparison returned byte-identity" is a receipt and not a memory.

## B4 — The void rule **contradicts the governing dispatch** and lets the corpus silently deliver less than §3 registers, with nothing required to report the shortfall.

`wp21_DISPATCH.md` §5, transcribed verbatim as the binding registration clause:

> *"Failure handling: a failed tranche is VOID **and re-run whole**; a partial
> tranche is never assembled; two consecutive void tranches = STOP."*

`wp21_prereg.md` §4: *"A tranche that fails any criterion is **VOID AS A
WHOLE**: its artifacts are kept, its ledger row records the failure, and **no
part of it enters the corpus**."* §5: *"A successor reads the log, finds the last
tranche with a PASS or VOID verdict, and starts at the next one. **A completed
tranche is never re-run**."*

The dispatch says a void tranche **is** re-run. The registration says it is
**not**, and moves on. The registration never states that it supersedes the
dispatch here, never gives the reason (which exists — `book_v2_ledger.md`: *"a
tranche that voids does not give its openings back"*), and never registers what
the supersession costs.

What it costs is the headline number. With the two-consecutive-voids STOP as the
only brake, up to **eight** tranches (alternating) can void without stopping the
sweep. Each is ~218 openings ≈ **5 819 distinct positions**. The closure's first
number is, in the dispatch's own words, *"distinct labelled positions
delivered"*, and §3 registers **93 076** as the estimate for it. Nothing in §4,
§5 or §6 requires the closure to state how many openings were voided or to
report the delivered count against the number of openings actually labelled
rather than against 3 487. A closure that reports "N distinct positions
delivered" beside a registration that says "93 076 ESTIMATED from 3 487
openings", when the real denominator was 3 051, is a wrong conclusion presented
as a shortfall in the per-opening yield.

**FIX.** Either (a) adopt the dispatch's re-run-whole rule, or (b) state
explicitly that the registration supersedes it, give the reason, and register a
closure obligation: **the delivered distinct count is reported against the
openings ACTUALLY labelled**, with the voided ranges enumerated, and the ledger
row's `13..3499` annotated with what of it never became corpus.

---

# MAJOR

## M1 — The hit rate **0.5323 is not measured under the key the cache uses.** The receipt measures a different, coarser key, and the row the two registrations attribute to it is not in it.

§3: *"`wp21_throughput_prereg.md` §4.1's count over the pilot's own corpus: 742
asked prefixes, **347 distinct under the cache's exact key**, hit rate **0.5323
MEASURED**."*

The cache's exact key is, per the sibling §2 revision 2 and `arc3_ledger.md`
F-1.4, *"the `position` line's exact bytes"*. The receipt:

```
$ cat artifacts/arc3_leverB_41_count.txt
…
key: the replayed position's sorted (cell, player) list, wp21_throughput_prereg.md §2
derivation: parsed from each record's own `moves` column; no engine involved

asked prefixes          742
distinct sorted-stone   347
distinct key_pos column 347
distinct key_full column 347
…
```

**Three** distinct columns, and the key named in the receipt's own header is the
**sorted (cell, player) list** — revision 1's key, the one revision 2 *replaced
because its soundness argument was false*. There is no `distinct raw move lists`
row. The sibling's §4.1 quotes a four-row block whose first row —
*"distinct raw move lists (§2's key)    347"* — **does not exist in the receipt
it cites**, and `arc3_ledger.md` F-1.4 repeats the same fabricated fourth column
(*"347, 347, 347, 347"*), while the ledger's own §1 count block, twenty lines
earlier in the same file, lists only the three the receipt has.

So the number this document calls MEASURED under the cache's exact key was
measured under a key the cache does not use. The direction is bounded but not
free: a finer key can only raise the distinct count, and the structural argument
(self-match, each prefix asked exactly twice) caps it at 742/2 = **371**. So
distinct ∈ [347, 371] and the hit rate ∈ [0.5000, 0.5323] — the registered figure
is the **best case**, and §3's cached tranche (1.95 h) is a floor of a floor, up
to 1.07x optimistic.

This is the arc's own named recurring defect — *"a claim asserted at the scope
where it was convenient rather than derived at the scope where it is true"* —
appearing for the fourth time, in the document that quotes the receipt.

**FIX.** Take the count under the actual key (it costs milliseconds and needs no
engine — `sort -u` over the `position` column of `corpus_v1.txt`), print the
command with its scope, and re-derive §3's cached block from it. Fix the
sibling's §4.1 block and `arc3_ledger.md` F-1.4 to quote the receipt they have.

## M2 — §3 quotes as **MEASURED** a figure its own source says is **not governed**.

The sibling's §4.1, in the same revision this document is amended against:

> *"This count was taken **before** the first fresh-context review of the
> document that registers it … the count is RE-TAKEN after this revision passes
> review, and the post-review run is the one every downstream number cites. The
> figures above are this document's own record of what was seen; **they are not
> governed until the re-take**."*

`arc3_ledger.md` F-1.1 says it again: *"Until then no number above may be quoted
as governed."* The ledger's §1 table row reads *"taken UNGOVERNED | done, and
**re-take owed**"*.

§3 of this document quotes 0.5323 and 2.1383 flat, labels them MEASURED, and
carries no note that they are ungoverned pending a re-take. A reader of this
document alone would not know. That is the two-documents-one-claim defect running
in the direction D-423 warns about: the qualification lives in the section that
owns the number, and the section that *uses* it drops it.

**FIX.** One clause in §3: the count is ungoverned until the sibling's re-take,
and this document's cached figures move with it.

## M3 — The replay estimate applies a 4-worker pilot rate to a criterion that mandates one worker. The tranche wall is **~8% understated**, and the error is exactly the one the play line's `x4` exists to prevent.

§3:

```
ESTIMATED  play    436 x 0.8271 x 4     =  1 442 s   (x4: the measured rate is a 4-worker THROUGHPUT and this seat runs one)
ESTIMATED  replay  436 x 0.81           =    353 s   (from the pilot's 21 s over 26 games)
```

The pilot's replay ran at four workers:

```
$ /usr/bin/grep -n -- "--replay" docs/experiments/wp20_pilot_prereg.md
297:**THE INSTRUMENT.** `arena --replay <report> --out <path> --workers <n>` at `31c1cc1`.
1059:"$A" --replay "$ART/report_v1.txt" --out "$ART/replay_v1.txt" --workers 4
1149:| command | `--workers` on `--replay` | §4C — `4`; …
```

`crates/pistol-arena/src/replay.rs:20,33` (`pub fn run(transcript, workers)`,
`for _ in 0..workers.min(…)`) confirms replay really does fan out. And **T-B
requires `--workers 1`**. So `0.81 s/game` is a four-worker throughput exactly as
`0.8271` is, and the same `x4` applies:

```
replay, corrected    436 x 0.81 x 4  = 1 413 s
tranche, corrected   11 017 + 1 442 + 1 413 + 57 = 13 928 s = 3.87 h
eight-way wall       2 x 3.87 = 7.74 h   (registered: ~7.2 h / "7.15 h")
```

The document applies the correction to one pilot rate and not to the other,
without saying why they differ, and the one it skipped is the one whose criterion
forces serial execution.

It propagates. The sibling's §1 rules out a lever on the strength of
*"capture 11 017 s of 12 870 s — 85.6%. Play is 11%, **replay 3%**"* and
concludes *"a lever that is not on the capture pass cannot matter … which is what
rules out … a faster replay"*. Corrected, capture is 79.1% and replay is
**10.1%** — still not the biggest lever, but the exclusion argument as written is
based on a figure three times too small.

**FIX.** `436 x 0.81 x 4`, with the same parenthetical the play line carries;
re-derive the tranche total, the two-wave wall, and tell the sibling.

## M4 — §6.1 attributes to *this* document a wall figure this document does not state.

§6.1: *"every remaining tranche runs uncached at **the 7.15 h bound this document
already registers**"*.

§3 registers *"two waves x ~3.6 h = **~7.2 hours**"*. `7.15` appears nowhere in
`wp21_prereg.md` except that one sentence claiming the document already
registered it. `7.15` is the sibling's number (`wp21_throughput_prereg.md`'s ONE
LINE and §5, three times). The exact sum is `2 x 12 869 / 3600 = 7.149 h`, so
7.15 is the right figure and **~7.2** is the wrong one — the document rounded
3.5748 to 3.6 and doubled the rounding.

So §6.1's *registered consequence* of a lever-B failure is stated as a number
that (a) this document does not carry, (b) contradicts by 0.05 h the number it
does carry, and (c) is wrong anyway under M3. A registered consequence quoted
from the wrong document is D-423's defect at the point where it does the most
damage: the consequence clause is what stops an after-the-numbers decision.

**FIX.** State the wall once, in §3, correctly; have §6.1 point at §3 rather than
restate a figure.

## M5 — The ONE LINE FOR THE MORNING asserts as settled the one parameter revision 3 un-settled.

Line 6: *"is partitioned into sixteen tranches **run eight at a time**"*.
Line 23, twelve lines later: *"**THE WORKER COUNT IS A REGISTERED SLOT UNTIL
LEVER A MEASURES IT** … the eight of §3 is the incumbent, not the answer."*

The ONE LINE is, by the document's own framing, the sentence a successor reads
first, and it states as fact the number the revision it heads exists partly to
convert into a slot. §1's table row got the amendment; the headline did not.

**FIX.** *"run N at a time, N a registered slot with incumbent 8."*

## M6 — §1's re-registration trigger **can never fire**, so lever A can move the wall four-fold without reopening this document — while §6.1 registers a consequence true only at N=8.

§1: *"If the selected N is not 8 the tranche count is re-registered to a multiple
of it, which reopens this document."*

The sibling's §3.2 fixes the field: *"Concurrency **N ∈ {1, 2, 4, 8, 16}**"*, and
§3.4 guards it with *"The selected N must divide the tranche count."* Every
member of {1, 2, 4, 8, 16} divides 16. So the tranche count never changes, and by
§1's own wording this document is never reopened — yet at N=4 the sweep is four
waves and the wall is **~14.3 h** (uncorrected) against the registered ~7.2, and
§6.1's registered consequence still names a bound from a setting that lost.

The sibling's §3.4 does say the selection *"may set `wp21_prereg.md`'s
concurrency … by an amendment that reopens that document's review"* — which is
the right rule. §1 states the narrower one, and the narrower one is the one a
successor reading this document will apply.

**FIX.** §1's trigger becomes: *any* selected N other than the incumbent reopens
this document, because the wall arithmetic and §6.1's consequence are stated at
N=8.

## M7 — T-A's defect column claims to exclude a cache defect on tranches that have no cache.

T-A's defect class: *"and, under R1's amendment, **a cache that answers a hit
differently from the search that produced the miss**"*.

§1 and §6.1 put tranche one and its whole wave — at the incumbent N=8, **half the
sweep** — on the uncached path. On those eight tranches there is no cache, so the
named defect cannot be present and the criterion passes vacuously with respect to
it. That is `docs/process.md`'s own test failed: *"A criterion that is a property
the named defect class PRESERVES … passes vacuously and is not a criterion."*

The document is *aware* of the record-list/cache distinction — the §4 note says
*"the derivation is over the record list, not over the cache"* and explains why
the two tranches are comparable. What it never says is the consequence: **T-A's
second defect class is only in force on cached tranches.** A closure reporting
"T-A passed on all sixteen" would be reporting one criterion answered on sixteen
and a second answered on eight.

**FIX.** Split the column: T-A excludes the warm/cold defect on every tranche and
the cache defect only on tranches run cached; the closure reports the two counts
separately.

## M8 — The T-A amendment names an instrument that cannot perform it, and quotes an output line that cannot express the answer.

T-A: *"`cold_label_check: N of N sampled record(s) agree byte for byte`, exit 0,
**stride 200 over cache HITS and stride 200 over cache MISSES, sampled and
reported separately**"*.

`tools/cold_label_check.py` as shipped takes **one** `--stride` and samples over
the whole record list:

```python
sampled = [(at, row) for at, row in enumerate(rows) if at % stride == 0]
…
say(f"{len(sampled)} of {len(sampled)} sampled record(s) agree byte for byte")
```

It has no notion of hit or miss, no second stride, and one summary line. §4 says
*"`tools/cold_label_check.py` gains that derivation"* — so the instrument the
criterion quotes **does not yet exist**, and the sentence the criterion quotes is
the *un-amended* tool's. Two invocations of the amended tool would print two
identical-shaped `N of N` lines with nothing in either saying which class it
sampled, so the run log cannot be read to tell whether both classes were checked
or one was checked twice — which is the whole point of the amendment.

I confirm the derivation *is* possible (see WHAT SURVIVED): `CaptureRecord`
carries `position` — *"The `position` line as sent"* — and `Capture.records` is
documented *"in the order they were asked"*, so first-occurrence-is-a-miss is
sound. The gap is the instrument and the receipt, not the idea.

**FIX.** Register the amended tool's actual invocation and its actual output
lines — the class must be on the line (`… over cache HITS …`) — and note that the
amended script is itself a `tools/` change owing the SHELL_CHECKLIST coverage
rule and a test driving the shipped script, exactly as the generator does.

## M9 — T-A does not say what an **empty class** does, and the two possible behaviours are a vacuous pass and a VOID.

The review asked this directly and the document is silent. Both outcomes are
reachable in the shipped code:

- `records_of()` raises `Void("holds no records, so there is nothing to re-ask")`
  → exit 2. A VOID is *not* a failure (the script's own header insists on the
  distinction), so a tranche whose miss-class was empty would produce a
  non-answer that T-A's *"exit 0"* clause reads as a failure and the script reads
  as no answer at all.
- If the amended tool instead filters an already-read list, `len(sampled)` is 0
  and it prints *"0 of 0 sampled record(s) agree byte for byte"* and exits 0 —
  a **vacuous pass**, which is exactly what `docs/process.md` forbids.

Neither is registered. For a full tranche neither class is empty (hits ≈ 6 624,
misses ≈ 5 819), so this is not going to bite tranche one — but T-F runs a
20-opening sub-range and the same instrument may be pointed at other captures,
and a criterion that is silent about its degenerate case is a criterion whose
reading is decided after the fact.

**FIX.** Register it: an empty class on a tranche that should have one is a VOID
and not a pass, and the criterion states the minimum sample it requires per class.

## M10 — §8 is an empty slot that line 43 says is filled, and it omits two instruments that produce registered numbers.

Line 43: *"Governing revision: the WP-2.0b closure head, **named in §8 with its
digest**."*

§8, in full, names no revision and no digest — it is a promise: *"Filled from the
closure head before tranche one."* So the document asserts a fact about itself
that is false, and this review cannot do what `CLAUDE.md`'s Process section asks
of it (*"a pre-registration is reviewed at the revision that GOVERNS the run"*)
because the governing revision is not stated. A slot filled later is an
amendment, which reopens this review again.

Two omissions besides:

1. **The instrument that produced §3's 0.5323 and 2.1383.** The receipt names it:
   *"script: **scratchpad count_key.py** (transcribed into the ledger's §1)"*.
   ```
   $ /usr/bin/find . -name 'count_key*' -not -path './.git/*'
   (no output)
   ```
   It does not exist in the tree. `docs/process.md`, "Instrument governing
   revision": *"An artefact that produces a registered number — a `tools/`
   script, **a scratchpad harness**, or a command block the document prints — is
   named in the pre-registration WITH ITS REVISION … `tools/` is where such
   artefacts usually live; living there is not what makes the rule apply."* The
   rule names the scratchpad case explicitly, and this is it.
2. **The assembly instrument.** §6 registers two manifests with **MEASURED**
   counts, including *"the **DEDUPED** manifest: one record per distinct position
   under D-562(2)'s default — three-key distinct, the deeper label wins, ties to
   the first"*. No such tool exists (`ls tools/` carries no dedup or assembly
   script; `arena --labels` writes a corpus, not a deduped manifest) and §8 names
   none. A registered MEASURED output with no registered instrument is a number
   whose producer is chosen after the run.

**FIX.** Fill §8 or delete line 43's claim that it is filled; commit
`count_key.py` (or its replacement under M1) to `tools/` with the coverage-rule
test and name it; name the assembly instrument, or move §6's manifests to a
successor package and say so.

## M11 — §4.1 exists to close the output-path gap and leaves three paths open — two of which `O_EXCL` turns into a refusal mid-sweep — and their wall cost is absent from §3.

§4.1: *"REVISION 1 FIXED THE CONFIGS AND NOT THE OUTPUT PATHS, which is a gap in
the run plan … and it is closed here."* Four paths follow, one capture per
tranche. Unregistered:

- **Tranche one's CACHED capture.** §4.4 of the sibling makes tranche one's own
  report the referent and compares *"captured uncached and captured cached"*. Two
  captures, one registered path. `crates/pistol-arena/src/outpath.rs:10-25`
  claims `--out` with `create_new(true)`, so the second write to
  `tranche-1/capture.txt` is *"a named refusal before any game"* — the very
  property §4.1 praises, firing against the run's own plan.
- **T-F's duplicate pair.** *"a second capture over a registered sub-range … is
  byte-identical to the first"* needs its own config, its own report and two
  capture paths. None registered.
- **The sub-range's generated config.** `tools/wp21_tranche_config.py` writes
  only the sixteen tranche slices; `--tranche 1` writes `take = 218`, not 20.
  Nothing in the registration says what writes the `skip = 13, take = 20` config
  T-F requires, and hand-writing it is the defect §2 says the generator exists to
  prevent.

The costs are missing too. §3's per-tranche block is uniform across sixteen
tranches, but tranche one additionally carries the cached re-capture
(~1.43 h ESTIMATED, the sibling's §5 figure) and T-F's pair
(20 openings ⇒ ~1 142 records ⇒ ~2 x 1 011 s ≈ 0.56 h plus two play passes). That
is roughly **+2 h on wave one**, unregistered in a document whose §3 is titled
"THE ARITHMETIC, SHOWN".

**FIX.** Register the three paths and the sub-range config's producer; add
tranche one's extra passes to §3 as their own line.

## M12 — *"THE DISPATCH SAYS 3,500"* — it does not. The number is D-568's, and the misattribution is in the paragraph whose entire job is to reconcile it.

Line 86: *"**THE DISPATCH SAYS "3,500" AND THE ARITHMETIC SAYS 3,487** … because
the dispatch's round number is the book's remainder before the pilot's thirteen
are subtracted."*

```
$ /usr/bin/grep -n '3,500\|3500\|3 500' docs/experiments/wp21_DISPATCH.md
(no output)
```

`wp21_DISPATCH.md` contains no 3,500 in any spelling; it registers *"the full
book_v2 range"*. The source of 3,500 is **D-568**, the architect's ruling:
*"book_v2 holdout of 1,000 openings reserved for governed runs, never labelled;
**sweep takes the remaining 3,500**"*.

That matters more than a wrong pointer. The paragraph is where the registration
declares that it is departing by 13 openings from a number in a governing
document — and it names the wrong governing document, so the departure is
recorded against a source that never said it. `overnight2_ledger.md` §2 repeats
the same misattribution verbatim (*"The dispatch says 3,500 and the arithmetic
says 3,487"*), which is how a wrong attribution becomes two documents' worth of
record.

**FIX.** *"D-568 says 3,500 and the arithmetic says 3,487"*, and correct the
ledger.

---

# minor

**m1 — §3's headline capture figure does not reproduce from the numbers §3
shows.** *"`ESTIMATED capture 199 027 x 0.8854 = 176 228 s`"*.

```
199027 * 0.8854      = 176 218.51
199027 * (657/742)   = 176 227.41   -> 176 228 (rounds up), the printed answer
```

The line is computed with the unrounded rate and *displayed* with the rounded
one, in the section titled "THE ARITHMETIC, SHOWN". The difference is 10 s and
both give 48.95 h, so nothing turns on it — but a shown derivation that does not
reproduce is the one thing this section may not be. Show `x 0.885445` or print
176 219.

**m2 — the cold-check line was not re-derived for the amended T-A, and its rate
has no source.** *"`ESTIMATED cold 63 x 0.9 = 57 s (stride 200, see §4)`"*.
`ceil(12443/200) = 63` is the *un-amended* single-stride count. Under T-A as
amended: `ceil(6624/200) + ceil(5819/200) = 34 + 30 = 64`. And `0.9` is nowhere
sourced — it is `0.8854` rounded up, and it is not among §3's own enumerated
inputs. 0.4% of the tranche, so it changes nothing; it is listed because §3's
claim is that revision 3's figures were *"re-derived … not scaled in prose"*, and
this one was neither.

**m3 — §3 enumerates "the four inputs and where each is actually read" and then
uses six.** The replay's `21 s over 26 games` and the cold check's `0.9` are both
load-bearing and both outside the enumeration. The replay one is M3.

**m4 — "Twenty is the smallest take that exceeds the pilot's thirteen."**
Fourteen is. The stated ground for the registered sub-range size is arithmetically
false. (The choice itself is fine and pre-registered; only its reason is wrong.)

**m5 — `outpath.rs:9-24` is off at both ends.**
```
$ /usr/bin/grep -n '' crates/pistol-arena/src/outpath.rs | sed -n '6,25p'
6:/// Claim `path` exclusively, creating it empty.
10:pub fn claim(path: &Path) -> Result<File, ArenaError> {
25:}
```
`claim` is 6-25 with its doc, 10-25 without. `9-24` is neither. This is
`arc3_ledger.md` F-1.5's own class — *"a citation asserted rather than read off
the file"* — in a document written after F-1.5 was recorded. (`instrument_v0.toml:113`
in the same table is **correct**; see WHAT SURVIVED.)

**m6 — line 46's *"its ledger row is added in the same commit"* is no longer
true.** `docs/book_v2_ledger.md`'s row is committed and names
*"`docs/experiments/wp21_prereg.md` **revision 2**"*; `overnight2_ledger.md` §2
says revision 2 too. Revision 3 is not a new row but it does stale two committed
pointers.

**m7 — T-C's first limb does not say where to look.** *"pass 1's summary reports
no forfeit"*. The pilot's stdout summary
(`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`) carries no forfeit token at all; the
counts live in the report file, written by
`crates/pistol-arena/src/conclusion.rs:81` (`counts n … forfeits …`) and `:111`
(`first_player_wins … forfeits N`). Name the line. The second limb (`end` column
`normal` on every record, readable off `corpus_check`'s
`end 1 (normal)`) is sound and carries the criterion on its own.

---

# WHAT SURVIVED ATTACK

**The partition is exactly right, and I re-derived all of it.**

```
$ python3 -c "..."
available 13..3499                     = 3487
15*218 + 1*217                         = 3487
3487 // 16 = 217 rem 15  -> 15 tranches of 218, 1 of 217
skip(1)=13 … tranche 16: skip=3283 take=217 end=3500
skip(17)                               = 3500  == the holdout's first opening
```
The recurrence lands on 3 500 exactly. The remainder really is spread over the
first fifteen, so no tranche is materially larger than another and §3's
per-tranche wall does hold for all sixteen, as §2 claims.

**The book really holds 4 500 openings, so the holdout really is the last 1 000.**
```
$ /usr/bin/grep -cv -e '^#' -e '^$' crates/pistol-cli/tests/fixtures/random_openings_v2.txt
4500
$ /usr/bin/grep -n '4500' configs/random_openings_v2.toml
50:n_openings = 4500
```
(The file is 4 576 lines; 76 are header. A cardinality check that agrees with its
source only after the comment lines are excluded is the kind that was worth
taking.) `3500..4499` is therefore the last 1 000, chosen by position and by
nothing observable, stated before the sweep. That argument is sound and it is the
document's best clause.

**The generator implements §2's partition, and the holdout test drives the
shipped script.** `tools/wp21_tranche_config.py`'s `slice_of` uses
`divmod(OPENINGS, TRANCHES)` with `OPENINGS = 4500 - 1000 - 13 = 3487` and gives
the extra opening to the first `extra = 15` tranches — identical to §2.
`no_tranche_reaches_the_reserved_holdout` in
`crates/pistol-arena/tests/wp21_tranche_config_tests.rs` **does** drive the
shipped script (`Command::new("python3").arg(repo().join("tools/wp21_tranche_config.py"))`),
over all sixteen tranches, asserting `skip + take <= 3500`. It is not a
re-implementation, it is not scoped to a sample of tranches, and it pins what the
arithmetic is *for* separately from the arithmetic. The suite also distinguishes
REFUSED (1) from VOID (2) at the exit-code level and drives the VOID path with a
read-only directory — `tools/SHELL_CHECKLIST.md` item 12 answered by a test
rather than by a constant. I attacked this file looking for a re-implementation
and did not find one.

**Every per-unit rate reproduces.**
```
742/13     = 57.0769    347/13   = 26.6923    742/347   = 2.13833
657/742    = 0.885445   21.505/26 = 0.827115
3487*2     = 6 974      3487*57.0769 = 199 027.15   3487*26.6923 = 93 076.05
218*57.0769 = 12 442.76 218*26.6923 = 5 818.92
12443*0.8854 = 11 017.03   436*0.8271*4 = 1 442.46   436*0.81 = 353.16
sum of the rounded parts = 11017+1442+353+57 = 12 869  (exact sum 12 869.35)
```
Every one matches the printed figure except m1's 176 228.

**The cache block's internal cross-check is real and it works.**
```
12443 * (1 - 0.5323) = 5 819.59  ->  5 819
218   * 26.6923      = 5 818.92  ->  5 819
1 - 1/2.1383         = 0.53234        1 - 347/742 = 0.53235
```
§3's *"(= 218 x 26.6923, the distinct count, as it must)"* is not decoration: it
is the check that would have caught a hit rate applied as a discount to a total
rather than as a distinct count, and it passes. Likewise *"THE HIT RATE AND THE
DUPLICATION FACTOR ARE ONE NUMBER READ TWO WAYS"* — verified. (M1 is about which
key was counted, not about this arithmetic.)

**The provenance paragraph for 347 is correct, and it is the paragraph I expected
to break.** §3 warns that the run log's `distinct-n 13` is distinct **GAMES** and
that 347 is not in the run log:
```
$ /usr/bin/grep -n "347" docs/experiments/wp20_CLOSURE.md
46:13 openings -> 26 games -> 742 records -> 347 distinct positions
50:   key_seq = key_pos = key_full = 347   all three keys agree on distinctness
```
Both quoted strings are in `wp20_CLOSURE.md` as quoted, and `distinct-n 13` is
indeed in the run log as games. 13, 26, 742, `capture1 seconds=657` and
`wall 21505 ms` are all in `artifacts/wp20pilot_RUN_2cd4f79_v1.txt` where §3 says
they are.

**T-A's derivation IS performable from the capture file alone.**
`crates/pistol-arena/src/capture_file.rs:27` — `position: String`, *"The
`position` line as sent"* — and `:44` — `records: Vec<CaptureRecord>`, *"The
records, in the order they were asked"*. `tools/cold_label_check.py`'s
`records_of` docstring says the same. Pass 2 is serial by construction, so
first-occurrence-is-a-miss is exact. The document is right that this needs no new
column and no format-version bump. (The instrument that must perform it is M8.)

**`instrument_v0.toml:113` is exactly right.**
```
113:on_search_path = false
```
So is the sibling's `79-81` (`killers`/`history`/`countermove` = `false`), which
this document's §1 depends on indirectly.

**`O_EXCL` is real, not asserted.** `outpath.rs` uses
`OpenOptions::new().write(true).create_new(true)`, one syscall, with a named
refusal. §4.1's *"belt and braces"* reading — the claim makes concurrency safe,
the directory makes a resume point readable — is correct as far as it goes
(M11 is about paths it forgot, not about this).

**T-F's sub-range is genuinely pre-registered and correct.** Tranche one is
`skip 13, take 218`, so its first 20 openings are `skip = 13, take = 20`, exactly
as §4 states, fixed before any tranche runs. The reason for a sub-range rather
than a whole re-run (a full re-run doubles the sweep; the pilot already ran the
whole-corpus form — `capture-determinism exit=0` is in the run log, verified) is
sound.

**The contention factor is handled correctly, and it is the clause I most
expected to be fudged.** §3 refuses to guess it, measures it from wave one
against the pilot's 0.8854, and says *"Nothing in this registration depends on
it."* That is true — and it composes: wave one runs **uncached** per §1, so its
realised seconds-per-label is comparable to the pilot's uncached rate. Had wave
one been cached, the contention measurement would have been meaningless, and the
document does not make that mistake.

**Other things I checked and found sound:** the census-off supersession states
its consequence rather than leaving it to a closure (*"this sweep starts no clock
against D-537's minimum"*); the two-consecutive-voids STOP matches the dispatch
verbatim; `<SWEEP_DIR>` on `/home` and never `/tmp`, with `CLAUDE.md`'s
Environment section as the ground; the SPRT block's
`inconclusive_degenerate`-by-construction note matches the pilot's own
`VERDICT inconclusive_degenerate`; the generator's refusal to overwrite an
existing `--out` (D-199) and its `binary_sha256` slot-pass rationale; §7's four
disclaimers, each of which I could tie to a cited ruling.

---

# ATTACKS I ATTEMPTED AND REJECTED

1. **"The remainder should have gone in the last tranche, and 16 x 218 = 3 488
   ≠ 3 487."** Rejected. The document says the remainder is spread over the FIRST
   tranches and both the arithmetic and `slice_of` do exactly that. `divmod` gives
   `base = 217, extra = 15`, and the recurrence closes on 3 500 with no gap and no
   overlap. The partition test asserts both `total == 3487` and
   `expected_skip == 3500`.

2. **"`3487/16 = 217.94`, so a 218/217 split is a choice made to look tidy."**
   Rejected. It is forced: any exhaustive contiguous partition of 3 487 into 16
   parts differing by at most one is 15x218 + 1x217.

3. **"The two derivations of the cached label count disagree."** Rejected — they
   agree to the integer (5 819.59 vs 5 818.92), which is what §3 predicts. This is
   the case where the derivation and its source *did* agree and the agreement is
   real, because the two paths are genuinely different (a rate applied to records
   vs a per-opening distinct count).

4. **"The `x4` on the play line over-corrects — perfect 4-worker scaling is a
   fiction."** Rejected as a finding, though the direction is worth knowing. The
   pilot's own log gives `compute a: 38 011 ms` + `compute b: 38 008 ms` = 76 s of
   engine CPU over 26 games ≈ 2.9 s/game serial, against the 3.31 s/game the `x4`
   implies. So the `x4` is *conservative* by ~12% — an over-estimate of cost, which
   is the safe direction, and it is labelled ESTIMATED. (This is exactly why M3's
   omission of the same correction on replay is a finding: the two lines are the
   same situation and only one was handled.)

5. **"The cache violates D-562(2)'s no-dedup-at-capture ruling, which §6 restates
   as load-bearing."** Rejected. A cache drops no record; every record is written
   with its own `position`, and the 2.14x duplication survives in the corpus
   intact. The sibling's §1 makes this argument and it holds.

6. **"The capture file's records might not be in ask order (workers, buffering,
   sorting), which would break the hit/miss derivation."** Rejected on the code:
   `Capture.records` is documented and written in ask order,
   `render_records` writes them in slice order, and the capture pass walks one
   channel.

7. **"The uncached/cached byte-identity comparison can't succeed because the
   capture header carries counts or digests that differ."** Rejected.
   `capture_sha256` is over `(format_version, experiment_sha256, label_go)` only,
   and `games`/`records` counts are cache-invariant. Nothing in the header depends
   on how a label was obtained.

8. **"`no_tranche_reaches_the_reserved_holdout` is a re-implementation of the
   partition and would pass on a broken script."** Rejected. It shells out to
   `tools/wp21_tranche_config.py` and parses the written file — the shipped
   script, per `docs/process.md`'s tools/ coverage rule.

9. **"The 4 500 is a transcribed number and the fixture really holds 4 576."**
   Rejected: 4 576 lines, 4 500 non-comment non-blank, and
   `configs/random_openings_v2.toml:50` says `n_openings = 4500`. Three sources
   agree.

10. **"T-C is unfalsifiable because a forfeited game can't reach a corpus
    anyway."** Rejected — the `end` column really does carry `forfeit` as a value
    (`conclusion.rs:25`), and `corpus_check`'s `end 1 (normal)` summary would show
    it. The criterion can fail. Only the *location* of its first limb is unclear
    (m7).

11. **"T-F's second capture will collide with the first under `O_EXCL`."**
    Rejected as stated — T-F's pair are two runs over a sub-range, not two writes
    to one path. The real defect is that neither path is registered at all, which
    is M11.

12. **"The `x` in `12 869 - 11 017 + 5 152 = 7 004` hides a rounding error."**
    Rejected. It is exact on the printed integers, and 7 004 s = 1.9456 h rounds to
    the printed 1.95 h.
