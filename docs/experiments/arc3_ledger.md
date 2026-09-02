# Arc III — the ledger. A successor continues from THIS, never from memory.

**HOW TO READ IT.** One section per dispatch section, appended to as it runs. A
step is CLOSED only when its row says so and names the receipt. Anything not
written here did not happen. The governing dispatch is the arc III
GROUNDWORK dispatch: consolidate -> sweep -> census -> anchor v3.

**THE PRIOR ARC'S ACCOUNT** is `docs/experiments/overnight2_RESUMED_SUMMARY.md`
and its ledger `docs/experiments/overnight2_ledger.md`; arc II's §2/§3/§4 rows
are the Phase 2-4 text this dispatch says is still binding.

---

## §0 — consolidation. IN PROGRESS.

| step | state | receipt |
|---|---|---|
| `overnight2-stopped` head committed (the resumed summary) | done, `2b94f04` | `git log` |
| pre-merge CI at `2b94f04` | RUNNING | `artifacts/arc3_ci_premerge_2b94f04.txt` |
| merge to `dev`, tag, ancestry check, branch delete | not started | — |
| audit files copied, three digests verified BEFORE commit | not started | — |
| audit D-line | not started | — |
| §R rulings block | not started | — |
| CI green on `dev` after all three | not started | — |

### FINDINGS RAISED IN §0, BEFORE ANY OF IT LANDED

**F-0.1 — the dispatch's `D-570` is taken, and the text it calls "verbatim" is
not in any tree.** §0.2 says *"append D-570 verbatim as the audit proposed"*.
`docs/decisions.md` already carries **D-570** (the census identity, landed by the
prior arc), and `/usr/bin/grep -n "D-57\|ADR line\|proposed" docs/audit/*.md` in
the audit worktree returns **nothing** — the three audit deliverables propose no
D-line text at all. So there is no verbatim to append and no free D-570 to append
it at. **ARCHITECT DEFAULT APPLIED**: the audit lands with a D-line written here,
at the next free number, marked as this session's words rather than the audit's.

**F-0.2 — the arc II resume dispatch is not transcribed anywhere in the tree.**
This dispatch says it is *"still binding for Phases 2-4"*. `/usr/bin/grep -rln
"quiet box\|50 registered openings"` over `docs/` returns only documents that
QUOTE it. What is recoverable is what the prior arc's ledger and registrations
quote: `wp21_prereg.md` for Phase 2's seat and criteria, `overnight2_ledger.md`
§3 and §4 for calibration and census, `sealbot_anchor_v3_prereg.md` for the
anchor's report shape. **ARCHITECT DEFAULT APPLIED**: those documents are read as
the binding text, and where this dispatch and they disagree this dispatch wins.

**F-0.3 — R1 names an identity that admits a wrong answer, and a finer one is
already registered.** R1: *"the sweep caches labels by the WP-2.0b canonical
identity"*. D-570's canonical identity folds **symmetries as well as
transpositions**. A cache keyed on it returns, for a position, the `bestmove` and
`totals` computed for a SYMMETRY IMAGE of that position — a bestmove in the wrong
frame, and node counts that D-137 says are not symmetry-invariant, because the
search's lexicographic tie-break is not. `wp21_throughput_prereg.md` §2 already
registers the sound key: the replayed position's sorted `(cell, player)` list,
which folds transpositions and nothing else. **AND THE EXTRA FOLD BUYS A MEASURED
ZERO**: WP-2.0b §9 took 798 in-tree firings on two committed fixtures at both
caps and found distinct `key` equal to distinct `key_pos` in every cell.
**ARCHITECT DEFAULT APPLIED**: the cache is keyed on §2's transposition-only key,
R1's approval of the cache is taken in full, and the naming difference is
recorded here and in the D-line rather than resolved silently. §4.4's
byte-identity criterion would catch the other reading; it should not have to.

---

## §1 — WP-2.1 pre-run reviews. IN PROGRESS.

