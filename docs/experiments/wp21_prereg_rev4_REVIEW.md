# REVIEW — `docs/experiments/wp21_prereg.md` revision 4. FRESH CONTEXT. ROUND 2.

**NAMED REVISION.** Commit **`114b5623fe40698bb0cf73b04990424935981e26`** on
`overnight2-stopped`.

**DOES IT STILL MATCH HEAD?** **YES.** `git rev-parse HEAD` =
`114b5623fe40698bb0cf73b04990424935981e26`; the only untracked path is
`docs/experiments/overnight2_RESUMED_SUMMARY.md` and no tracked file is dirty.

**WHAT I READ.** `CLAUDE.md`; `docs/process.md` (whole);
`docs/decisions.md` D-537, D-539, D-540, D-556..D-581 (D-575, D-576, D-577,
D-578, D-579, D-580, D-581 in full; D-568 in full);
`docs/experiments/wp21_prereg_rev3_REVIEW.md` (whole);
`docs/experiments/wp21_throughput_prereg.md` revision 3;
`docs/experiments/matrix_label_cache_key.md` revision 2 and
`..._REDTEAM.md` (verdict + §4 flip clause);
`docs/experiments/arc3_ledger.md`; `docs/book_v2_ledger.md`;
`docs/experiments/wp21_DISPATCH.md` (grep scope only);
`docs/experiments/overnight2_ledger.md` (§2 grep);
`docs/experiments/wp20_pilot_prereg.md` (the replay and cold invocations);
`artifacts/wp20pilot_RUN_2cd4f79_v1.txt`; `artifacts/arc3_leverB_41_count_v3.txt`;
`tools/wp21_tranche_config.py`; `tools/cold_label_check.py`;
`crates/pistol-arena/src/bin/arena.rs`; `crates/pistol-arena/src/bin/corpus-check.rs`;
`crates/pistol-arena/src/{outpath,conclusion,replay,capture_file,labels_file}.rs`;
`configs/instrument_v0.toml`; the pilot artifact directory listing.

No `cargo` was run and no repository file was modified except this report. Every
count below was taken with `/usr/bin/grep`, `git grep`, `git show`, `sed`,
`python3` or `ls`, with the command and its scope printed beside the claim.

## VERDICT: **FAIL**

**3 BLOCKING, 12 MAJOR, 5 minor.**

Revision 4 is a real improvement and not a cosmetic one. Eleven of round 1's
twenty-three findings are genuinely closed, several of them well: the hit rate is
re-measured under the key the cache actually uses by a committed instrument with a
coverage test; D-576 exists; T-A is split into two rows with two defect classes;
§6.1 points at §3 instead of restating a number; every single `file:line`
citation revision 4 added is correct when opened.

What fails is the same class round 1 named, in new places. §9's registered command
block — added to close round 1's B2 — contains a command word (`--label-cache`)
the shipped binary refuses and that exists nowhere in the crate, which is the
exact defect the sibling review found and which §9's own preamble boasts of having
prevented. The dry run that `docs/process.md` says must be exercised **before this
review passes** was registered, not taken. §5's resume rule still contradicts §4's
new re-run-whole rule — the rule round 1's B4 was raised about. And the new
arithmetic introduces three errors of its own: a doubled T-F cost, an
un-derivable headline "~53 hours", and a wall block that declares itself the
document's only wall while omitting the 2.62 h surcharge stated two sections
later, which turns a registered 1.63 h saving into ~0.20 h.

**A structural note that is not a numbered finding.** Revision 4 registers at
least five amendments it says must land before tranche one — `--partition`,
the generator's `--skip/--take` form, `--label-cache`, `tools/wp21_assemble.py`,
and §8's digest slot — and states of each that it reopens this registration.
Under CLAUDE.md's "reviews of superseded revisions do not transfer", revision 4
therefore cannot be the revision that governs the run whatever this review says.
That is honest of the document, but it means a PASS here would have licensed
nothing.

---

# DISPOSITION OF ROUND 1

