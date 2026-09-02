# REVIEW-design — `wp21_label_cache_design.md` revision 1. **VERDICT: FAIL.**

**REVISION READ**: `docs/experiments/wp21_label_cache_design.md` revision 1, at the
dispatched commit **`239f21f`**.

**DOES IT STILL MATCH HEAD? THE FILE DOES; THE TREE DOES NOT.** HEAD moved during
this review — it was `239f21f` when I started and is **`f1acc57`** now
(`git rev-parse HEAD`; `git rev-list --count f1acc57..HEAD` → `0`). The design
file itself is unchanged (`git diff --stat 239f21f HEAD -- docs/experiments/wp21_label_cache_design.md`
→ empty), **but `f1acc57` lands one of the seven changes the design's own §1
registers as work still to do** — see BLOCKING 2. All `file:line` citations below
are read at `f1acc57` unless a line is marked `@239f21f`; none of the code files
this review cites differ between the two.

**READ**: `CLAUDE.md`; `docs/process.md` (all seven sections, including the
standing re-derivation clause); `tools/SHELL_CHECKLIST.md`; `docs/decisions.md`
D-540, D-553, D-562, D-568, D-570, D-571, D-572, D-576, D-581, D-582, D-583;
`docs/experiments/wp21_throughput_prereg.md` revision 3 in full;
`docs/experiments/wp21_prereg.md` revision 4 §1/§4/§5/§6.1;
`matrix_label_cache_key.md` revision 3 and `..._REDTEAM.md`;
`wp21_throughput_prereg_rev3_REVIEW.md`; `arc3_ledger.md` §1c;
`crates/pistol-arena/src/{capture.rs,capture_file.rs,passes.rs,usage.rs,outpath.rs,labels.rs,labels_file.rs,bin/arena.rs}`;
`crates/pistol-engine/src/instance.rs`;
`crates/pistol-search/src/{heuristics.rs,params.rs,pvs.rs,search.rs}`;
`crates/pistol-core/src/symmetry.rs`; `tools/{ci.sh,determinism.sh,cold_label_check.py,file_justification_check.sh,governing_citation_check.sh,design_citation_check.py}`.

---

## RE-DERIVATION — commands I chose, each with its scope

`docs/process.md` §*Re-derivation*: a count reproduced only by running the
document's own command is not reproduced. Mine, none of them the document's:

| # | command, with its scope | returns | the document says |
|---|---|---|---|
| R1 | `awk -F'\t' '!/^#/ && NF>0 {n++; seen[$3]=1} END {print n, length(seen)}' /home/tom/pistol-runs/wp20pilot-artifacts/capture_v1.txt` — a field-3 set over the pilot capture's body, no shared code with `tools/label_cache_count.py` | `742 347` | 742 asks / 347 distinct — **REPRODUCED** |
| R2 | `/usr/bin/grep -v '^#' <same file> \| cut -f3 \| LC_ALL=C sort -u \| wc -l` — a second scope on the same file, no parser at all | `347` | **REPRODUCED** |
| R3 | `/usr/bin/grep -n "GATE_TOTAL\|determinism" tools/ci.sh`, scope = the whole gate script | `:21 readonly GATE_TOTAL=20`; `:109 step "gate 9/$GATE_TOTAL: cross-process determinism"`; `:110 gate "determinism" tools/determinism.sh` | *"CI gate 6"* (§ONE LINE) and *"CI gate 9 of 19 (`tools/ci.sh:104-105`)"* (§2) — **BOTH REFUTED**, see BLOCKING 5 |
| R4 | `/usr/bin/grep -n "SEATS" tools/determinism.sh` then read the array at `:67-81`, scope = the array and not the header prose | 5 seats: `radius`, `staged`, `staged-heuristics`, `staged-solver`, `staged-safety-net-cap` | *"five"* — **REPRODUCED** |
| R5 | `/usr/bin/grep -n "normalize()" tools/determinism.sh` | `:154 normalize() { sed -E 's/ nps [0-9]+ time [0-9]+//'; }` | *"the same normalisation `capture::normalise` applies"* (`capture.rs:66-96`) — **REPRODUCED** |
| R6 | `git grep -n "key_pos" -- crates/pistol-arena/src` — scope = the crate that DEFINES the column, not the prose that names it | `labels.rs:202 key_pos: state.key().to_string()`; `labels_file.rs:27` *"`GameState::key`: the same position up to transposition"*; `labels_file.rs:245` *"is not thirty-two hex digits"* | *"its transposition key (the replayed position's sorted `(cell, player)` list)"* — **REFUTED**, see MAJOR 6 |
| R7 | `git show 239f21f:tools/cold_label_check.py \| /usr/bin/grep -c partition` / same at HEAD | `0` / `6` | §1 row 6 registers `--partition` as work — **it landed at `f1acc57`**, see BLOCKING 2 |
| R8 | `git grep -n "capture::run" -- crates` — scope = every crate, `src` and `tests` alike | one call site, `passes.rs:59` | §1's three-row `capture.rs`/`passes.rs` split is complete *for the call site*; it is not complete for the return path (BLOCKING 2) |
| R9 | `git grep -ni "label_cache\|LabelCache\|label-cache" -- crates tools` | only `tools/label_cache_count.py` and its test; no `LabelCache`, no `--label-cache` | the cache genuinely does not exist — §5's *"the sites do not exist yet"* is true |
| R10 | `wc -l < crates/pistol-arena/src/capture.rs`; `/usr/bin/grep -n SOFT_CAP tools/file_justification_check.sh` | `389`; `SOFT_CAP=300` | design is silent on rule 9 — MAJOR 12 |
| R11 | `timeout 120 python3 tools/design_citation_check.py docs/experiments/wp21_label_cache_design.md` | `13 citation(s) checked, 0 unreproduced` | **and that is worthless here**: `design_citation_check.py:48-52` matches a path plus an optional line RANGE and checks only that the range is inside the file, so `tools/ci.sh:104-105` (a comment block) passes. The design is also not on `tools/governing_citation_check.sh`'s `GOVERNING` list (`:46-54`) — see MAJOR 17 |

---

# BLOCKING

## BLOCKING 1 — §1 row 1 REGISTERS THE ONE SHAPE §2.0 OF THE GOVERNING REGISTRATION FORBIDS, AND MAKES THE INVARIANT UNCHECKABLE

The design, §1 row 1 and row 3:

> | 1 | `crates/pistol-arena/src/capture.rs` | **`run` takes a `&mut LabelCache`**; before `ask`, a lookup on the `position` string; after `ask`, an insert |
> | 3 | `crates/pistol-arena/src/passes.rs` | `capture` **threads the cache from the binary's argument** |