| step | state | receipt |
|---|---|---|
| lever B §4.1's count, taken UNGOVERNED | done, and **re-take owed** | `artifacts/arc3_leverB_41_count.txt` |
| `wp21_throughput_prereg.md` amended for R1 | not started | — |
| `wp21_prereg.md` amended for R1 and re-derived | not started | — |
| fresh-context review, throughput prereg | not started | — |
| fresh-context review, sweep prereg | not started | — |
| lever A, the concurrency study (~1.1 h) | not started — **runs only after its prereg's review** | — |
| the label cache package | not started | — |

### THE §4.1 COUNT, AND WHAT IT MEASURED

`wp21_throughput_prereg.md` §4.1 registers, before any code: *"Over the pilot's
own report, count asked prefixes and distinct replayed positions under §2's exact
key. … If the ratio is below 1.5, lever B is dropped and no code is written."*

Taken over the pilot's own committed corpus
(`corpus_v1.txt`, sha256 `493f4fa8…`), with the key built by parsing each
record's own `moves` column into a sorted `(cell, player)` list — **no engine
involved**, so the count does not depend on the thing it is about to justify:

```
asked prefixes           742
distinct sorted-stone    347
distinct `key_pos`       347
distinct `key_full`      347
ratio                    2.1383   >= 1.5, so lever B SURVIVES §4.1
hit rate                 0.5323   MEASURED, and it is what §3's wall is re-derived with
```

**THE THREE COLUMNS AGREE AGAIN, ON A SECOND POPULATION.** WP-2.0b §9 found
distinct `key` equal to distinct `key_pos` on 798 in-tree firings; here the
sorted-stone key, the transposition key and the SYMMETRY-FOLDED key all return
347 on 742 corpus records. That is F-0.3's point measured twice: the extra fold
R1 names buys nothing any measurement in this project has found.

### F-1.1 — THE COUNT ABOVE WAS TAKEN BEFORE ITS REGISTRATION'S REVIEW, WHICH IS THE PRIOR ARC'S B2

`wp21_throughput_prereg.md` revision 1 has **never passed a fresh-context
review**, and §4.1 is a registered instrument step with a registered threshold.
Running it first is exactly the defect that STOPPED the prior arc — *"a
registration that governed a run before its review"* — and it is written down
here rather than discovered by a reviewer. **THE REMEDY IS THAT IT COSTS
MILLISECONDS**: the count is re-taken after the review passes, and the
post-review run is the one every downstream number cites. Until then no number
above may be quoted as governed.

### F-1.2 — LEVER B's REGISTERED VERIFICATION COSTS MORE WALL THAN LEVER B SAVES

§5 prices §4.4's byte-identity at *"~2 x (11 min + 3 h)"* — the tranche-sized
referent is what makes it ~4.7 h — against an ESTIMATED saving of **3.26 h** of
the 7.15 h eight-way wall. Taken as written the lever LOSES wall on this sweep
before a line of it is designed.

**THE AMENDMENT THIS ARC PROPOSES, for the reviewer to attack**: the
tranche-sized referent is **tranche one's own capture, run UNCACHED**. That
capture is work the sweep must do anyway; it is the pass that exists today, so it
is still an external referent sharing no code with the cache; and the only added
cost is the cached RE-capture of the same report (~1.4 h ESTIMATED). If the two
disagree, lever B is abandoned per §4.4 and tranche one's uncached corpus still
stands — nothing is wasted in either direction. Tranches two through sixteen run
cached.

### THE COLD-LABEL CRITERION R1 AMENDS, AND HOW IT IS DERIVED WITHOUT A WIRE CHANGE

R1: *"the cold-label agreement criterion is AMENDED to sample cache hits and
misses separately at the registered stride, both byte-equal in fresh processes."*
**WHICH RECORDS WERE HITS IS DERIVABLE FROM THE CAPTURE ITSELF** — walk the
records in order under §2's key; a key's first occurrence is a miss and every
later one is a hit — so `tools/cold_label_check.py` gains the derivation and the
capture grammar gains **no column**. A new column would be a format version bump
(D-572) for a fact the file already determines.

### F-1.4 — THE THROUGHPUT STUDY'S SOUNDNESS ARGUMENT FOR ITS CACHE KEY IS FALSE, AND THE REPAIR MAKES THE CACHE STRICTLY SAFER AND NO SLOWER