| # | round-1 finding | status | evidence |
|---|---|---|---|
| **B1** | D-576 does not exist | **CLOSED** | `/usr/bin/grep -n 'D-57[5-9]' docs/decisions.md` → `1218:D-575`, `1220:D-576`, … `1230:D-581`. D-576 records R1 verbatim, the declined key and its ground. D-581 later corrects two of D-576's supporting claims — the decision itself rests. |
| **B2** | no command block, no dry run | **PARTIAL** | §9 exists and four of its six commands match the shipped binaries word for word (below). But §9.1 registers the dry run and records **no input and no output**, and the document defers it: *"the review that governs tranche one is taken after both are recorded"* (`:640`). `docs/process.md:34` — *"A pre-registration's literal commands are exercised **before its review passes**"* and *":41* — *"records the dry-run input and its output"*. The sibling took its dry run at §7; this one did not. See **N-B2**, **N-B1**, **N-M1**, **N-M7**. |
| **B3** | run log cannot distinguish a cached tranche | **PARTIAL** | §5 now records *"each pass **with its command VERBATIM**"* (`:456`) and requires the `cmp -s` line above the first `--label-cache` block (`:471-474`) — limbs (b) and (c) of round 1's fix are done. Limb (a), *"how a tranche is run cached vs uncached, by its literal spelling"*, is answered with a spelling that does not exist: **N-B1**. |
| **B4** | void rule contradicts the dispatch | **PARTIAL** | §4:`367-370` now reads *"**the tranche is RE-RUN WHOLE**"* and `:380-385` adds the shortfall obligation against 93 076 — both limbs of the fix. But §5:`478-479` was not touched: *"finds the last tranche with a PASS or **VOID** verdict, and starts at the next one. **A completed tranche is never re-run**"*. See **N-B3**. |
| **M1** | 0.5323 not measured under the cache's key | **CLOSED** | `artifacts/arc3_leverB_41_count_v3.txt` states *"SUPERSEDES _v1 (which counted the sorted-stone key, not the cache's)"* and prints `distinct \`position\` lines (cache key) 347`, `cache hits 395`, `hit rate 0.5323`, from `tools/label_cache_count.py` with its sha256. `git ls-files tools/ \| grep label` → `tools/label_cache_count.py` committed; `crates/pistol-arena/tests/label_cache_count_tests.rs` drives the shipped script (`.arg(repo().join("tools/label_cache_count.py"))`, lines 17 and 142). This is the best-executed fix in the revision. |
| **M2** | figure quoted as MEASURED is ungoverned | **CLOSED** | The condition dissolved rather than being papered over: the sibling's §4.1 (`:440`) is *"THE COUNT, RE-TAKEN BY A COMMITTED INSTRUMENT UNDER THE CACHE'S OWN KEY"* and §4.1.1 registers a second instrument and its agreement criterion before either runs. `/usr/bin/grep -n 'ungoverned' docs/experiments/wp21_throughput_prereg.md` → no output. |
| **M3** | replay rate not corrected to one worker | **CLOSED in §3** | `:237` — `ESTIMATED replay 436 x 0.8077 x 4 = 1 409 s`, re-derived: `436*0.8077*4 = 1408.63`. The rate is now `21/26 = 0.8077` rather than round 1's `0.81`, which is the more exact figure. But the preamble states the consequence as a different number: **N-M5**; and the sibling was not corrected: **N-M6**. |
| **M4** | §6.1 quotes a 7.15 h this document lacks | **CLOSED** | §6.1:`515-517` — *"§3 owns the wall and this section does not restate it (D-423; revision 3 quoted `7.15 h` here …)"*. `/usr/bin/grep -n '7.15' docs/experiments/wp21_prereg.md` returns only `:24` (the preamble's narration of the correction) and `:516` (the disclosure). |
| **M5** | ONE LINE asserts "eight at a time" | **CLOSED** | `:6` — *"run **N at a time, N a REGISTERED SLOT with incumbent 8**"*. (A different number in the same rewritten sentence is now wrong: **N-M2**.) |
| **M6** | re-registration trigger can never fire | **CLOSED** | §1's concurrent-tranches row: *"**ANY selected N other than the incumbent 8 reopens this document**"*, with the reason (every member of {1,2,4,8,16} divides 16) spelled out. |
| **M7** | T-A claims a cache defect on uncached tranches | **CLOSED in form** | T-A is two rows: T-A1 *"IN FORCE ON EVERY TRANCHE"*, T-A2 *"IN FORCE ONLY ON TRANCHES RUN CACHED"*, closure reports the counts separately. The residual gap the split creates is **N-M11**. |
| **M8** | instrument cannot perform T-A; output cannot express it | **CLOSED** | §4:`320-334` states outright *"THE INSTRUMENT DOES NOT EXIST YET AND THE CRITERION QUOTES ITS AMENDED OUTPUT"*, registers `--partition hits\|misses\|all` as **required** rather than defaulted with the reason, puts the class on the summary line, and names the coverage-rule test. Confirmed the gap is real: `/usr/bin/grep -n 'add_argument' tools/cold_label_check.py` → `--capture --binary --engine-config --stride --timeout-s`, and `:251` prints `{n} of {n} sampled record(s) agree byte for byte` with no class token. |
| **M9** | empty class unregistered | **PARTIAL** | §4:`338-347` registers both reachable behaviours and rules a class with *"fewer than **10 sampled records**"* a VOID. The registered minimum is unreachable on the one case the paragraph names as its reason: **N-M12**. |
| **M10** | §8 empty slot; two unnamed instruments | **CLOSED** | Preamble `:79-80` now reads *"whose digest §8 carries **a slot for, not yet a value**"*; §8 names `tools/label_cache_count.py` (committed, tested) and registers `tools/wp21_assemble.py` as **not existing yet** and as a precondition of §6 rather than of tranche one. `ls tools/` confirms no `wp21_assemble.py`. |
| **M11** | §4.1 paths, sub-range producer, missing costs | **PARTIAL** | Paths: all four added (`:392-407`), with the O_EXCL reasoning. Producer: `:415-422` registers the generator's `--skip/--take` form — confirmed absent from the shipped script (`tools/wp21_tranche_config.py:167-169` takes `--tranche --out --binary-sha256` only). Costs: added at `:426-431` — but in §4.1, not §3 as the fix asked, and not propagated into §3's wall (**N-M4**), and the capture figure is doubled (**N-M3**). |
| **M12** | "the dispatch says 3,500" | **PARTIAL** | The prereg's own paragraph is corrected (`:129-137` (the block ending at `:137`)) and its grep verifies: `/usr/bin/grep -n '3,500\|3500\|3 500' docs/experiments/wp21_DISPATCH.md` → no output, exit 1. D-568's text does say *"sweep takes the remaining 3,500"* (`docs/decisions.md:1204`), so the re-attribution is right. But `:137` claims *"`overnight2_ledger.md` §2 repeats it verbatim **and is corrected with it**"* — **N-M10**: it is not. |
| **m1** | 176 228 does not reproduce | **CLOSED** | `:213` now prints `199 027 x 0.885445 = 176 227`; `199027*0.885445 = 176227.41`. |
| **m2** | cold line not re-derived; `0.9` unsourced | **PARTIAL** | The count is re-derived (`34 + 30 = 64`, correct). The rate was changed to the wrong one on a false claim about the old one: **N-M8**. |
| **m3** | four inputs enumerated, six used | **OPEN, and worse** | Both paragraphs are now present and they contradict: **N-M7**. |
| **m4** | "twenty is the smallest take above thirteen" | **CLOSED** | `:355-360` — *"**TWENTY IS NOT … FOURTEEN IS**"*, choice retained, ground restated. |
| **m5** | `outpath.rs:9-24` off at both ends | **CLOSED** | `:441` cites `10-25` and `6-25`; `sed -n '1,28p' crates/pistol-arena/src/outpath.rs` → `6:/// Claim`, `10:pub fn claim`, `25:}`. Correct at both ends. |
| **m6** | ledger row stale at revision 2 | **OPEN** | `:83` still claims *"its ledger row is added in the same commit"*; `/usr/bin/grep -n 'wp21_prereg' docs/book_v2_ledger.md` → `42: … \`docs/experiments/wp21_prereg.md\` **revision 2**`. Now two revisions stale. |
| **m7** | T-C does not say where to look | **CLOSED** | T-C names `conclusion.rs:81` and `:111`. Verified: `:81` is the `counts n … forfeits {} …` format string, `:111` is `first_player_wins … forfeits {}`. And `end 1 (normal)` appears verbatim in `artifacts/wp20pilot_RUN_2cd4f79_v1.txt` (the C-E control block). |

**Score: 11 CLOSED, 8 PARTIAL, 2 OPEN, 2 CLOSED-in-form-with-a-new-gap.**

---

# BLOCKING

## N-B1 — §9 registers a command word the shipped binary REFUSES, and the flag exists nowhere in the crate. This is the sibling's own finding, in the document that opens §9 by citing it.

§9, `:592-597`:

> ```
> # 2 — capture. The word order is POSITIONAL (crates/pistol-arena/src/bin/arena.rs:51)
> arena --capture <SWEEP_DIR>/tranche-<n>/report.txt \
>       --out <SWEEP_DIR>/tranche-<n>/capture.txt --label-nodes 400000
> #     ... and, from the first tranche after §6.1's comparison returns, with
> #     --label-cache appended as the last word
> ```

`arena`'s dispatcher is an exhaustive match over literal argument slices
(`crates/pistol-arena/src/bin/arena.rs:39-86`). The capture arms are exactly two:

```
51:        ["--capture", source, "--out", out, "--label-nodes", nodes] => (
59:        [
60:            "--capture", source, "--out", out, "--label-nodes", nodes, "--census",
67:        ] => (
79:        _ => {
80:            return Err(format!(
81:                "--config and --out are both required, or --replay, --out and --workers, or \
```

A seventh word that is not `--census` falls to `_ =>` and the process exits
**REFUSED (2)** before any game. The `--census` arm is the proof of the shape:
adding one trailing word to this dispatcher requires a new arm.

And the flag does not exist at all:

```
$ git grep -n 'label.cache\|label_cache' -- crates/
crates/pistol-arena/tests/label_cache_count_tests.rs:17:        .arg(repo().join("tools/label_cache_count.py"))
crates/pistol-arena/tests/label_cache_count_tests.rs:142:        .arg(repo().join("tools/label_cache_count.py"))
```

Two hits, both the *counting script's* test, neither the arena. There is no
`--label-cache`, no cache module, no env var and no build feature in
`crates/pistol-arena`.

This is BLOCKING for three compounding reasons.

1. It is the identical defect the sibling review found and that this very
   section's preamble cites as its reason for existing: *"the sibling study's
   registered `arena --capture` command turned out to be REFUSED by the shipped
   binary for a word-order reason a dry run would have caught in seconds"*
   (`:584-587`). Revision 4 fixed the sibling's word order and then registered a
   new refused spelling four lines below the citation.
2. It is the answer to round 1's **B3(a)** — *"register how a tranche is run
   cached vs uncached, by its literal spelling"* — so B3 is closed by a spelling
   that cannot be typed. §5's void rule (*"a block carrying the flag with no such
   line above it is a VOID tranche"*) and §6.1's protective rule are both
   keyed on a token no run can produce.
3. §9.1 limb 1 registers *"every command above is **accepted by the shipped
   binary and exits 0**"* and §9.1 closes with *"A DRY-RUN FAILURE STOPS THE
   SWEEP"*. Limb 1 is unsatisfiable as written.

**FIX.** Either (a) drop the cached spelling from §9 and state that the cache's
invocation is registered by the amendment that lands it — with §5's and §6.1's
rules re-expressed over whatever that amendment actually adds — or (b) land the
flag first and register the spelling the binary accepts, with a new match arm
(and a decision about `--census`, which D-576 says the cache must refuse by name).

## N-B2 — the dry run is registered but NOT TAKEN, and `docs/process.md` requires it before this review passes. Three of §9's registered commands cannot be exercised at all, and §9.1 carves out only one of the three.

`docs/process.md:34-42`:

> *"A pre-registration's literal commands are exercised **before its review
> passes**, on an input of the SAME KIND as the registered workload … **The
> pre-registration records the dry-run input and its output.**"*

§9.1 records the input (`skip = 0, take = 1`, `--label-nodes 2000`) and five
falsifiable limbs. It records **no output**, and `:638-641` says so:

> *"Limb 3 cannot be taken until `tools/cold_label_check.py`'s `--partition`
> lands, so the dry run is taken in two parts and **the review that governs
> tranche one is taken after both are recorded**."*

That is a plan to satisfy the rule later, not the rule satisfied. The sibling
registration took its dry run (its §7) at the same revision it was reviewed at,
so the in-project precedent runs the other way.

Worse, the carve-out names one blocker and there are three:

| §9 / §9.1 element | instrument | exists? |
|---|---|---|
| `--partition hits\|misses` (§9 `:604-607`, limb 3) | `tools/cold_label_check.py` | **no** — `add_argument` at `:208-212` is `--capture --binary --engine-config --stride --timeout-s` |
| `--skip/--take` form (limb 2) | `tools/wp21_tranche_config.py` | **no** — `:167-169` is `--tranche --out --binary-sha256` only |
| `--label-cache` (§9 step 2, limb 1) | `arena` | **no** — N-B1 |

§9.1 acknowledges only the first. Limbs 1 and 2 are as untakeable as limb 3.

**FIX.** Take the dry run for the four commands that DO work (`--config/--out`,
`--capture/--out/--label-nodes`, `--labels/--report/--out`,
`--replay/--out/--workers`, `corpus-check`, and the generator's `--tranche`
form — all verified below to match the binaries), record its input and its
literal output in §9.1, and state that the remaining three limbs are taken by the
amendment that lands each instrument, each of which reopens this review anyway.

## N-B3 — §5's resume rule contradicts §4's new re-run-whole rule. The correction round 1's B4 demanded was made in §4 and not carried into the section that a successor actually resumes from.

§4, `:367-370`:

> *"A tranche that fails any criterion is **VOID AS A WHOLE**: … and **the
> tranche is RE-RUN WHOLE**."*

§5, `:477-480`, untouched from revision 3:

> *"A successor reads the log, finds the last tranche with a PASS or **VOID**
> verdict, and **starts at the next one**. **A completed tranche is never
> re-run**: its capture is 4 hours and its answer is already recorded."*

A successor following §5 skips past every VOID tranche and never re-runs one,
which is exactly the behaviour round 1's B4 identified as contradicting the
governing dispatch and as silently shrinking the delivered corpus. §5 is *the
resume point* by its own designation, so it is the rule that will actually be
applied.

The two rules cannot both be followed, and nothing says which wins. The §4
shortfall clause does not rescue it: under §5 a voided tranche is a permanent
shortfall the sweep never attempts to close.

**FIX.** §5's resume rule becomes: the successor starts at the first tranche with
no PASS — a VOID is an unfinished tranche, not a finished one — and *"a completed
tranche"* is defined as one with a PASS verdict. (While there: *"its capture is 4
hours"* is neither §3's capture figure, 3.06 h, nor its tranche figure, 3.87 h.)

---

# MAJOR

## N-M1 — §9.1 limb 3 registers a check that is arithmetically FALSE at the sweep's own numbers and false on the registered stand-in, and a dry-run failure "STOPS THE SWEEP".

§9.1, `:632-633`:

> *"`--partition hits` and `--partition misses` each print a line **naming the
> class**, and **their sample counts sum to the single-stride count**"*

Striding two partitions separately does not in general give the whole list's
stride count — it gives `ceil(n_hit/s) + ceil(n_miss/s) ≥ ceil(n/s)`. §3's own
figures falsify it:

```
$ python3 -c "import math; print(math.ceil(6624/200), math.ceil(5819/200), math.ceil(12443/200))"
34 30 63
```

**34 + 30 = 64 ≠ 63.** §3:`237` prints exactly this: `cold 64 x 0.8854`, with the
derivation *"`ceil(6624/200) + ceil(5819/200) = 34 + 30 = 64`"* (`:242-246`). So
§3 and §9.1 state opposite arithmetic four hundred lines apart.

On the registered stand-in it is worse. One opening ⇒ ≈57 records, ≈30 misses,
≈27 hits: `ceil(30/200) + ceil(27/200) = 2` against `ceil(57/200) = 1`. The limb
fails **by construction** on the input the document registers for it, and
`:637` says *"A DRY-RUN FAILURE STOPS THE SWEEP AND IS REPORTED AS A FINDING"*.

**FIX.** The check that actually holds and is worth registering is
`hits + misses == total records` and `class(hit) ∩ class(miss) = ∅` — the
partition is exhaustive and disjoint — not that their strided samples sum.

## N-M2 — the ONE LINE's "~53 hours" is derivable from nothing in §3, and revision 4 introduced it by applying the replay correction to a capture-only figure.

`:5` — *"**~53 hours of labelling ESTIMATED from MEASURED per-unit rates**"*.

Revision 3 said **~49**, which was right: §3's only serial labelling figure is
`199 027 x 0.885445 = 176 227 s = 48.95 h` (`:213`, `:281`).

```
$ python3 -c "
print('capture serial          ', 16*11017/3600)
print('capture + cold          ', 16*(11017+57)/3600)
print('capture + surcharge     ', 48.95+2.62)
print('whole tranche x16 serial', 16*13925/3600)
print('48.95 * 13925/12869     ', 48.95*13925/12869)"
capture serial           48.964
capture + cold           49.218
capture + surcharge      51.57
whole tranche x16 serial 61.889
48.95 * 13925/12869      52.97
```

Nothing in §3 gives 53 except the last line — scaling the **capture-only** serial
hours by the ratio of the new tranche total to the old one. The replay correction
does not touch a single label, so it cannot move a labelling-hours figure. This is
the headline number of the sentence the document calls *"the sentence a successor
reads first"*, and the same sentence round 1's M5 forced a rewrite of.

**FIX.** `~49 hours`, or state what 53 is a sum of.

## N-M3 — §4.1's T-F capture cost is DOUBLE, because it multiplies a per-OPENING rate by a GAME count. The same "40" is used correctly as games two lines below.

`:428-430`:

```
ESTIMATED  T-F's pair (20 openings)   2 x (40 x 57.0769 x 0.885445) = 2 x 2 022 s = 1.12 h
ESTIMATED  T-F's two play passes      2 x 40 x 0.8271 x 4          =   265 s = 0.07 h
```

`57.0769` is defined at `:206` as `MEASURED records per opening 742/13`. T-F is
**20 openings** (`:352-353`, `openings_skip = 13`, `openings_take = 20`). The
`40` is 20 openings × 2 games — which is what the play line correctly uses, since
`0.8271` is a per-game rate.

```
$ python3 -c "
r=0.885445
print('as written 40 x 57.0769 x r =', 40*57.0769*r)
print('correct    20 x 57.0769 x r =', 20*57.0769*r)
print('cross-check, per-game: 40 x (742/26) x r =', 40*(742/26)*r)"
as written 40 x 57.0769 x r = 2021.5
correct    20 x 57.0769 x r = 1010.8
cross-check, per-game: 40 x (742/26) x r = 1010.8
```

Round 1's own M11 estimated it correctly: *"20 openings ⇒ ~1 142 records ⇒
~2 x 1 011 s ≈ 0.56 h"*. Revision 4 doubled it while claiming to cost it properly.
The surcharge is therefore **2.06 h, not 2.62 h**.

The direction is conservative, so nothing downstream is under-budgeted — but §3
and §4.1 are the sections whose whole claim is that figures are re-derived and
not scaled, and a reader who checks this one finds it does not reproduce.

**FIX.** `2 x (20 x 57.0769 x 0.885445) = 2 x 1 011 s = 0.56 h`; surcharge total
`1.43 + 0.56 + 0.07 = 2.06 h`.

## N-M4 — §3 declares itself the document's ONE wall and omits the surcharge §4.1 registers, which is on the critical path by the sibling's own words. The registered 1.63 h saving is really ~0.20 h.

§3, `:277-282`:

> *"**THE WALL IS STATED ONCE, HERE, AND EVERY OTHER SECTION POINTS AT IT**
> (D-423)"*
> ```
> UNCACHED, N = 8, sixteen tranches   2 waves x 13 925 s = 27 850 s = 7.74 h
> CACHED from wave two, N = 8         13 925 + 8 060     = 21 985 s = 6.11 h
>                                     ESTIMATED SAVING ON THIS SWEEP  1.63 h
> ```

§4.1, `:426-431`, states 2.62 h of tranche-one surcharge (2.06 h once N-M3 is
fixed) and calls it *"costed rather than absorbed"*. It is not in the wall.

It cannot be absorbed. §6.1:`507-510` and the sibling's §4.4 both require the
byte-identity comparison to **return** before any tranche runs cached, and the
comparison's first half IS tranche one's 1.43 h cached re-capture. So on the
cached schedule the 1.43 h sits between wave one and wave two:

```
$ python3 -c "
print('cached wall as registered ', (13925+8060)/3600)
print('cached wall with the gate ', (13925+5152+8060)/3600)
print('uncached wall             ', 2*13925/3600)
print('realised saving           ', (2*13925 - (13925+5152+8060))/3600)"
cached wall as registered  6.107
cached wall with the gate  7.538
uncached wall              7.736
realised saving            0.198
```

The sibling says exactly this and the prereg does not: `wp21_throughput_prereg.md`
ONE LINE — *"the cache's applicable saving is 1.63 h ESTIMATED against ~1.43 h of
registered verification (§5) — **roughly a wash**"* — and its §5, *"1.43 h
(tranche one's cached re-capture, **on the critical path by construction**)"*.

So the one section that claims sole ownership of the wall carries the number the
sibling calls roughly a wash, without the qualification that makes it one, in the
document a successor reads for the wall.

**FIX.** §3's wall block gains a third line: the cached schedule with the §6.1
gate, and the net saving. Then §4.1's surcharge points at §3 rather than
standing alone.

## N-M5 — the preamble and §3 give two different values for the corrected tranche, and the sibling at HEAD carries the preamble's.

`:24` — *"**Corrected, a tranche is 13 928 s = 3.87 h**"*.
`:239` — `ESTIMATED  tranche  13 925 s = 3.87 h`.

§3's is the one that reproduces from §3's own inputs (`11 017 + 1 442 + 1 409 +
57 = 13 925`). The 13 928 is round 1's figure, computed at its `0.81` replay rate
(`436*0.81*4 = 1413`), transcribed into the preamble while §3 was recomputed at
the more exact `0.8077`.

It has already propagated: `wp21_throughput_prereg.md:665` reads *"wave 1,
uncached 12 869 s (registered; **13 928 s** under wp21_prereg revision 4's replay
correction)"*.

This is the memory's own "fix rounds derive, never transcribe" pattern: the
headline of the correction was copied from the review rather than derived from
the corrected block.

**FIX.** `13 925` in the preamble, and in the sibling.

## N-M6 — the prereg states the consequence for the sibling; the sibling at HEAD still carries the argument the consequence refutes.

`:248-250` — *"the consequence reaches the sibling study: capture is **79.1%** of a
tranche rather than 85.6%, and replay is **10.1%** rather than 3%."*

```
$ /usr/bin/grep -n '85.6\|replay 3%' docs/experiments/wp21_throughput_prereg.md
73:`wp21_prereg.md` §3's own arithmetic: capture **11 017 s of 12 870 s — 85.6%**.
74:Play is 11%, replay 3%, the cold check 0.4%. **A lever that is not on the capture
```

The sibling's §1 *rules a lever out* on that block — *"A lever that is not on the
capture pass cannot matter, which is what rules out … a faster replay"* — and its
ONE LINE was updated to 7.74 h while §1 and §5 were not. So at HEAD the two
registrations state the same percentages two ways, and the one that survives is
the one doing the excluding.

This is D-423's defect across the pair, in the exact place round 1's M3 said it
would land (*"tell the sibling"*).

**FIX.** The sibling's §1 block is re-derived from §3's 13 925: capture 79.1%,
play 10.4%, replay 10.1%, cold 0.4% — and the exclusion argument restated, since
a 10.1% component is not excluded by "cannot matter".

## N-M7 — §3 now carries BOTH the "four inputs" paragraph and a "SIX INPUTS" paragraph. They contradict, the "six" list enumerates seven items, and neither names the input that produces the document's most load-bearing rate.

`:192-198` (revision 3's paragraph, untouched):

> *"**The four inputs and where each is actually read**, kept apart because three
> of them are in the run log and the fourth is not: 13 openings, 26 games, 742
> records and **the two wall figures** are in `artifacts/…`"*

`:216-222` (revision 4's replacement, added below the same code block):

> *"**THE SIX INPUTS, AND WHERE EACH IS READ** — revision 3 enumerated four and
> used six … 13 openings, 26 games, 742 records, **the play wall and the replay
> wall** are in `artifacts/…`; the 347 distinct positions … come from
> `wp20_CLOSURE.md`; the 0.5323 hit rate comes from `artifacts/…`"*

Three defects in one place:

1. **The old paragraph was not removed.** The document that quotes D-423 —
   *"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING"* — states this claim
   twice, with different counts, five lines apart.
2. **The "SIX" list enumerates SEVEN** (13 openings, 26 games, 742 records, play
   wall, replay wall, 347 distinct, 0.5323).
3. **Neither list names the capture wall, 657 s.** The block's most load-bearing
   rate is `MEASURED seconds per label 657 / 742 = 0.8854` (`:205`) and `657` is
   in neither enumeration. The old paragraph's *"two wall figures"* meant 657 and
   21.505; the new one silently swapped 657 out for the replay wall. The true
   count of inputs consumed by the block is **eight**:

   ```
   $ /usr/bin/grep -n 'seconds=\|wall 21505\|n 26' artifacts/wp20pilot_RUN_2cd4f79_v1.txt
   18:  n 26  distinct-n 13  (13 duplicate games)
   27:  wall 21505 ms at 4 workers
   39:capture1 seconds=657
   72:replay seconds=21
   ```

Round 1's m3 was *"enumerates four and uses six"*. Revision 4 answered it by
adding a paragraph that enumerates six and uses eight, keeping the old one.

**FIX.** Delete `:192-198`; make the surviving list eight and include `657`.

## N-M8 — the cold-check rate was replaced with the WRONG measurement, on a claim about the old one that the document's own named source refutes.

`:242-246`:

> *"**THE COLD-CHECK LINE IS RE-DERIVED FOR T-A's AMENDED FORM** … at the
> MEASURED per-label rate rather than at **revision 3's unsourced `0.9`**."*

`0.9` was not unsourced. It is in the run log §3 names as its own source:

```
$ sed -n '62,68p' artifacts/wp20pilot_RUN_2cd4f79_v1.txt
== C-A: the cold-label agreement check, stride 1 ==
cold_label_check: 742 record(s) in capture_v1.txt; … which is 742 of them
cold_label_check: the label ask is `go nodes 400000`, one fresh process per sampled position
cold_label_check: 742 of 742 sampled record(s) agree byte for byte
cold exit=0
cold seconds=671
```

The pilot's cold check ran at **stride 1 over all 742 records** (registered as
SLOT S3 = 1, `wp20_pilot_prereg.md:220`, `:1168`) and took 671 s:
`671/742 = 0.9043` — MEASURED, per cold sample, and rounded to `0.9`.

`0.8854 = 657/742` is the **warm in-process capture** rate. A cold sample is not a
warm label: the run log's own line says *"one fresh process per sampled
position"*, so each sample pays a process spawn, a config load and an eval-table
load that the capture rate does not include. Revision 4 substituted the rate for a
different operation and called the correct one unsourced.

Magnitude is 1 s (`64 x 0.9043 = 57.9` vs `57`), so nothing downstream moves. It
is reported because the claim being made is a claim about provenance, in the
section whose entire thesis is provenance, and because it is round 1's m2 closed
by transcribing round 1's error rather than deriving the answer.

**FIX.** `64 x 0.9043 = 58 s`, sourced to `cold seconds=671 / 742 records` in the
run log, and drop the "unsourced" characterisation.

## N-M9 — "tens of gigabytes" is off by roughly a hundred-fold, and it is the SOLE stated ground for the `/home` rule. D-291.

`:433-437`:

> *"**`<SWEEP_DIR>` IS ON `/home` AND NEVER ON `/tmp`.** … this machine's `/tmp`
> is a **24 GiB RAM-backed tmpfs**, and a sweep that filled it would take every
> other running command down with it. The sweep's own output is **ESTIMATED in
> the tens of gigabytes** across sixteen tranches."*

The pilot's own artifact directory — already this document's source for five other
figures — measures it in one command:

```
$ ls -la /home/tom/pistol-runs/wp20pilot-artifacts/
-rw-r--r-- … 227828 capture_v1.txt      (742 records)
-rw-r--r-- … 469783 corpus_v1.txt       (742 records)
-rw-r--r-- …  15418 report_v1.txt       (13 openings)
$ python3 -c "
recs=199027; op=3487
print((227828/742*recs + 469783/742*recs + 15418/13*op)/1e9, 'GB')"
0.191 GB
```

**~0.19 GB, not tens of GB.** The whole sweep, captures and corpora and reports
together, is under a fifth of a gigabyte — 0.8% of the 24 GiB tmpfs. The stated
danger does not exist, so the stated ground does not license the rule.

CLAUDE.md's Process section: *"an estimate that could have been measured in
seconds is a finding (D-291)"* — and this one could, from a directory the
document already cites. It also fails the overrule test: the disputed claim is
the only thing said in support of a rule, so it does change what a reader may
conclude.

(The rule itself is right for a different reason — a tmpfs does not survive a
reboot and this sweep runs for days across one. That reason is not in the
document.)

**FIX.** *"ESTIMATED ~0.2 GB from the pilot's own per-record artifact sizes; the
ground for `/home` is that a tmpfs does not survive a reboot and this sweep
spans days."*

## N-M10 — the document asserts a correction to another document that was not made.

`:135-137`:

> *"`docs/experiments/overnight2_ledger.md` §2 repeats it verbatim **and is
> corrected with it**."*

```
$ /usr/bin/grep -n 'dispatch says\|3,500' docs/experiments/overnight2_ledger.md
186:- **The dispatch says 3,500 and the arithmetic says 3,487**: the round number is
```

Unchanged at HEAD. This is precisely the shape the round-2 brief names: a finding
closed by a sentence asserting the fix rather than performing it. The paragraph
doing the asserting is the one whose whole subject is that a claim was recorded
against a document that never said it.

**FIX.** Edit `overnight2_ledger.md:186` to `D-568 says 3,500`, in the same commit
as the assertion — or delete the assertion.

## N-M11 — T-A2 RUNS on uncached tranches but is NOT IN FORCE there, so a disagreement on half the sweep has no registered consequence — and that disagreement is exactly D-581's strongest surviving attack.

T-A2's defect column: *"**IN FORCE ONLY ON TRANCHES RUN CACHED**"*.
§4's derivation note, `:315-318`: *"**On an UNCACHED tranche the partition is
still taken and both strides still run**: the derivation is over the record list,
not over the cache."*

So on the eight uncached tranches the HIT-stride invocation is executed and its
result is registered as governing nothing. If it reports a disagreement, the table
header (*"A tranche passes only if every line below holds"*) says T-A2 does not
hold — but T-A2's own column says it is not in force. Nothing decides it.

That is not a hypothetical class. On an uncached tranche a HIT record is the
**second in-process ask of an identical `position` line**, and D-581 records that
this exact shape is what no limb of `tools/determinism.sh` ever takes:

> *"**AND THE SHARPER FACT IS THAT NO LIMB OF IT EVER ASKS THE SAME POSITION
> TWICE INSIDE ONE PROCESS** … *"The same position, asked again, in the same
> process"* is exactly a cache hit and exactly the shape the determinism gate
> never takes"* (`docs/decisions.md:1230`)

The uncached HIT stride is the only place in the whole sweep that check gets
taken, and the document runs it and then declines to be bound by it.

**FIX.** T-A2's defect column becomes: on a CACHED tranche it excludes a cache
that answers a hit differently from the miss; on an UNCACHED tranche the same
invocation is in force under **T-A1's** defect class, because a second in-process
ask disagreeing with a fresh process is a warm/cold defect whatever produced it.
Then it is in force on all sixteen and the closure still reports two counts.

## N-M12 — the empty-class rule's registered minimum is UNREACHABLE on the one case the paragraph names as its reason for existing.

`:341-347`:

> *"**REGISTERED**: a class with fewer than **10 sampled records** on a tranche
> that should have both makes the tranche's T-A a **VOID** and not a pass … On a
> full tranche neither class is empty (ESTIMATED 6 624 hits, 5 819 misses); **the
> rule exists for T-F's 20-opening sub-range** and for any other capture this
> instrument is pointed at."*

Ten *sampled* records at stride 200 requires ≥1 801 records in the class. T-F's
sub-range has neither class anywhere near it:

```
$ python3 -c "
import math
recs = 20*57.0769; miss = 20*26.6923; hit = recs-miss
print('records %.0f  misses %.0f  hits %.0f' % (recs, miss, hit))
print('sampled at stride 200:', math.ceil(miss/200), 'misses,', math.ceil(hit/200), 'hits')
print('records needed for 10 samples at stride 200:', 9*200+1)"
records 1142  misses 534  hits 608
sampled at stride 200: 3 misses, 4 hits
records needed for 10 samples at stride 200: 1801
```

So the registered rule VOIDs T-F's capture automatically, on the very case it was
written for. Round 1's M9 asked for *"the minimum sample it requires per class"* —
a minimum in **samples** at a fixed stride is unreachable at small n by
construction; the minimum has to be in **records**, or the stride has to scale.

**FIX.** State the minimum in records (*"a class holding fewer than N records"*),
or register a smaller stride for a sub-range capture, and re-check that T-F's
classes clear it.

---

# minor

**n-m1 — §3's code block uses two different roundings of one rate, and the cached
derivation mixes them.** `:234` computes capture at `12 443 x 0.8854 = 11 017`
(`11 017.03`) while `:265` computes cached capture at `5 819 x 0.885445 = 5 152`;
`:266`'s `13 925 - 11 017 + 5 152` therefore subtracts a rounded-rate product from
a total and adds an unrounded-rate one. 0.6 s, but the section's own thesis
(`:224-228`) is that a shown derivation must reproduce from the rate shown beside
it.

**n-m2 — `docs/book_v2_ledger.md`'s row is two revisions stale while `:83` claims
it lands in the same commit.** `/usr/bin/grep -n 'wp21_prereg' docs/book_v2_ledger.md`
→ `42: … revision 2`. Round 1's m6, unclosed.

**n-m3 — §4.1's registered file list omits `replay.txt`, which §9's own T-B
command writes, and §9.1 limb 5 forbids any file §4.1 does not register.**
§4.1:`392-407` lists `report.txt`, `capture.txt`, `corpus.txt`,
`tranche-<n>.toml`, `capture-cached.txt`, `tf/tf.toml`, `tf/report.txt`,
`tf/capture-{a,b}.txt` — no `replay.txt`. §9:`610-611` writes
`--out <SWEEP_DIR>/tranche-<n>/replay.txt`. §9.1 limb 5: *"the whole sequence
leaves exactly the files §4.1 registers **and no others**"*. Limb 5 fails on the
document's own command block.

**n-m4 — §9 registers no command for T-F's `cmp -s`, for the cached re-capture,
or for T-F's four files, though §4.1 registers their paths and §4 registers their
criterion.** §9's block covers passes 0-3, T-A, T-B and T-D. `cmp -s` appears at
`:353`, `:406` and `:471` in prose and in no command block. Round 1's B2 fix asked
for *"the three-pass command block **plus the four criterion commands**"*.

**n-m5 — §5's *"its capture is 4 hours"* matches neither of §3's figures**
(capture 3.06 h; whole tranche 3.87 h).

---

# WHAT SURVIVED ATTACK

**Every printed integer in §3 reproduces.** I re-derived the whole section from
its own stated inputs and found no arithmetic error except the two named above
(N-M3's T-F line and N-M5's preamble figure):

```
$ python3 -c "
r=0.885445
print('capture   12443*0.8854      =', 12443*0.8854)
print('play      436*0.8271*4      =', 436*0.8271*4)
print('replay    436*0.8077*4      =', 436*0.8077*4)
print('cold      64*0.8854         =', 64*0.8854)
print('tranche   sum of rounded    =', 11017+1442+1409+57, '=', 13925/3600, 'h')
print('wall      2*13925           =', 2*13925, '=', 2*13925/3600, 'h')
print('cached    5819*0.885445     =', 5819*r)
print('tranche c 13925-11017+5152  =', 13925-11017+5152, '=', (13925-11017+5152)/3600,'h')
print('cached w  13925+8060        =', 13925+8060, '=', (13925+8060)/3600, 'h')
print('saving    27850-21985       =', 27850-21985, '=', (27850-21985)/3600, 'h')
print('serial    199027*0.885445   =', 199027*r, '=', 199027*r/3600, 'h')
print('hits      12443-5819        =', 12443-5819)
print('pct       cap/tr, rep/tr    =', 11017/13925, 1409/13925)"
capture   12443*0.8854      = 11017.03      -> 11 017   ok
play      436*0.8271*4      = 1442.46       ->  1 442   ok
replay    436*0.8077*4      = 1408.63       ->  1 409   ok
cold      64*0.8854         = 56.67         ->     57   ok
tranche   sum of rounded    = 13925 = 3.868 h -> 3.87    ok
wall      2*13925           = 27850 = 7.736 h -> 7.74    ok
cached    5819*0.885445     = 5152.40       ->  5 152   ok
tranche c 13925-11017+5152  = 8060  = 2.239 h -> 2.24    ok
cached w  13925+8060        = 21985 = 6.107 h -> 6.11    ok
saving    27850-21985       = 5865  = 1.629 h -> 1.63    ok
serial    199027*0.885445   = 176227.4 = 48.95 h         ok
hits      12443-5819        = 6624                       ok
pct       cap/tr, rep/tr    = 0.7912, 0.1012 -> 79.1%, 10.1%  ok
```

The `34 + 30 = 64` derivation is right (`ceil(6624/200)=34`,
`ceil(5819/200)=30`), and the cached block's internal cross-check
(`12 443 x (1-0.5323) = 5 819 = 218 x 26.6923`) still passes. **The 1.63 h saving
is robust** and I could not break it: it reduces to `11 017 - 5 152 = 5 865 s`
and is invariant under the replay correction — which is why the prereg and the
sibling agree on it from different tranche totals. (Whether it is *realisable* is
N-M4; the arithmetic is right.)

**M1 is genuinely fixed, and it was the hardest one.** `_v3.txt` explicitly
supersedes `_v1` (*"which counted the sorted-stone key, not the cache's"*), prints
the count under the `position` line's exact bytes (347 of 742, hits 395, rate
0.5323), names its instrument with a sha256, names both inputs with their
sha256s, and prints the multiplicity structure (345 prefixes asked twice, 2 asked
26 times — the empty board and the forced origin stone, which is the right
explanation). `tools/label_cache_count.py` is committed and
`crates/pistol-arena/tests/label_cache_count_tests.rs` drives the shipped script.

**Every `file:line` citation revision 4 added is correct.** I opened all of them:

| citation | what is there |
|---|---|
| `wp20_pilot_prereg.md:1059` | `"$A" --replay "$ART/report_v1.txt" --out … --workers 4` |
| `replay.rs:20` | `pub fn run(transcript: &Transcript, workers: usize)` |
| `replay.rs:33` | `for _ in 0..workers.min(total.max(1))` |
| `conclusion.rs:81` | `"counts n {} distinct_n {} … forfeits {} decided {}"` |
| `conclusion.rs:111` | `"first_player_wins {} of {} decided_non_forfeit forfeits {}…"` |
| `outpath.rs:10-25` / `6-25` | `claim` without / with its doc — both exact |
| `instrument_v0.toml:113` | `on_search_path = false` |
| `arena.rs:51` | the six-word positional `--capture` arm |
| `docs/decisions.md` D-576 | exists, at `:1220`, with R1 transcribed |
| `matrix_label_cache_key.md` §4 | the two `key_pos`/`key_full` miss counters, at `:198-199` |

Round 1's m5 (`outpath.rs:9-24`) was a real citation defect and it is properly
fixed, with the off-by-both-ends noted.

**The `3,500` grep and the re-attribution are both right.**
`/usr/bin/grep -n '3,500\|3500\|3 500' docs/experiments/wp21_DISPATCH.md` returns
no output, exit 1 — scope: that one file. And `docs/decisions.md:1204` (D-568)
quotes the architect verbatim: *"sweep takes the remaining 3,500"*. The number is
D-568's, as revision 4 now says.

**Four of §9's six commands match the shipped binaries exactly, in word order.**
`arena --config X --out Y` (`arena.rs:44`), `arena --capture S --out O
--label-nodes N` (`:51`), `arena --labels C --report R --out O` (`:75`),
`arena --replay S --out O --workers N` (`:47`), `corpus-check <path>...`
(`corpus-check.rs:109`), and the generator's `--tranche/--out/--binary-sha256`
(`wp21_tranche_config.py:167-169`). `cold_label_check.py`'s four registered
arguments also match `argparse` at `:208-211`. Only `--label-cache`,
`--partition` and `--skip/--take` are unshipped.

**T-A1 is a real criterion with an external referent.** The instrument spawns one
fresh process per sampled position (`cold_label_check.py`, and the pilot log's own
*"one fresh process per sampled position"*), so it shares no table and no cache
with the capture pass — `docs/process.md:53-55`'s "externally derived referent",
which is what a reviewer looks for first. It is falsifiable: exit 1 on a
disagreeing byte, and §4's void rule is its consequence.

**T-C's and T-D's quoted lines exist verbatim.** `end 1 (normal)` and
`corpus_check: … ok, 742 record(s), capture_sha256 …` are both in the pilot run
log's C-E block, and `spread()` in `corpus-check.rs:50-54` produces the
`1 (normal)` shape.

**T-E is satisfiable by §9's own commands** — see the rejected attacks below.

**§8's honesty about `tools/wp21_assemble.py`.** Registering an instrument as
*"It does not exist yet; it is a precondition of §6 and not of tranche one, and
the closure may not report §6's counts until it is landed and reviewed"* is the
right shape for an owed instrument, and it closes M10's second limb properly.

**The partition, the holdout and the generator all still hold** — I re-derived
them rather than trusting round 1: `OPENINGS = 4500 - 1000 - 13 = 3487`
(`wp21_tranche_config.py:50-55`), `divmod(3487,16) = (217,15)` giving
`15 x 218 + 1 x 217`, `skip(17) = 3500`, and `slice_of` implements exactly that.

---

# ATTACKS I ATTEMPTED AND REJECTED

1. **"T-E's `capture_manifest`/`corpus_manifest` rows have no producer, like §6's
   assembly manifests."** Rejected. They are printed by the passes §9 already
   registers: `crates/pistol-arena/src/capture_file.rs:124` writes
   `capture_manifest capture_sha256 … body_sha256 …` and
   `labels_file.rs:152` writes `corpus_manifest body_sha256 …`, both pinned by
   tests (`capture_tests.rs:464`, `labels_tests.rs:869`). T-E is satisfiable as
   written; it is a different thing from §6's RAW/DEDUPED manifests, which §8
   correctly flags as instrument-less.

2. **"The `--census` token is missing from §9's capture command and the census is
   supposed to be off."** Rejected, and it is right the other way: the six-word
   arm at `arena.rs:51` sets the census flag `false`, and
   `wp21_tranche_config.py`'s generated config carries no census key. The seat
   matches the dispatch's *"census OFF"* by construction.

3. **"`corpus-check` needs an `--out` and §9 omits it."** Rejected — its USAGE
   says *"It reads and prints. It writes nothing and removes nothing."*

4. **"T-F's two captures will collide under `O_EXCL`."** Rejected — §4.1 now
   registers `tf/capture-{a,b}.txt` as two distinct paths, which is exactly the
   round-1 M11 fix.

5. **"The `x4` on the replay line over-corrects, since perfect scaling is a
   fiction."** Rejected, same as round 1: the direction is a cost over-estimate,
   it is labelled ESTIMATED, and the pilot's own per-game CPU (`compute a` +
   `compute b`) makes the `x4` conservative by ~12%. Applying it to replay is
   the correct symmetry, and T-B does mandate `--workers 1`.

6. **"0.5323 is still ungoverned, since the sibling revision 3 has not passed
   review either."** Rejected as a finding against *this* document. The
   ungoverned status came from a clause that no longer exists: the sibling's §4.1
   now records a re-take by a committed, tested instrument, and §4.1.1 registers
   a second instrument and its agreement criterion before either runs. The
   ordinary "revision N owes its own review" obligation covers what remains.

7. **"93 076 is stale after the replay correction."** Rejected — the distinct
   count is `3487 x 26.6923` and depends on no timing figure at all.

8. **"`no_tranche_reaches_the_reserved_holdout` would pass on a broken script."**
   Rejected, as in round 1: it shells out to the shipped `tools/wp21_tranche_config.py`
   and parses the written file, over all sixteen tranches.

9. **"§5's three cache counters have no registered instrument."** Rejected — §8
   names `target/release/arena` with its digest, and the counters are inside the
   arena per `matrix_label_cache_key.md:198-199` (*"Two counters, no extra
   search, one sort the arena already does"*). The instrument-naming rule is
   satisfied; the flag that turns it on is N-B1's problem, not this one's.

10. **"The `--partition` invocation in §4 misspells `cold_label_check.py`'s
    existing arguments."** Rejected — `--capture`, `--binary`, `--engine-config`
    and `--stride` all match `argparse` at `:208-211`. Only `--partition` itself
    is unshipped, and the document says so in bold.