The governing registration, `wp21_throughput_prereg.md:249-251` (§2.0, *"THE
INVARIANT, REGISTERED"*):

> *the memo is **constructed inside `capture::run` and dropped with it**; it is
> never a field of a longer-lived value, and it is never shared between two
> invocations.*

A `&mut LabelCache` parameter is by definition a memo constructed **outside**
`run` and outliving it. The two rows together say the map is built in
`passes::capture` (or in `bin/arena.rs`) and lent to `run` — which is the exact
shape the invariant names. `matrix_label_cache_key.md:134` and `:301` rest on the
same assumption (*"the memo is built inside `capture::run` and dropped with it"*,
*"a tranche-scoped key: identical to K1 within one `capture::run`, which is the
only scope the memo has"*), so the design contradicts two governing documents at
once.

**AND THE COST IS NOT PEDANTRY — IT IS THAT REVIEW-IMPL CANNOT CHECK IT.**
`capture::run` is `pub` (`capture.rs:326`). Under the design's signature, the
obligation *"never shared between two invocations"* is a property of **every
present and future caller**, and `wp21_throughput_prereg.md:257-260` makes it a
check the cache's own review must perform:

> **THIS IS AN OBLIGATION THE CACHE'S OWN REVIEW CHECKS**, not a remark: an
> implementer who lifts the memo to a struct reused across two `--label-nodes`
> values silently makes the key wrong, and §4.3 says exactly what that costs —
> nothing downstream would notice.

A reviewer handed `run(&mut LabelCache)` has no local evidence at all: the answer
lives at the construction site, and a future second caller re-opens it silently.
The design nowhere states the obligation, nowhere says who checks it, and nowhere
gives a reviewer a test that could fail.

**FIX.** `run` takes a two-state **mode** — `LabelCache::Off` / `LabelCache::On`,
or a bare `use_cache: bool` — and constructs the `BTreeMap` in its own body,
beside `let go = label_go_line(...)` at `capture.rs:333` where the other three
key arguments are already fixed. Then the invariant is one function's text, the
`&mut` disappears, and the mutation set gains a mutant that is actually
meaningful (`the memo is hoisted above the game loop's enclosing scope`). If the
`&mut` shape is kept for some reason this review cannot see, §1 must say why, and
the registration's §2.0 must be amended — which reopens its review.

## BLOCKING 2 — THE CHANGE LIST REACHES NEITHER END OF THE CODE IT CHANGES, AND ONE OF ITS SEVEN ROWS IS ALREADY IN THE TREE

Priority 1 of the dispatch. Four separate defects in one seven-row table.

**(a) THE COUNTERS AND THE HIT RATE HAVE NO ROUTE OUT AND NO PRINT SITE.** §1
row 7 puts the counters *"in `crates/pistol-arena/src/capture.rs`"*, and §1.1
says:

> They are **printed once per capture, beside the hit rate**, and the run log
> carries them per tranche.

`capture::run` returns `Result<Vec<CaptureRecord>, ArenaError>` (`capture.rs:330`).
Nothing prints in that function. Every line a capture pass prints is written at
`passes.rs:82-96`:

```
passes.rs:82   println!("arena: captured {} position(s) from {} game(s) at {go_line}", …)
passes.rs:87   println!("{}", crate::capture_file::manifest_row(…)?)
passes.rs:91   println!("arena: capture written to {}", out_path.display())
```

So the package requires (i) a change to `run`'s **return type** to carry the hit
count and the two counters out, and (ii) a new print site in `passes::capture`.
§1 row 3 registers only the direction **in** (*"threads the cache from the
binary's argument"*). **And nothing in the tree prints a hit rate at all today** —
`git grep -n "hit rate\|hit_rate" -- crates` returns nothing — so *"beside the
hit rate"* names an output the package must also invent, in a row the table does
not have. Two missing sites, both load-bearing: `wp21_throughput_prereg.md:219-222`
and `matrix_label_cache_key.md:249-253` register the counters as *"reported in
the run log beside the hit rate"*, and a number with no print site is a number
nobody can read.

**(b) `Mode::Capture`'S TUPLE.** `bin/arena.rs:36` is
`Capture(PathBuf, u64, bool)` and `:115` destructures three fields. The flag
needs a fourth, plus a new match arm; §1 row 4 says only *"one optional trailing
word"*, which under-describes it. Minor on its own; it matters because the
mutation set (MAJOR 11) has to name the site.

**(c) NO TEST SITE ANYWHERE.** The word `test` occurs three times in the design,
all inside §5's mutation sketch (*"dies at the hit-count test"*, *"dies at its own
test"*, *"a fixture"*). The change table has no row for
`crates/pistol-arena/tests/capture_tests.rs` or a new suite, and hard rule 7 plus
D-553's corollary make the tests part of the package, not a consequence of it.
This is not bookkeeping: D-553's whole point is that *which driver* a test uses
decides whether a call-site mutant dies, and a change list that does not name the
test file cannot register that. See MAJOR 11.

**(d) ROW 6 IS ALREADY DONE, AND THE ROW THAT ISN'T DONE IS MISSING.** §1 row 6:

> | 6 | `tools/cold_label_check.py` | `--partition hits\|misses\|all`, defaulting to nothing: the argument is required, and the summary line names the class |

`git show 239f21f:tools/cold_label_check.py | /usr/bin/grep -c partition` → `0`;
the same at HEAD → `6`. It landed at **`f1acc57`**
(`git log -S'--partition' --oneline -- tools/cold_label_check.py`), whose message
is *"the cold check samples cache hits and misses separately and names the class
it sampled"*, with a coverage suite
(`crates/pistol-arena/tests/cold_label_check_tests.rs`) and the required argument
at `tools/cold_label_check.py:253` (`parser.add_argument("--partition",
required=True, choices=PARTITIONS)`). The shipped file's own header even carries
the design's justification sentence verbatim (`cold_label_check.py:41-44`: *"A
default here would answer about `all` while a criterion said `hits`, which is the
one failure this argument exists to prevent"* against the design's *"a default
here would silently answer about `all` while a criterion said `hits`"*).

This is not merely stale. `arc3_ledger.md:637-661` orders the work **1. the label
cache package … 2. `tools/cold_label_check.py --partition`** as *separate* steps;
the design pulls step 2 into step 1's change list, and its own header line 14 says
**"THIS DOCUMENT DESIGNS ONLY WHAT §4.2 NAMES"** — and §4.2 of the registration
(`wp21_throughput_prereg.md:527-569`) names the memo, the switch, the map kind and
the census refusal, and says nothing about the cold check. §1 row 6 and the whole
of §4 are out of the design's own declared scope, and step 2 has now been
implemented and shipped **without this design being reviewed at all**.

**AND THE ONE COLD-CHECK CHANGE THAT IS STILL OWED IS NOT IN THE LIST.** §4 says:

> **AN EMPTY CLASS IS A VOID AND NOT A PASS**, registered at `wp21_prereg.md` §4:
> **fewer than ten sampled records** in a class a tranche should have makes T-A a
> VOID.

Two different rules are run together. The empty-class rule is implemented
(`cold_label_check.py:187-191`, `raise Void(f"{source} holds no {want}
record(s) …")`). The **ten-record floor** — registered at `wp21_prereg.md:346-348`,
*"a class with fewer than **10 sampled records** on a tranche that should have both
makes the tranche's T-A a **VOID**"* — is implemented nowhere:
`git show HEAD:tools/cold_label_check.py | /usr/bin/grep -n "10\|ten "` returns
nothing. So the design's cold-check row registers a change that exists and omits
the one that does not.

**FIX.** Delete row 6 and §4 (they belong to ledger step 2, which is landed);
replace them with a pointer. Add rows for `run`'s return type, the
`passes::capture` print site, the hit-rate line's format, `Mode::Capture`'s arity,
and the test file(s). If the ten-record floor is meant to be enforced by the
instrument rather than read off its output, that is a separate finding against the
landed `f1acc57` and belongs in its own report, not in a change row here.

## BLOCKING 3 — THE DESIGN DOES NOT SAY WHICH SIDE OF `capture::normalise` THE MEMO SITS ON, AND ONE OF THE TWO CHOICES WRITES NON-DETERMINISTIC BYTES

Priority 6. §1 row 2 is the entire specification of the stored value:

> | 2 | `crates/pistol-arena/src/capture.rs` | `LabelCache`, a two-state type: `Off`, or `On(BTreeMap<String, (String, String)>)` |

and row 1 says *"before `ask`, a lookup … after `ask`, an insert"*. The record is
built at `capture.rs:352-358`:

```
capture.rs:356                    totals: normalise(&totals)?,
```

`ask` returns the engine's **raw** totals line, carrying ` nps <n> time <n>`
(`capture.rs:273`, `return Ok((totals, line))`); `normalise` (`capture.rs:66-96`)
strips exactly those two fields and **returns `Err` if they are absent**
(`:71`, *"a totals line carries no `nps` field"*).

Three implementations satisfy the design's sentence, and they are not equivalent:

1. insert **raw**, and on a hit run `normalise` again → correct.
2. insert **normalised**, and on a hit run `normalise` again → a hard refusal at
   the first hit (`:71`), because the stored line has no `nps`. Loud, and it
   would be caught in seconds.
3. insert **raw**, and on a hit **skip** `normalise` (the natural reading of
   *"a hit returns the `(totals, bestmove)` … already produced"*, and the shape a
   `map.get(k).cloned()` early-return falls into) → **53% of records carry
   ` nps <n> time <n>` and are therefore machine- and load-dependent**. §4.4's
   `cmp -s` fails, and — worse — every *cached* capture is now non-reproducible
   against itself, so §4.4's registered diagnosis procedure
   (`wp21_throughput_prereg.md:690-698`, re-run the uncached pass twice) would
   correctly point at the cache and the lever would be abandoned for a
   one-line ordering slip.

The registration itself names this class — `wp21_throughput_prereg.md:618-621`
lists *"a wrong entry returned, records emitted out of order, an entry mutated
after insertion, **a hit that skips the `no_tab` guard**"* as candidate cache
defects. The design mentions `no_tab` **zero times**
(`/usr/bin/grep -c no_tab docs/experiments/wp21_label_cache_design.md` → `0`) and
`normalise` only as an aside in §2 about gate 9's field set.

**AND §5's MUTATION SET CANNOT SEE IT.** Its six mutants are: lookup removed,
lookup inverted, insert removed, insert mis-keyed, X1 negated, counters
removed/inverted. Reading §5's own kill criteria — *"dies at the hit-count test"*,
*"dies at the first record"*, *"dies at a two-seat fixture holding a
transposition"* — **none of them compares a HIT record's bytes with its MISS
record's bytes**. Implementation 3 leaves every one of them green.

**FIX.** §1 row 2 states the stored value **explicitly** — *"the `(totals,
bestmove)` `ask` returned, BEFORE `normalise`; the hit path builds its record
through the same `normalise(&totals)?` and `no_tab(&record)?` calls at
`capture.rs:356` and `:359` as the miss path, so the record-construction block is
shared and not duplicated"*. Add a mutant *"the memo stores the normalised
totals"* and *"the hit path returns the stored totals directly"*, each dying at a
new test that, over one capture, asserts every HIT record's `totals` and
`bestmove` are byte-equal to those of the MISS carrying the same `position`, and
that no record's `totals` contains ` nps `.

## BLOCKING 4 — TWO GUARDS INSIDE `ask` STOP RUNNING ON 53% OF PREFIXES, AND §3 CLAIMS ITS REFUSAL LIST IS COMPLETE WHILE §6 DISCLAIMS NOTHING ABOUT THEM

§3's heading: **"THE REFUSALS, EACH NAMED AND EACH WITH THE DEFECT IT EXCLUDES"**,
and its table has two rows. But a hit does not call `ask`, and `ask` carries two
guards the design never accounts for:

```
capture.rs:241-246   if let Some(stray) = channel.unsolicited() {
                         return Err(refuse(… "the engine spoke before it was asked" …))
capture.rs:288-295   if census.request == CensusRequest::Off {
                         return Err(refuse(… "the engine wrote a census row for a `{}` that did not ask for one" …))
```

`channel.unsolicited()` is a **per-ask** protocol check: it drains whatever the
engine said between one ask and the next. Under the cache it runs on misses only,
so an engine that emits a stray line while the pass is serving cached answers is
not caught at the position it happened at — it is caught at the **next miss**, and
`ask`'s refusal names `game {game}, turn {k}` from that later prefix
(`capture.rs:240`, `where_()`). The refusal is therefore attributed to the wrong
position, and if the stray output ends the run's last few prefixes it is not
caught at all. That is a change in what the pass detects and in what its error
message means, in a package whose entire premise (§ONE LINE) is *"It drops no
record, changes no field."*

The design does not mention it in §3 (which claims completeness), and §6 (*"WHAT
THIS DESIGN DOES NOT DO"*) does not disclaim it either — priority 8's question,
answered: §6 is incomplete, not untrue.

**FIX.** Either (a) keep the guard on the hit path — `channel.unsolicited()` is
cheap and has no relationship to whether a search ran — and say so in §1 as its
own row, which is the fix this reviewer recommends because it restores the
per-prefix attribution exactly; or (b) register it in §6 as a named consequence
with the residual stated (*"a stray line is attributed to the next MISS"*) and add
a mutant. Silence is not an option: the pass's own doc comment at
`capture.rs:323-325` says *"a capture that silently omits positions is a corpus
whose gaps are invisible to its consumer"*, and this is the same shape one level
down.

## BLOCKING 5 — THE DESIGN'S HEADLINE SOUNDNESS SENTENCE NAMES THE WRONG GATE, AND IT IS THE SAME WRONG GATE THE GOVERNING REGISTRATION CORRECTS BY NAME

The design's ONE LINE, lines 7-8:

> so the cache's soundness is the determinism law **CI gate 6** already enforces
> and is not a new claim about the search.

**Gate 6 is config validation.** `tools/ci.sh:97-98`:

```
step "gate 6/$GATE_TOTAL: config validation"
gate "config check" tools/config_check.sh
```

The governing registration corrects exactly this, in parentheses, at
`wp21_throughput_prereg.md:153-154`: *"**revision 2 said gate 6, which is config
validation**"*, and `arc3_ledger.md:230` records it as finding **F-1.5**, *"THE
DOCUMENT CITED THE WRONG GATE NUMBER"*. The design reproduces the retracted error
in the one sentence that carries its whole argument.

**AND §2's REPAIR OF IT IS ALSO WRONG, TWICE.** Design §2, lines 71-72:

> `tools/determinism.sh` is **CI gate 9 of 19** (`tools/ci.sh:104-105`)

- *of 19* is stale. `tools/ci.sh:21` is `readonly GATE_TOTAL=20`, and gate 20
  (`:202-203`, governing-document citations) landed at `9390b48`/`bf4d695` under
  **D-583**, which also records that CLAUDE.md's own `gate N/19` sentence was
  corrected. D-583 is required reading for this design.
- *`tools/ci.sh:104-105`* is a comment block (*"# It runs last of the two engine
  gates because it is the slowest…"*). The gate is at `:109-110`.

`tools/design_citation_check.py` cannot catch either: it matches
`` `path:line-line` `` and checks only that the range is inside the file
(`:48-52`, `:76`), so a citation pointing at the wrong lines of the right file is
green. This is precisely what D-582 says a mechanism cannot do and a reviewer can.

**FIX.** ONE LINE → *"CI gate 9"*. §2 → *"CI gate 9 of 20 (`tools/ci.sh:109-110`;
the total lives once, at `:21`)"*.

## BLOCKING 6 — THE DESIGN IS GOVERNED BY A SUPERSEDED REVISION, AND CITES SECTION NUMBERS THAT REVISION DID NOT HAVE

Design lines 11-13:

> **GOVERNING REGISTRATION**: `docs/experiments/wp21_throughput_prereg.md`
> **revision 2**, §2 (the key and why), §4.2 …

The tree holds **revision 3** (`wp21_throughput_prereg.md:1`, *"PRE-REGISTRATION,
revision 3"*), and revision 2 **failed its review** — 2 BLOCKING, 12 MAJOR, 8
minor (`:15-17`). CLAUDE.md's Process section: *"A pre-registration is reviewed at
the revision that GOVERNS the run … reviews of superseded revisions do not
transfer."* A design that names revision 2 as its governor names a document that
was refused.

It is not a typo with no consequence. Two of the sections this design most depends
on **do not exist in revision 2** and were created by revision 3's amendments:
**§2.0** (the lifetime invariant BLOCKING 1 turns on) and **§2.1**. Revision 3
also replaced §2's key argument outright (`:128-146`) and rewrote §4.1's count
under a new instrument (`:449-493`). The design's §2 in fact tracks **revision
3's** text — which makes the header's *"revision 2"* not just stale but
self-contradicting.

**FIX.** Header → *"revision 3, §2, §2.0, §2.1, §4.1, §4.2, §4.4, §4.5, §7"*, and
§1.1's counters cite `§2:216-222` rather than the matrix alone.

---

# MAJOR

## MAJOR 6 — `key_pos` IS NOT WHAT THE DESIGN SAYS IT IS, AND THE NAME IS THE TREE'S

§1.1:

> the counters additionally hold, for each miss, its transposition key (**the
> replayed position's sorted `(cell, player)` list**) and its symmetry key
> (`canonical_form` over the stones)

and it labels these `key_pos` and `key_full` throughout, as does D-581 and as does
`matrix_label_cache_key.md:249-253`. In this tree:

```
labels.rs:202        key_pos: state.key().to_string(),
labels_file.rs:27    /// `GameState::key`: the same position up to transposition.
labels_file.rs:245   "record {}: `key_pos` is not thirty-two hex digits",
```

`GameState::key` (`crates/pistol-core/src/state.rs:134`) returns a **`Key128`** —
a 128-bit zobrist, rendered as 32 hex digits. `key_pos` is a **digest**, not a
stone list. (`key_full` the design gets right: `labels.rs:203`,
`render_key_full(&canonical_form(&stones))`.)

Why this matters rather than being a wording slip:

1. **The design's own §2 rejects a 128-bit zobrist as a key** because *"a cache
   that answers from a collision returns a label for a position nobody played"*.
   Naming the collision-free stone list `key_pos` while the tree's `key_pos` **is**
   that zobrist means the two documents disagree about which object the flip
   clause's number describes.
2. D-581's flip clause and D-562(2)'s open question are both stated over the
   corpus columns. If IMPL builds a set of sorted stone lists, the counter reports
   the *idealised* fold; if it builds `GameState::key`, it reports the corpus
   column's fold. A REVIEW-impl checking the counter against D-581 has no way to
   pick.

**FIX.** §1.1 names the two counters with the definitions it actually intends and
says explicitly whether they are the corpus columns or idealised twins — e.g.
*"`n_stoneset` (the sorted `(cell, player)` list, which is `key_pos`'s equivalence
without its digest) and `n_symmetry` (`canonical_form` over the stones, which is
`key_full`'s)"* — and records that a stone-list counter can only over-report
relative to `key_pos`, never under-report, which is the safe direction.

## MAJOR 7 — THE FLIP CLAUSE IS MISQUOTED (ONE CONJUNCT DROPPED), AND ONE OF THE TWO COUNTERS' ANSWERS IS ALREADY KNOWN

Priority 3. §1.1:

> **That number is the coarser keys' yield at the scale that matters**, it is
> what **re-takes this matrix if it is materially above zero**, and it settles
> D-562(2)'s open three-key question in the same pass.

**(a) THE SECOND CONJUNCT IS GONE.** D-581's last sentence: *"Flips: the selection
flips only if that count is materially above zero **AND a seat pins `countermove`
false as a rule rather than as a value**; either alone buys nothing."*
`matrix_label_cache_key.md:253-256` says the same. The design registers the
disjunct-free half, which is a strictly weaker trigger than the ruling's — and
the design is what IMPL and the run log will be written against.

**(b) THE `key_full` COUNTER'S ANSWER IS ALREADY MEASURED, AND IT IS ABOVE ZERO.**
`wp21_throughput_prereg.md:205-211` and `matrix_label_cache_key.md:133-140`, both
from `tools/opening_prefix_fold.py` (receipt
`artifacts/arc3_opening_prefix_fold.txt`): over tranche one's own 218 openings,
`k = 2` gives **213 exact-key classes against 171 symmetry classes — 42 extra
merges**, and `wp21_throughput_prereg_rev3_REVIEW.md:262-264` reports **42 to 60
per tranche, 792 across the sweep, from `k = 2` alone**. So the `key_full` counter
is guaranteed to print a number ≥ 42 on tranche one before a single prefix past
the book is counted.

`docs/process.md` (§*Cost, replication, and the second instrument*, closing
sentence): *"neither catches a run whose answer is already known before it is
taken — that defect is judged, not checked."* The rev-3 review raised this as
BLOCKING; the design inherits it unaddressed. A run log reading *"key_full
collisions: 48"* will be read as a discovery when 42 of it is the book's shape,
already in the tree, derivable in about two seconds.

**FIX.** §1.1 (i) restores D-581's `AND`; (ii) records the **known floor** —
*"tranche one's `key_full` counter is expected to read at least 42 from `k = 2`
alone (MEASURED, `tools/opening_prefix_fold.py`); what the counter adds is the
`k > opening_turns` remainder, which nothing has measured"*; (iii) states what
*"materially above zero"* means against that floor, before the run, or the phrase
constrains nothing (D-424).

## MAJOR 8 — "IT SETTLES D-562(2)'s OPEN THREE-KEY QUESTION" IS FALSE, AND THE GOVERNING REGISTRATION SAYS SO FOUR LINES AFTER SAYING THE OPPOSITE

D-562(2)'s open question, in its own words (`docs/decisions.md:1192`):

> the pilot's three keys AGREED … so *three-key agreement* has never been read
> against a corpus where they disagree … **which key rules a disagreement** is
> WP-2.0b's transposition question and not this line's.

The question is a **corpus-assembly dedup policy** — *which* key wins when two
disagree. The counters measure **how many** misses collide under a coarser key.
Counting collisions cannot select a tie-break rule; at best it establishes that
disagreements exist at all, which is a precondition for answering the question and
not an answer.

The registration says both things. `wp21_throughput_prereg.md:222-223` asserts the
design's claim; `:235-240` denies it:

> **D-562(2)'s unsettled three-key question is therefore not merely untouched but
> **unreachable from here**** … *Which key rules a disagreement* stays WP-2.0b's
> transposition question, as D-562(2) says.

That is a defect in the registration (a claim it makes twice, in two directions —
D-423's own failure mode). The design picks up the false half and repeats it
without noticing the contradiction, which is what a design review is for.

**FIX.** Delete the clause from §1.1 and replace it with the true, smaller claim:
*"it produces the first evidence in this project that the three keys can disagree
at all; which key rules a disagreement stays D-562(2)'s open question."* Raise the
contradiction against `wp21_throughput_prereg.md` §2 so revision 4 states it once.

## MAJOR 9 — §2 RE-ASSERTS THE PLAY-ORDER CLAIM D-581, THE MATRIX AND THE REGISTRATION ALL RETRACTED, AND ITS NUMBER IS WRONG UNDER EVERY READING

Design §2, lines 85-88:

> **No claim about what the search reads out of `GameState` is made, and revision 1
> of the registration made one that is false** (the search reads `played()`
> **under two of the three ordering gates**: `heuristics.rs:155`, `:89`).

Traced at HEAD:

```
heuristics.rs:89    for played in state.played() {          ← inside record_cutoff (fn at :70)
pvs.rs:491-494      if let CandidatePolicy::Staged(params) = self.policy
                        && params.ordering.any()             ← the call-site gate
params.rs:114-116   pub const fn any(self) -> bool { self.killers || self.history || self.countermove }
heuristics.rs:153-155  if gates.countermove
                        && state.phase() == Phase::First
                        && let Some(at) = last_stone(state)
```

So:

- **as a claim about READS**, `:89` runs under `any()` — i.e. under **all three**
  gates, not two;
- **as a claim about what can MOVE AN ANSWER**, it is **one** gate. D-581: *"SO
  THE SINGLE PATH FROM A ROOT'S PLAY ORDER TO A SEARCH CHOICE IS `countermove`"*;
  `wp21_throughput_prereg.md:136-146`: *"**THE OTHER TWO GATES DO NOT REACH IT,
  and an earlier draft of this paragraph said they did**"*;
  `matrix_label_cache_key.md:178-187`: *"revision 1 said all three ordering gates,
  which is **three times wider than the code makes it**"*.

*"Two of the three"* is right under neither reading. `arc3_ledger.md:425` records
this exact class as **F-1.8**, *"THE PLAY-ORDER CLAIM WAS THREE TIMES WIDER THAN
THE CODE"*. The design is the fourth document in this arc to get the same sentence
wrong, in a parenthesis whose job is to explain why an earlier version of it was
wrong.

**FIX.** Replace the parenthesis with: *"(the single path from a root's play order
to a search choice is `countermove` — `heuristics.rs:191-193` read at `:155`;
`:89`'s walk is gated by `params.ordering.any()` at `pvs.rs:491-494` and feeds
only `pair_killers`, written at `Phase::Second` nodes, and every capture root is
`Phase::First` — D-581)."*

## MAJOR 10 — THE COUNTERS' COST CLAIM IS CREDIBLE, ITS REGISTERED MEASUREMENT CANNOT TEST IT, AND A CHEAPER FORMULATION IS NOT COSTED

Priority 3. §1.1:

> **THE COST IS ONE REPLAY PER MISS**, which the cache does not otherwise need —
> at most eighty placements and twelve symmetry images against a search of
> ~885 ms, paid on the 47% of asks that miss. **ESTIMATED at well under 0.1% of a
> capture**; it is **MEASURED on the dry run** and the number goes in the run log.

**The magnitude is credible and I am not disputing it.** `turn_cap = 40`
(`wp21_prereg.md:149`) gives at most `1 + 39×2 = 79` placements, so *"at most
eighty"* is a true bound; `canonical_form` (`symmetry.rs:166-180`) is twelve
transforms over ≤79 pairs plus a compare; the marginal cost lands on a prefix that
is already paying 0.8854 s of search
(`wp21_throughput_prereg.md:424`, MEASURED). Microseconds against ~885 ms is well
under 0.1%.

**Three things are wrong around it.**

**(a) NOTHING CAN MEASURE IT.** *"It is MEASURED on the dry run"* — but the
registered dry run (`wp21_throughput_prereg.md:757-772`) runs at
`--label-nodes 2000` rather than 400 000, i.e. a search **two hundred times
cheaper**, so the counter overhead's *fraction of a capture* there is inflated by
roughly that factor and can neither confirm nor refute a claim about the sweep's
budget. More basically: the counters are **unconditional** in this design (no
switch), so there is no cached-without-counters referent to subtract, and the run
log has nothing to print. A registered MEASURED number with no instrument is
`docs/process.md`'s *"criterion nothing can fail"* one level down.

**(b) THE CHEAPER FORMULATION EXISTS AND IS NOT NAMED.** The design's *"one replay
per miss"* is quadratic in game length: `run`'s prefix loop
(`capture.rs:341`) walks `k = 0..n` and a per-miss replay costs `O(k)`, so
`O(n²)` per game. The stone set at prefix `k` is available incrementally — turn
`i`'s placements belong to player `i mod 2`, and `asked_prefixes`
(`capture.rs:28-44`) has already proved the whole move list legal, so no
`make_turn` is needed for a stone set. Maintaining one running
`Vec<(Coord, Player)>` across the prefix loop makes the transposition key `O(1)`
amortised and leaves only `canonical_form`'s twelve images per miss. That is a
~20x reduction on a cost that is already negligible — so it is not a correctness
finding — but a design that names its cost owes the cheaper shape it declined, and
D-291's spirit is that an estimate reachable by a moment's thought is a finding.

**(c) THE GOVERNING MATRIX SAYS THE OPPOSITE AND IS NOT AMENDED.**
`matrix_label_cache_key.md:249-252` registers the counters as *"Two counters, **no
extra search, one sort the arena already does**"*. There is no such sort in
`capture::run`: the pass never builds a `GameState` at all after
`asked_prefixes` discards its own, and the stones/`canonical_form` work belongs to
`labels.rs` (`:196-203`), a different pass over a different file. The design is
**right** and the matrix is wrong — and the matrix is on CI gate 20's governing
list (`tools/governing_citation_check.sh:51`). A design that silently contradicts
a governing document is the silent-drift breach CLAUDE.md names; the matrix must be
amended (which reopens its review) or the design must record the disagreement.

**Do the counters risk changing an answer?** Structurally no, and §1.1's argument
for that is sound: they write only to counters and the cache still keys on
`position` alone. One residual the design should close: the per-miss replay's
**error path**. If IMPL replays through `GameState::make_turn`, that returns a
`Result`, and a `?` there converts a counter's bug into a refused capture —
turning "cannot mislabel a record, only misreport a number" into false advertising.
`asked_prefixes` (`capture.rs:32-39`) has already proved every turn legal, so the
design should say the replay's failure is `unreachable!` and not propagated.

## MAJOR 11 — THE MUTATION SET OMITS X2 ENTIRELY, HAS NO CALL-REMOVED MUTANT (D-553), AND ITS X1 MUTANT IS THE ONE THAT WOULD SURVIVE

Priority 5. §5's six bullets, against three standing obligations.

**(a) X2 HAS NO MUTANT.** §3 registers X2 (*"a hit whose cached `bestmove` or
`totals` is empty is refused naming the position"*) and §5's list has no X2 line.
D-553 is *"for every guard or invariant"*.

**And X2 is the wrong guard anyway.** `ask` cannot return an empty string: it
returns only on a line starting with `bestmove ` (`capture.rs:266-273`) with a
`totals` that `exchange::totals_of` matched (`:190`, `:276-278`). So X2 fires
only under a mutation of the memo itself — which is legitimate as a mutation
target but means §3's *"the defect it excludes"* column is empty in practice, and
D-572's own lesson applies: *"a refusal that cannot fire is not a guard"*.
Meanwhile the memo defect that **can** happen — a hit returning **the wrong
non-empty entry** — is the one X2 cannot see, and §5's mis-keyed-insert mutant is
the only thing aimed at it.

**(b) NO CALL-REMOVED MUTANT ANYWHERE.** D-553 (`docs/decisions.md:1174`),
standing law: *"for every guard or invariant, the mutation set includes a
call-REMOVED mutant, and it must die at a test that **drives the call site with
reachable input** — a test invoking the guarded function directly pins the
function and leaves 'the call was never made' alive."* §5 has *"the lookup:
removed"* and *"the insert: removed"* (which are call-removals of the memo
operations, so those are fine) but **no call-removed mutant for X1, X2, `no_tab`
on the hit path, or the flag's threading**. The last is the motivating class
itself: **`--label-cache` parsed in `bin/arena.rs` and dropped on the floor in
`passes::capture`**. §5's lookup/insert mutants *"die at the hit-count test"* only
if that test drives the shipped **binary** with the word `--label-cache`; a test
calling `capture::run(&transcript, nodes, &mut sink, LabelCache::On(...))` directly
leaves the threading mutant alive, and §5 does not say which driver is used.

**(c) THE X1 MUTANT AS WRITTEN IS THE WEAK ONE, AND ITS STRONGER SIBLING SURVIVES
THE TEST §5 REGISTERS.** §5 has *"X1: the refusal's condition negated — dies at
its own test"*. But under §1 row 4's shape, X1 is a **match arm** in
`bin/arena.rs`'s positional `match words` (`:39-86`). Delete that arm — the
call-removed mutant — and the combination falls through to the catch-all at
`:79-85`, which **still refuses and still exits 2**:

```
bin/arena.rs:80-84   return Err(format!(
                       "--config and --out are both required, or --replay, --out and --workers, or \
                        --capture, --out and --label-nodes, or --labels, --report and --out, each in \
                        that order\n\n{USAGE}"))
```

So a test that asserts only *"exit 2 and no output file"* stays green with X1
deleted. It dies **only** if the test asserts that the message **names both
words** — which is what §3 promises (*"refused before any game, **naming both
words**"*) and what §5's kill criterion does not say. **This is the defect class
the dispatch asked me to name: X1's call-removal, killed by nothing §5
registers.**

**(d) AND "NAMING BOTH WORDS" IS NOT ACHIEVABLE BY THE SHAPE §1 REGISTERS.**
`bin/arena.rs:51-74` matches word slices **positionally**. With `--label-cache`
added as a trailing word beside the existing `--census`, the capture mode needs
**five** arms — `[…nodes]`, `[…nodes, "--census"]`, `[…nodes, "--label-cache"]`,
and the two refusal orders `[…, "--census", "--label-cache"]` and
`[…, "--label-cache", "--census"]`. §1 row 4 names *"one optional trailing word"*
and §3 names one refusal. If only one refusal arm is written, the **other**
spelling falls to the catch-all and is refused with *"each in that order"* — a
message that names neither word and actively misleads by suggesting a reorder
would work.

**FIX.** §5 gains: an X2 mutant; a call-removed mutant per guard, each with the
**driver** named (the shipped `arena` binary, per D-553); an X1 arm-removed mutant
whose kill criterion is *"stderr contains both `--label-cache` and `--census`"*;
the normalise-side mutants from BLOCKING 3; and a flag-threading mutant
(`passes::capture` ignores its cache argument) killed by a binary-driven hit-count
test. §1 row 4 enumerates all five arms. The `git grep` receipt at IMPL is
correctly deferred — that part of §5 is right.

## MAJOR 12 — HARD RULE 9: THE PACKAGE FALSIFIES `capture.rs`'s OWN JUSTIFICATION AND THE DESIGN IS SILENT

`wc -l < crates/pistol-arena/src/capture.rs` → **389**;
`tools/file_justification_check.sh:65` → `SOFT_CAP=300`. The file is already over
and already justified, at `docs/rule9_justifications.md:82`:

> `crates/pistol-arena/src/capture.rs`: **one pass, and the census sink is not a
> second subject.** Every line here is the capture ask — what is sent, what the
> engine may say back, and what each of those answers means to the pass …

This design adds to that file a `LabelCache` type, a memo, a hit counter, a
per-miss position replay, a twelve-image `canonical_form` fold and two collision
counters. The transposition/symmetry fold is `labels.rs`'s subject
(`labels.rs:196-203`, and `rule9_justifications.md:27` says so: *"the replay that
supplies `to_move` is the replay that supplies both position keys and the stones
the third is folded from"*). The counters are a **study instrument**, which is
plainly a second subject by the justification's own test. The design mentions
rule 9 zero times
(`/usr/bin/grep -c "rule 9" docs/experiments/wp21_label_cache_design.md` → `0`).

**FIX.** §1 states the placement decision and its rule-9 consequence: either the
memo and counters go in a new `crates/pistol-arena/src/label_cache.rs` (PROPOSED; no such file exists) (which is
this reviewer's recommendation — it also makes BLOCKING 1's construction site a
single readable function and gives the mutation receipt a clean `git grep`
scope), or `docs/rule9_justifications.md:82` is amended and the amendment argues
why a cache and a symmetry fold are the same subject as the ask. Silence is the
drift.

## MAJOR 13 — HARD RULE 1: THE `--census` PRECEDENT HOLDS FOR THE RULE AND FAILS FOR THE PROPERTY THAT MATTERS HERE

Priority 7. §4.2 of the registration leans on *"`--census`'s own precedent at
`crates/pistol-arena/src/bin/arena.rs:57-72`: an optional trailing word, absent
meaning off, named in `crates/pistol-arena/src/usage.rs` which is the one place
the default lives"*, and the design's §1 row 4/5 inherit it.

**On hard rule 1 itself the precedent is genuine and I am not raising a finding.**
`usage.rs:14` spells `[--census]`; `:56-64` documents *"LAST, and optional: a
capture without it is the capture this pipeline has always taken"*; the same
review-round question was already put and rejected
(`wp21_throughput_prereg_rev3_REVIEW.md:1001-1011`, A11). A mode selector is not a
tunable, and `--workers`/`--label-nodes` — the actual tunables — are required with
no default, which is rule 1 working.

**But the precedent breaks on the one property this package depends on.**
`--census` changes `label_go_line` (`capture.rs:124-132`), which is digested into
`capture_sha256` (`:104-110`), and `capture.rs:119-123` says so in as many words:

> A census-on capture therefore has a **different `capture_sha256`** from the
> otherwise identical census-off one … it is said out loud because a tranche
> registered against one digest is not a tranche run under the other.

`--label-cache` is the exact opposite by design — §1.1: *"`capture_sha256` is
untouched"*. **So a census capture tells you which instrument it is, and a cached
capture cannot.** Nothing in the artifact, the header, the manifest row or the
corpus records whether the word was typed. That is not a rule-1 breach, but it is
load-bearing, because `wp21_throughput_prereg.md:594-600` makes the whole
cached/uncached audit rest on it:

> **the `cmp -s` exit line and its timestamp appear in the run log BEFORE the
> first tranche block whose command carries `--label-cache`** … A tranche block
> carrying the flag with no `cmp -s` line above it is a VOID tranche.

The sole evidence for "this tranche was cached" is a **hand-transcribed command
line in a prose run log**, in a project whose named recurring defect is a claim
transcribed where it was convenient. The design cites the precedent and does not
say that the precedent's own auditability half does not come with it.

**FIX.** §1 records the consequence explicitly (*"unlike `--census`, the cache
leaves no trace in any artifact; the run log's command block is the only
record"*), and either (a) accepts it with the residual stated, or (b) prints the
hit rate and the cache state on `arena`'s stdout beside the manifest row — which
the package needs a print site for anyway (BLOCKING 2a) — so the run log's
transcription is corroborated by the process's own output rather than only by the
typist.

## MAJOR 14 — X1 IS THE RIGHT SHAPE AND IS ENFORCED IN THE WRONG PLACE: §1's OWN SIGNATURE MAKES THE FORBIDDEN STATE REPRESENTABLE AT THE SEAM WHERE THE DEFECT LIVES

Priority 2, second half. On the shape: **refusal is correct** and I attacked the
alternatives without finding a better one (see *ATTACKS I ATTEMPTED AND
REJECTED*). On placement it is wrong.

§3: *"**X1 IS THE ONE THAT MATTERS AND IT IS A CONFIG-TIME REFUSAL**, not a
runtime one: it fires from the argument parse, before `--out` is claimed."* The
*ordering* claim is **TRUE** — verified below in *WHAT SURVIVED*. But the defect
X1 excludes — *"a cache hit performs no search and so emits no census row"* — is a
property of **`capture::run`**, which is `pub` (`capture.rs:326`) and which §1
row 1 gives **both** a `&mut CensusSink` (carrying `request: CensusRequest`) and a
`&mut LabelCache`. The illegal combination is representable at exactly the seam
where it does damage, and the only thing forbidding it lives in a different crate
(`bin/arena.rs` is a separate crate from the `pistol-arena` library —
`usage.rs:6-7` says so). Every test, every future caller and every later pass can
construct it.

`docs/rule9_justifications.md:82` names this shape as the reason the census sink
lives in `capture.rs` at all: *"Splitting the row-keeping from the classifier that
recognises a row would put a guard's decision in one file and its consequence in
another, which is exactly the shape that let a token be honoured by the engine and
its rows dropped downstream at exit 0."* X1 as designed is that shape again.

**FIX.** Keep the CLI refusal (it is what gives D-200's no-file-left-behind
property) **and** add the same refusal as the first statement of `capture::run` —
`if census.request == CensusRequest::On && cache.is_on() { return Err(refuse(…)) }`
— naming both, before `one_engine`. One comparison, before any game, with its own
call-removed mutant. Better still, make it unrepresentable: `run` takes one
closed enum — `Mode::Plain`, `Mode::Census(sink)`, `Mode::Cached(mode)` — and the
combination cannot be spelled at all, which is what hard rule 3's *"fail loud"*
prefers to a guard.

## MAJOR 15 — §6.3 IS MISSING, AND §6's THREE DISCLAIMERS ARE TRUE BUT NOT COMPLETE

Priority 8, answered in full. The three §6 items are each **TRUE**:

1. *"does not persist across processes"* — consistent with D-572 and with
   `matrix_label_cache_key.md:293-297`, which costs the persistent option and
   dismisses it on the same ground. ✓
2. *"does not fold transpositions or symmetries. MEASURED at zero yield on the
   pilot corpus"* — the pilot's 347/347/347 is in
   `artifacts/arc3_leverB_41_count_v3.txt` and I reproduced 742/347
   independently (R1, R2). ✓ **But the sentence is now half-retired**: D-583 and
   `matrix_label_cache_key.md:133-166` measure the symmetry fold at **42 merges
   per tranche** on the sweep's own openings, which is not zero. *"Zero yield on
   the pilot corpus"* is literally true and reads as more than it is; the design
   should carry the tranche number beside it, since §1.1's whole counter argument
   is that the pilot number does not transfer.
3. *"changes no criterion of `wp21_prereg.md` §4"* — ✓, T-A's amendment is R1's
   (`wp21_prereg.md:54`, `:302-303`).

**WHAT IS DISCLAIMED THAT THE REGISTRATION REQUIRES: nothing.** **WHAT THE
REGISTRATION REQUIRES AND §6 NEITHER DOES NOR DISCLAIMS: three things** —
BLOCKING 4's two skipped guards; the `no_tab` question (BLOCKING 3); and the
consequence the registration goes out of its way to register at
`wp21_throughput_prereg.md:551-556`:

> under the cache, **53% of corpus records carry `search_nodes` for a search that
> was not run** … a consumer summing `search_nodes` as machine work would
> over-count by the duplication factor.

That is exactly the kind of fact §6 exists to hold, it is registered in the
document the design says governs it, and the design does not repeat or point at
it.

## MAJOR 16 — THE COLD-CHECK SECTION (§4) IS SOUND ARGUMENT IN THE WRONG DOCUMENT

§4's substantive claim is **correct** and worth keeping somewhere:

> **AND ON A HIT THE CHECK IS A CHECK OF THE MEMO, NOT OF THE KEY.** … if the
> memo returned the entry belonging to some other position, the record carries
> that position's answer and the fresh ask does not.

It matches `matrix_label_cache_key.md:262-278`'s rejection of the red team's
MAJOR 2(b) and it is a real property of `cold_label_check.py:186` (the partition
is over the record's own `position` field). But per BLOCKING 2(d) the whole
section is outside the design's declared scope, and its instrument shipped at
`f1acc57` without this design. Keeping §4 here means the same argument now lives
in three documents (`wp21_prereg.md` §4, `cold_label_check.py`'s header,
`matrix_label_cache_key.md` §5) — D-423's failure mode at four.

**FIX.** Reduce §4 to one sentence and a pointer to `wp21_prereg.md:302-348`,
which owns it.

## MAJOR 17 — THE DESIGN GOVERNS AN IMPLEMENTATION AND IS ON NO CITATION GATE

`tools/governing_citation_check.sh:46-54` lists eight documents; this one is not
among them, though it is the document IMPL and REVIEW-impl will be written
against. D-583's own criterion for the list is *"documents that GOVERN a run or a
decision now, where a stale citation misleads rather than remembers"* — which
this is. Had it been on the list, BLOCKING 5's `tools/ci.sh:104-105` would have
been caught by the machine instead of costing a reviewer a paragraph; that half
of the gate (*rot*, not scope) is exactly what it is for.

**FIX.** Add `docs/experiments/wp21_label_cache_design.md` to `GOVERNING` when it
passes, and update `crates/pistol-arena/tests/governing_citation_check_tests.rs`
(which pins the list at `:45`). Remove it when the package lands and it becomes a
record.

---

# minor

**minor 18.** §2: *"**A cache hit is exactly 'the same position, asked again, in
the same process', and that is the one shape gate 9 never takes**."* Not quite.
`tools/determinism.sh:221-227` builds the A/B session as
`for budget in BUDGETS: for position in positions:`, so run A **does** ask every
position twice in one process — at `depth_turns 4` and at `nodes 200000`. What
gate 9 never takes is the same position twice **at the same `go`**. The error is
in the conservative direction (it understates the gate's coverage and so
overstates §4.4's burden), and D-581 carries the same phrasing, so it changes no
conclusion. State it as *"the same position at the same `go`"*.

**minor 19.** §2 and §1.1 both price the memo lookup *"against a search of
~885 ms"*. On a **hit** there is no search; the honest framing is that the string
compare replaces the 885 ms. Trivial either way, and the conclusion is unchanged.

**minor 20.** §2 *"neither of its budgets is `nodes 400000`"* — `determinism.sh`
has four budgets in play, not two: `BUDGETS=("depth_turns 4" "nodes 200000")`
(`:85`) plus the solver seat's own `depth_turns-2 nodes-10000` (`:76`). None is
400 000, so the claim survives; *"none of its budgets"* is the accurate word.

**minor 21.** The memory cost of the counters is not stated. Estimated at ~12 MB
per tranche (5 819 misses × two stone lists of ≤79 pairs) against a 46 GB box and
at most 16 concurrent tranches — negligible, which is why this is minor and not a
finding, but §1.1 costs time and not space and the sweep's other memory argument
(D-572's ~26 GB) shows why space is worth a line here.

---

# WHAT SURVIVED ATTACK

1. **X1's timing claim, verified against the binary.** *"it fires from the
   argument parse, **before `--out` is claimed**, so the run leaves no file
   behind (D-200's shape)."* `bin/arena.rs:39-86` is the `match words`; the claim
   is at `:89`, `let claimed = outpath::claim(&out_path)?`. Every arm of the match,
   including the catch-all, returns before that line. **TRUE**, and it is the
   right property: `usage.rs:20-26` and `outpath.rs:27-41` are the same discipline.

2. **The key's soundness reduction, and its honesty about what the gate does not
   cover.** §2's *"the gate never takes the cache's own shape, which is said here
   because it is the package's strongest attack"* is the correct posture, matches
   D-581's *"THAT IS THE STRONGEST SURVIVING ATTACK AND IT IS RECORDED AS A
   COST"*, and correctly lands the whole verification burden on §4.4 rather than
   on an argument. `determinism.sh:154`'s `normalize()` really is
   `capture::normalise`'s rule (`capture.rs:66-96`), and none of the five seats
   (`determinism.sh:67-81`) is `configs/instrument_v0.toml`.

3. **The refusal to add a "cache disagrees" self-check.** §3: *"A cache that
   verified its own hits by re-asking would cost exactly what it saves … an
   internal self-check would be the vacuous-criterion class this project has
   already paid for six times."* Correct, and it is `docs/process.md`
   §*Criterion and defect class* applied properly: internal agreement between two
   components sharing an input is the shape that passes vacuously.

4. **`BTreeMap` and hard rule 4.** *"owes nothing to hard rule 4's hash-order
   clause because it has no hasher"* — true, and it correctly declines the wrong
   reason revision 2 gave (`wp21_throughput_prereg.md:544-549`).

5. **Record ORDER and the `position` field's provenance.** I attacked both and
   found nothing. `run` pushes in loop order (`capture.rs:360`) and no memo shape
   in §1 touches that; the key **is** the string the record carries
   (`capture.rs:342` builds it, `:354` moves it in), so a hit's `position` is
   byte-identical to its miss's by construction, which is also what makes T-A2's
   external referent bite (`cold_label_check.py:181-186`).

6. **The counters cannot change an answer**, subject to MAJOR 10(c)'s replay
   error path. They are write-only into counters and the cache still keys on
   `position` alone; `canonical_form(&[])` returns the empty list rather than
   panicking (`symmetry.rs:166-180`), so the `k = 0` prefix is safe.

7. **"At most eighty placements"** — `turn_cap = 40` (`wp21_prereg.md:149`) gives
   ≤79. True bound.

8. **742 / 347 REPRODUCED** by two of my own commands over the pilot capture
   (R1, R2), neither of them the document's `tools/label_cache_count.py`.

9. **§5's deferral of the `git grep` receipt to IMPL.** *"THE SITES DO NOT EXIST
   YET, so the receipt is owed AT IMPL and this section is not it."* Correct
   reading of D-568/D-553 — verified that no `LabelCache` or `--label-cache`
   exists anywhere (R9). The deferral is right; the set's contents are not
   (MAJOR 11).

---

# ATTACKS I ATTEMPTED AND REJECTED

1. **"`Searcher::census_folds` is reset by `clear`, so the registration's §2
   exception list is wrong and the design inherits it."** REJECTED. The
   `self.census_folds = 0` at `search.rs:214` is inside
   `collect_trigger_census` (fn at `:210`), not `clear`. The registration's claim
   at `wp21_throughput_prereg.md:117-122` holds, and the design's silence on it
   is correct because X1 forbids the combination.

2. **"A capture root could be `Phase::Second`, so `pair_killers` is a second
   play-order path."** REJECTED, as the rev-3 review's A9 rejected it.
   `capture.rs:342` builds every root from `&game.moves[..k]` — whole turns only.

3. **"Refusing `--census` is the wrong shape; the cache should re-ask on a hit
   when a census is armed, or mark the census partial."** REJECTED. Re-asking
   costs exactly what the cache saves (§3's own argument, and it is right); a
   partial-census marker is a `CENSUS_FORMAT_VERSION` bump (D-572) for a
   combination this sweep does not run (`wp21_prereg.md` preamble, census OFF).
   Refusal is the right shape. Only its *placement* is a finding (MAJOR 14).

4. **"The `position` String is moved into the record at `capture.rs:354`, so the
   memo forces a clone and the design hides a cost."** REJECTED as a finding. A
   clone of a ≤~400-byte string against an 885 ms search is nothing, and no byte
   of output moves.

5. **"`--label-cache` is a code-side default for a tunable and breaches hard
   rule 1."** REJECTED on the rule, as `wp21_throughput_prereg_rev3_REVIEW.md`'s
   A11 rejected it: `usage.rs:14`'s `[--census]` is the tree's convention for a
   capture-mode switch and a mode selector is not a tunable. What survives from
   the attack is a different property of the same precedent — MAJOR 13.

6. **"The counters' memory could bind at 16 concurrent tranches."** REJECTED —
   ~12 MB per tranche estimated, ~200 MB across the sweep, against the 46 GB box
   D-572's own arithmetic uses. Recorded as minor 21 only because the design
   costs time and not space.

7. **"`design_citation_check.py` would have caught the gate citations, so
   BLOCKING 5 is already mechanized."** REJECTED, and it is the reverse: I ran
   it (R11) and it returns `0 unreproduced` on this document, because `:48-52`
   checks only that a cited line range lies inside the file. The wrong-lines
   citation is green. That is D-582's point restated at this document.