`wp21_throughput_prereg.md` revision 1 §2: *"Two prefixes whose move orders
differ but whose stones agree replay to the SAME `GameState` … so they must take
the same label."* **THEY DO NOT.** `GameState` carries `played()` — the stones in
play order — and the search reads it:
`crates/pistol-search/src/heuristics.rs:191-193` (`last_stone`), consulted at
`:155` under `gates.countermove`, and `:89`, which walks the whole play order
under `gates.killers`. Two transposed prefixes have different last stones, so
under either gate they can order the root differently and return a different
`bestmove` and node count. The sweep's own seat is safe by coincidence —
`configs/instrument_v0.toml:79-81` has all three ordering gates `false` — and a
soundness argument that holds only under three config values it does not name is
not one.

**THE REPAIR: THE KEY IS THE `position` LINE'S EXACT BYTES.** Then a hit answers a
question spelled identically to the one that produced the answer, and the cache's
soundness reduces to a law the project already gates — same binary, same config,
same `position`, same `go`, after `newgame`, same bytes (hard rule 4, CI gate 6).

**AND IT COSTS NOTHING MEASURED.** Over the pilot's 742 records the distinct
counts under raw move lists, sorted stones, `key_pos` and `key_full` are **347,
347, 347, 347** — the corpus holds no cross-game transposition and no symmetric
pair, so every duplicate is one prefix asked twice and the finest key catches all
of them. **Hit rate 0.5323 under every one of the four.**

**THE TELL A SUCCESSOR SHOULD KEEP**: revision 1 reached for the coarsest key it
could argue for, and the measurement says the coarseness was never worth
anything. The arc's recurring defect is a claim made at the scope where it was
convenient; this is its cousin — a GENERALITY claimed where it was not needed.

### F-1.3 — TWO `tools/ci.sh` RUNS WERE ON THE BOX AT ONCE, AND BOTH WERE KILLED RATHER THAN READ

The prior session launched a pre-merge CI at `5dbff8e` moments before it ended;
this session launched one at `2b94f04` without checking, and `ps` found both
alive, 21 and 9 minutes in, sharing `$ROOT/target`. **Neither result was kept.**
Cargo's own lock keeps two builds from corrupting each other, but a
timing-sensitive gate under 2x load is a gate that answers about the load, and a
receipt whose run overlapped another is not a receipt. Both process groups were
killed, the partial log deleted, and the run re-launched alone at `2b94f04`.
**THE STANDING HAZARD THIS BELONGS TO** is CLAUDE.md's *"no concurrent
cargo/bench"*; what it did not say, and what this adds, is **check `ps` for the
PREVIOUS session's detached jobs before launching one of your own** — a detached
run outlives the session that started it, which is the whole reason it is
detached.

### THE §1 AMENDMENTS ARE WRITTEN AND HELD

`wp21_prereg.md` revision 3 and `wp21_throughput_prereg.md` revision 2 are
written and **stashed**, not landed: the dispatch's §0 says no other tree work
until `dev` is green. `git stash list` names them. They land on `dev` after §0
closes, and each is reviewed at the revision it is landed as.

### F-0.4 — THE SWEEP'S OPERATOR HOLD IS READ AS LIFTED BY THIS DISPATCH, AND THE READING IS WRITTEN DOWN BECAUSE THE LAST SESSION GOT THE SAME QUESTION WRONG

`overnight2_ledger.md` §2 records a standing hold — *"wait before you launch the
sweep"* — and warns in its own words that the prior dispatch's §3 *"reads as
directing the run"* and that reading the hold as spent on that ground was
**wrong**.

**THE GROUND HERE IS DIFFERENT AND IS STATED RATHER THAN ASSUMED.** The hold was
a mid-arc verbal instruction addressed to the arc it interrupted. This is a NEW
dispatch, written after it, whose **title** is *consolidate -> sweep -> census ->
anchor v3*, whose §2 orders the sweep with three amendments (cache ON, range and
holdout, box idle during tranches), and whose §4 closure demands **"distinct
labelled positions delivered"** as its first number. A dispatch that names the
sweep in its title, amends its parameters, and asks for its output at closure is
not one the prior arc's hold survives.

**ARCHITECT DEFAULT APPLIED: the hold is spent, and the sweep runs when its
preconditions are green.** The preconditions are not weakened by that — §1's two
reviews, the concurrency study, and the label cache package all still gate
tranche one, and each is a longer pole than the hold ever was.

### THE TWO §1 REVIEWS ARE DISPATCHED, EACH AT A NAMED REVISION

The work is uncommitted, so the named revision is the `git stash create` object
**`fce50bc5b00baab9066f1e7bdf10c48c025b7755`** on `overnight2-stopped`
(HEAD `2b94f04`). Each reviewer is asked to state in its header whether that
revision still matches the working tree, and each is forbidden from running
`cargo` while CI owns the box.

| review | document | report path |
|---|---|---|
| sweep registration | `wp21_prereg.md` revision 3 | `docs/experiments/wp21_prereg_rev3_REVIEW.md` |
| throughput study | `wp21_throughput_prereg.md` revision 2 | `docs/experiments/wp21_throughput_prereg_rev2_REVIEW.md` |

**BOTH DOCUMENTS ARE HAVING THEIR FIRST FRESH-CONTEXT REVIEW.** Revision 1 of the
sweep registration was reviewed; revision 2 was not, and an amendment reopens the
review. The throughput study has never had one at all.

### F-1.5 — THE DOCUMENT CITED THE WRONG GATE NUMBER, AND THE TREE WAS FROZEN BACK RATHER THAN CORRECTED UNDER THE REVIEWERS

Revision 2 of the throughput study wrote *"`tools/determinism.sh` is CI gate 6"*.
**IT IS GATE 9 OF 19** — `tools/ci.sh:104-105`, *"gate 9/19: cross-process
determinism"*; gate 6 is config validation. This is the arc's own recurring
defect in its smallest form: a citation asserted rather than read off the file.

**AND THE CORRECTION WAS MADE AND THEN UNMADE**, which is the part worth
recording. It was applied to the working tree **while two reviewers were reading
that file at named revision `fce50bc`** — moving the tree under a review is an
instrument fault, and a review of a revision that no longer exists adjudicates
nothing. The file was restored to `fce50bc`'s bytes, the reviewers keep the
revision they were dispatched against, and **the correction is held as a
post-review amendment** with the better citation it should have had: gate 9's own
`C vs D` limb — *"the same positions under one budget, C with one process per
position and D with all of them in one session"* — is the cache's premise
already gated, on four seats.

### F-1.6 — THE §4.1 RECEIPT DID NOT CONTAIN THE ROW TWO DOCUMENTS QUOTED FROM IT, AND THE FIX IS A COMMITTED INSTRUMENT RATHER THAN A CORRECTED SENTENCE

The sweep registration's review found it: `artifacts/arc3_leverB_41_count.txt`
carries **three** distinct-count rows and names its key as *"the replayed
position's sorted (cell, player) list"* — revision 1's key, the one revision 2
replaced. The four-row block quoted in `wp21_throughput_prereg.md` §4.1 and in
F-1.4 above, whose first row reads *"distinct raw move lists (§2's key) 347"*,
**is not in that receipt**. The raw-move-list count was taken in a separate
throwaway command and never written down, and then attributed to the receipt
that was.

**THIS IS THE ARC'S NAMED DEFECT IN ITS PUREST FORM** — a claim asserted at the
scope where it was convenient rather than derived at the scope where it is true —
committed by the session that had already written the defect down twice.

**THE REMEDY IS AN INSTRUMENT, NOT A SENTENCE.** `tools/label_cache_count.py`
counts, on a real capture file, the asked prefixes and the distinct counts under
**the cache's own key** (the `position` line's exact bytes), the sorted stone
list, and the twelve-image symmetry fold — all three in one printed block, with
the command and its scope in the receipt. Taken over the pilot's own capture
(`capture_v1.txt`, sha256 `4563f050…`):

```
asked prefixes                          742
distinct `position` lines (cache key)   347
distinct sorted (cell, player) lists    347
distinct symmetry-folded stone lists    347
duplication factor                      2.1383
hit rate                                0.5323
```

**THE REVIEWER'S BOUND RESOLVES TO ITS FLOOR.** It could only say distinct ∈
[347, 371] and hit rate ∈ [0.5000, 0.5323]; measured under the actual key the
answer is 347 and 0.5323, so the registered figure was right and its evidence was
not. Receipt: `artifacts/arc3_leverB_41_count_v2.txt`, which SUPERSEDES `_v1`.

**AND THE INSTRUMENT'S SYMMETRY COLUMN IS AN INDEPENDENT CHECK OF THE ENGINE'S
OWN**: it folds twelve images in Python with no `pistol-core` in the loop and
returns 347, the same figure `wp20_CLOSURE.md` reports for `key_full`.

### §1 REVIEW ROUND 1 — BOTH FAIL, AND BOTH REVIEWS ARE WORTH THE ROUND

| review | verdict | findings | report |
|---|---|---|---|
| `wp21_prereg.md` revision 3 | **FAIL** | 4 BLOCKING, 12 MAJOR, 7 minor | `wp21_prereg_rev3_REVIEW.md` |
| `wp21_throughput_prereg.md` revision 2 | **FAIL** | 2 BLOCKING, 12 MAJOR, 8 minor | `wp21_throughput_prereg_rev2_REVIEW.md` |

Both reviewers confirmed the named revision `fce50bc` still matched the working
tree, and neither ran `cargo`.

**THE PRIORITY-ONE ATTACK FAILED, WHICH IS THE MOST USEFUL THING EITHER REVIEW
PRODUCED.** The throughput reviewer was asked to break the cache's soundness and
hunted the TT generation counter, the census collector, `Instant::now()` on the
ordering path, hash iteration order, `Board::stones()`, the eval's incremental
apply/undo, `ThreatState`'s rebuild order and the solver. **It found no
within-process channel by which two identical `position` lines could answer
differently**: `Board` is a `BTreeMap`, `Table::clear` resets `generation` to 0,
`ordering.rs` performs zero clock reads under `Stop::Nodes`, `normalise` strips
the only timing fields, and `Position::reset_to` rebuilds from a freshly replayed
state. The only state that survives `clear` is the census collector, which cannot
move an answer and which the design already refuses to combine with the cache.

**THE FOUR FINDINGS THAT CHANGE SOMETHING REAL**, ahead of the rest:

1. **`D-576` does not exist yet** — both reviewers found it, and both are right:
   the rulings block is §0.3's work and the registrations cite it as landed. It
   lands before round 2 is dispatched.
2. **The registered `arena --capture` command is refused by the shipped binary.**
   `arena.rs:51` matches `--capture <src> --out <p> --label-nodes <n>`
   positionally and the study registers the words in a different order. **And no
   dry run was recorded**, which is exactly the discipline that would have caught
   it.
3. **The §4.1 receipt did not contain the rows two documents quoted from it** —
   F-1.6 above, now remedied by `tools/label_cache_count.py` and receipt `_v3`.
4. **The replay estimate applies a 4-worker pilot rate under a criterion that
   mandates one worker.** Corrected, a tranche is **13 928 s = 3.87 h** and the
   eight-way wall **7.74 h**, not 7.15 — and the throughput study's own
   lever-exclusion argument (*"replay 3%"*) is really **10.1%**.

**AND ONE FINDING IS A PROCESS BREACH THIS ARC COMMITTED**: the cache key was
selected without an OPTION MATRIX and without a DECISION-RED-TEAM, which
CLAUDE.md calls *"the same breach as silent architecture drift"*. The matrix is
written and goes to a fresh-context red team **before** the registrations may
call the key settled.

### F-1.7 — THE PRE-MERGE CI RAN WHILE UNTRACKED FILES WERE ADDED TO THE TREE

`tools/label_cache_count.py` and
`crates/pistol-arena/tests/label_cache_count_tests.rs` were created at ~18:10-18:14
while `tools/ci.sh` was between gates 3 and 4. **This is the live-edit hazard this
project has already voided one run for** (`artifacts/wp20pilot_ci_VOID_liveedit_v1.txt`).

**HOW IT IS READ, AND IT IS NOT READ AS GREEN-BY-ASSUMPTION.** Gate 2 builds from
the git-tracked file set and cannot see an untracked file at all. Gates 3 and 4
run in the live tree, so they may have compiled the new test — and if they did,
they passed with it, since the run continued. **Adding an isolated test file and
a `tools/` script can add failures and cannot remove them**, so the committed
subset passed either way. That reading is recorded here rather than assumed, and
**the operative green for §0 is the CI taken after every §0 commit lands, on a
clean tree** — which the dispatch asks for anyway as §0.4.

---

## §0 — CLOSED. What landed, and in what order.

| step | receipt |
|---|---|
| pre-merge CI at `2b94f04`, all 19 gates | `artifacts/arc3_ci_premerge_2b94f04.txt` — **`ci: all gates passed`, EXIT=0**, 19 gate lines. Read under F-1.7 |
| `dev` fast-forwarded to `2b94f04`; tag `arc2/closure` | `git merge --ff-only`, `git tag -f arc2/closure 2b94f04` |
| ancestry check before deleting the branch | `git merge-base --is-ancestor overnight2-stopped dev` -> **YES**; branch deleted |
| the three audit files, digested BEFORE the commit | `artifacts/arc3_audit_digests.txt` — **ALL THREE MATCH** the dispatch's list |
| **D-575** the audit lands; the "verbatim" D-line recorded as not existing | `de1a73e` |
| **D-576..D-580** the architect's five rulings | `043f000` |
| both worktrees removed, with an export receipt | `artifacts/arc3_worktree_export_receipt.txt` — every gitignored file identical to a main-tree copy, every tracked-path blob recoverable from `dev`'s object store |
| ledger, both reviews, the cache-key matrix, the counter and its tests | `31315f8` |
| gate 1 caught the counter's test suite unformatted | `artifacts/arc3_ci_dev_31315f8.txt` — **`ci: FAIL: formatting`, EXIT=1** — fixed at `0321cc1` |
| **CI on clean `dev` at `0321cc1`** | `artifacts/arc3_ci_dev_HEAD.txt` — **`ci: all gates passed`, EXIT=0**, 19 gate lines. **§0 IS CLOSED.** |

**THE GATE-1 FAILURE IS WORTH ITS OWN LINE, BECAUSE IT IS F-1.7's OTHER HALF.**
The pre-merge run passed gate 1 before `label_cache_count_tests.rs` existed and
then (probably) compiled it at gate 3, where it passed. The first run that saw
the file at gate 1 said **no**. So the live-edit reading in F-1.7 was right in
its direction — an added file can only add failures — and the added failure
arrived one run later. `cargo fmt --all`, then the suite green at 5 of 5 and
`cargo clippy -p pistol-arena --all-targets -- -D clippy::all` clean.

### THE CACHE-KEY MATRIX'S DECISION-RED-TEAM — **the OPTION survives, the MATRIX does not**

Round 1 at `31315f8`: **1 FATAL, 6 MAJOR, 5 minor**
(`matrix_label_cache_key_REDTEAM.md`). Revision 2 answers every one, and rejects
one with its attempted reproducer.

**THE FATAL, AND IT IS THE ARC'S OWN DEFECT WEARING A NEW COAT.** Revision 1's
decisive row — every key folding the same 395 hits — was **MEASURED at thirteen
openings and written as a claim about 3 487**. The red team measured the gap:
the pilot offers **1 576** cross-game transposition opportunities and the sweep
**~161 000 000**, a factor of **102 346**; a rule-of-three bound on the pilot's
zero constrains the rate only to `<= 1.9e-3`, which at the sweep's scale admits
**up to ~307 000 merges**. And the mechanism makes it worse: deterministic
self-play means two games that ever transpose are identical thereafter, so the
fold's yield is **heavy-tailed — zero in most small corpora and large when it
fires** — and a zero at thirteen openings is that process's modal observation,
not its mean.

**THE SELECTION SURVIVES BECAUSE IT NEVER NEEDED THAT ROW.** Even if a coarser
fold merges 307 000 records, what the exact key loses is a **saving**; what every
coarser key buys that saving with is a class of **wrong answer**, invisible
downstream. A missed saving is the only error direction a cache may have.

**THREE THINGS REVISION 2 ADDS, AND THE THIRD IS THE ONE THAT MATTERS.**

1. **An a-priori argument for the part of the range that has one**: the book
   dedupes openings by `canonical_form`
   (`crates/pistol-cli/src/random_openings/mod.rs:174`), so **no two openings can
   transpose or mirror onto each other at `k <= opening_turns`** — a result rather
   than an observation, and available in a `git grep` the matrix never ran.
2. **An honest blank beyond it**, rather than a softer adjective.
3. **A FLIP CLAUSE THE GOVERNED RUN CAN SATISFY.** Revision 1's asked for *"a
   measured corpus in which the exact key's hit rate is materially below a
   coarser key's"* — and under this selection the sweep never counts its captures
   under a coarser key, so **the run generated no evidence about its own flip
   condition** (D-424: prose, not a criterion). Replaced: **the cache counts, per
   tranche, how many of its own MISSES share a `key_pos` or a `key_full` with an
   earlier miss.** Two counters, no extra search, in the run log beside the hit
   rate. That IS the coarser keys' yield at the scale that matters — and it
   settles D-562(2)'s open three-key question in the same pass.

**THE STRONGEST SURVIVING ATTACK CHANGED, AND THE NEW ONE IS SHARPER THAN THE
RECORDED ONE.** Revision 1 recorded that gate 9 does not run the sweep's seat or
budget. True, and weaker than this: **no limb of gate 9 ever asks the same
position twice inside one process.** `A vs B` runs one script in two processes;
`C vs D` runs one-process-per-position against all-in-one-session, and the session
limb asks each position once. *"The same position, asked again, in the same
process"* is exactly a cache hit and exactly the shape the determinism gate never
takes. **What closes it is §4.4's byte-identity run at the sweep's own seat and
budget**, which takes that shape ~6 624 times per tranche.

### F-1.8 — THE PLAY-ORDER CLAIM WAS THREE TIMES WIDER THAN THE CODE, AND D-576 CARRIES THE WIDE VERSION

Both the matrix and D-576 said the search reads play order *"under
`gates.countermove` … and `:89` walks the whole play order under `gates.killers`"*.
**Traced rather than asserted**: `heuristics.rs:89` sits inside `record_cutoff`,
whose call site is gated by `params.ordering.any()` (`pvs.rs:491-498`,
`params.rs:114-116`) and not by `killers`; its `last`/`second_last` feed only
`pair_killers` — written at `Phase::Second` nodes, where the stone is
search-placed, and **every capture root is `Phase::First`** — and the countermove
table, which is READ only at `:155` under `gates.countermove`. `history` never
reads `played()` at all.

**SO THE SINGLE PATH FROM A ROOT'S PLAY ORDER TO A SEARCH CHOICE IS
`countermove`.** The stone-set key is unsound under one config value, not three.
That narrows the failure mode and does not remove it, and it does not move the
selection. **D-576 is committed and append-only**, so the correction is a new
D-line naming it rather than an edit.

### F-1.9 — ONE RED-TEAM FINDING REJECTED, WITH THE REPRODUCER ATTEMPTED

MAJOR 2(b): *"under K1 the cold check cannot fail on a cache defect … K1 makes the
external referent vacuous against the cache."* **REJECTED.** Followed to the
record it is about: let record R be a HIT, its `position` field the line P, its
`(totals, bestmove)` from the memo. `tools/cold_label_check.py` asks P in a fresh
process and compares. **If the memo returned the entry belonging to some other
position Q, R carries `answer(Q)` and the fresh ask returns `answer(P)`** — they
differ and the check fails. What is vacuous under K1 is a check of the key's
EQUIVALENCE, and K1 has none, which is why it was chosen. The finding conflates
*"the key cannot be wrong"* with *"the memo cannot be wrong"*; only the first
holds. **MAJOR 2(a) is accepted in full** — the shipped checker has no partition
and the matrix's cell said it did.

**DOCS WERE EDITED WHILE THAT RUN WAS IN FLIGHT, AND THE READING IS DERIVED RATHER
THAN ASSUMED.** F-1.7's hazard again, so the scope was checked instead of hoped:
gate 17 reads tracked `.rs` and `.sh` files, gate 18 reads `docs/decisions.md`
(untouched during the run), and gate 19's `DOCS` list is six named files
(`tools/label_consistency_check.sh:102-107`), none of them touched. **No gate in
`tools/ci.sh` reads `docs/experiments/*.md` outside that list**, so the edits
could not reach a gate.
