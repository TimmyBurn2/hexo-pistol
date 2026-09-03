# REVIEW of the WP-2.1 revision-9 amendment remedy (round 2, remedies-only)

**REVISION REVIEWED**: `8a73bec` on `dev` (HEAD at dispatch). `git rev-parse HEAD`
at the end of this review: `8a73bec4d167d24ecbc1044609d6ce7ee1a18ec2` — **still
equals `8a73bec`**, unchanged throughout.

**REVIEWED ON: Sonnet, per D-597.**

Scope, per the dispatch: the remedy only — `git diff e3497b9 8a73bec --
docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md` — and
whether that diff introduced anything new. Round 1
(`wp21_prereg_rev8_amendment_REVIEW.md`, at `e3497b9`) already confirmed the
N = 16 selection, C1 over all 93 capture files, C2, and §3's wall arithmetic
(the block computing capture/play/replay/cold and the two totals); none of that
is re-derived here except where a finding below required re-touching the same
numbers to check the remedy's *new* prose against them.

Worktree used: `git -C /home/tom/Projects/HeXO-AlphaBeta worktree add --detach
/home/tom/pistol-wt/amend2 8a73bec`, `CARGO_TARGET_DIR=/home/tom/pistol-wt/amend2/target`
per command, removed at the end of this review. No sweep workload was run, lever A
was not re-run. `tools/governing_citation_check.sh` was run (text-only, no build
of engine binaries) and exits 0, 26+16 citations checked in the two documents,
0 unreproduced.

---

## VERDICT: **FAIL** — 0 BLOCKING, 1 MAJOR, 1 minor

Five of round 1's six findings are cleanly CLOSED, independently re-derived
below. F-6 is only **PARTIAL**: the remedy correctly narrows the over-broad
"every figure is a lower bound" claim, but the replacement paragraph it wrote to
do so contains a new arithmetic error — "worth 0.79 h of the 6.17 h" for
play+replay+cold contradicts both the wall block's own line items (2 909 s =
0.81 h) and the document's own two headline totals (6.17 h − 5.36 h = 0.81 h).
This number is new text (absent from every prior revision, confirmed by diffing
against `c68e69e` and `e3497b9`) and it is wrong. Separately, my own sweep for
"wave" turned up one leftover ordinal ("wave one") the remedy did not touch,
present since revision 7 and outside round 1's four cited instances; it
contradicts nothing and gates no decision, so it is minor, but it is a second
data point that the "swept as a class" claim was not quite exhaustive. Neither
finding blocks the sweep's substance (N = 16, C1–C4, the cache being OFF, T-F's
consequence, the 6.17 h total itself), but the round is not a clean PASS: the F-6
remedy needs a one-line numeric fix (0.79 h → ~0.81 h, or the scope of "worth
X h" narrowed to just play+replay) before this document should be read as final.

---

## F-1..F-6 DISPOSITION

| # | round 1 finding | disposition | command that decides it |
|---|---|---|---|
| F-1 | §1's cache row ("ON from the tranche §6.1 names") contradicted §6.1's every-tranche-uncached | **CLOSED** | `sed -n '57p' docs/experiments/wp21_prereg.md` now reads `**OFF on every tranche of this sweep**`; `sed -n '352,369p'` (§6.1) concludes "EVERY TRANCHE OF THIS SWEEP THEREFORE RUNS UNCACHED." Both name the same fact; no contradiction. |
| F-2 | four leftover "between the waves"/"wave two" references, T-F's stop-before-wave-two no longer achievable | **CLOSED** for the four cited instances; **one pre-existing instance neither round found, minor, see below** | `/usr/bin/grep -n -i wave docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md \| LC_ALL=C sort` — the four cited lines (169, 192, 448, and the sibling's 370–371 by old numbering) now read "the wave"/"AFTER the wave"; T-F's row (line 196) restates the consequence at N = 16 explicitly rather than promising a "wave two" stop. |
| F-2 (T-F wording) | is the one surviving "between the waves" honest, and is the restated consequence coherent? | **CLOSED** | `sed -n '196p' docs/experiments/wp21_prereg.md`: the survivor is scoped to "At the incumbent N = 8 the second half would have stopped the sweep between the waves and saved half of it; that saving is gone with the second wave" — explicitly a counterfactual about N = 8, not a claim about this sweep's own schedule. The restated consequence ("the second half…VOIDS the corpus the sweep just produced — nothing is delivered and no tranche is re-run") is internally coherent: it correctly derives that at N = 16 tranche one's capture exists only once the (single) wave is over, so T-F's second half can no longer pre-empt compute, only invalidate output already produced. It stretches "THE SWEEP STOPS" to cover a post-hoc void rather than a mid-run halt, but the paragraph itself explains the mechanism, so no reader is misled. |
| F-3 | sibling still called the cached re-capture "on the critical path," priced the lever off a deleted §3 block | **CLOSED** | `/usr/bin/grep -n -i "critical path" docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md` — all four surviving occurrences (sweep :164,170; sibling :349,389) now read "on no tranche's critical path." §5's cost table and prose (`sed -n '380,399p' docs/experiments/wp21_throughput_prereg.md`) price the lever's net cost as "net negative on this sweep's wall by its whole cost" and describe the deleted 0.20 h block only in past tense ("went with the second wave"), never as a present quantity the sibling still computes against. |
| F-4 | C4's rep at N = 1 was misread off the 3-decimal display (rep 3, +2.86 %) instead of the registered exact-throughput statistic (rep 1) | **CLOSED, independently re-derived from the raw receipt** | see below |
| F-5 | sibling cited `wp21_prereg.md` at revision 7, which is now revision 8 | **CLOSED** | `sed -n '1p' docs/experiments/wp21_prereg.md` → "revision 9"; `sed -n '15p' docs/experiments/wp21_throughput_prereg.md` → "`wp21_prereg.md` revision 9" — both bumped in step. `grep -n "revision 9" docs/experiments/wp21_prereg.md docs/experiments/wp21_throughput_prereg.md` shows the two headers plus the corrected cross-reference; no other digit follows "revision" in either file's cross-references to the sibling. |
| F-6 | "EVERY FIGURE IS A LOWER BOUND" was stale for the capture term, which is now measured at contention rather than extrapolated | **PARTIAL — the over-broad claim is fixed; the fix's own new number is wrong** | see below |

### F-4, re-derived directly from the receipt (not from the document's prose)

```
$ grep "^leverA: N 1 " artifacts/arc3r_leverA_c68e69e.txt
leverA: N 1 rep 1 cache off rc 0 wall_s 138.420 records 152 asks 152 s_per_label 0.910660 ...
leverA: N 1 rep 2 cache off rc 0 wall_s 138.266 records 152 asks 152 s_per_label 0.909643 ...
leverA: N 1 rep 3 cache off rc 0 wall_s 138.431 records 152 asks 152 s_per_label 0.910729 ...
$ sha256sum artifacts/arc3r_leverA_c68e69e.txt
5eb62d6c909699d76078b96f71801741d544fda98c3a58cbb07a83209eeeef01   # matches §1's cited digest
```

Exact throughput `152 / wall_s` per rep:

```
rep1: 152/138.420 = 1.09810720994076
rep2: 152/138.266 = 1.0993302764237052
rep3: 152/138.431 = 1.0980199521783414
```

Sorted ascending: rep3 (1.098020) < rep1 (1.098107) < rep2 (1.099330) — **median
is rep 1**, exactly as the amended §1 row states. `(0.910660 − 0.885445) /
0.885445 = +2.8477 % → "+2.85 %"`, matching the document exactly (not +2.86 %).
The two candidates' distance: `(rep1 − rep3)/rep3 = 0.00795 %` of throughput and
`2.8477 − 2.8557 = −0.0082` points of C4's twenty — both match the document's
"0.008 % of throughput and 0.01 points of C4's twenty." C1–C4's verdicts and the
N = 16 selection are unmoved (both reps sit inside C4's 20 % bar and neither
changes lever A's 95 %-of-highest computation, which reads median *throughput*
per N, not per-rep percentages).

### F-6, checked against the wall block itself

The remedy's replacement text (`wp21_prereg.md:172-178`) reads:

> **THE CAPTURE TERM IS MEASURED AT THE SWEEP'S OWN CONCURRENCY; THE OTHER THREE
> ARE LOWER BOUNDS.** Capture is 87 % of the tranche above… **Play, replay and the
> cold check keep their uncontended rates**… and are therefore lower bounds,
> **worth 0.79 h of the 6.17 h**…

Capture-fraction check (against the §3 wall block, `sed -n '138,147p'
docs/experiments/wp21_prereg.md`):

```
capture = 12443 * 1.551364 = 19303.62 -> 19304 s
total   = 22213 s (the document's own printed total, 6.17 h)
19304 / 22213 = 86.90 %  ->  "87 %"   MATCHES
```

Play+replay+cold check:

```
play   = 436 * 0.827115 * 4 = 1442.49 -> 1442 s
replay = 436 * 0.807692 * 4 = 1408.61 -> 1409 s
cold   = 64 * 0.904313      =   57.88 -> 58 s
sum    = 1442 + 1409 + 58   = 2909 s  = 0.8081 h  ->  "0.81 h"
```

`0.81 h` also falls straight out of the document's own two headline totals with
no re-derivation needed: `6.17 h − 5.36 h = 0.81 h`. The paragraph's claim of
**"0.79 h"** is not this: `1442 + 1409 = 2851 s = 0.79 h` is play+replay alone —
the number appears to have been computed on two of the three named quantities
and the "cold check" left off the arithmetic while staying in the sentence.
Diffed against every prior revision (`git show c68e69e:… `, `git show
e3497b9:…`, neither of which contains "0.79" or "0.81" anywhere in this
document), this is new text this remedy wrote, not a stale carry-over. It gates
no criterion or decision — the paragraph's own next sentence says so — which is
why this is MAJOR rather than BLOCKING under the dispatch's scale, but it is a
number the tree's own arithmetic contradicts and should read **~0.81 h**, or the
sentence should drop "the cold check" from its list of what "0.79 h" prices.

---

## WHAT THE REMEDY INTRODUCED — MY OWN SWEEP, BEYOND F-1..F-6

**"wave"** (`/usr/bin/grep -n -i wave` over both files, all ~35 hits read): every
occurrence describes either the current one-wave schedule, an explicit N = 8
counterfactual, or the deliberate T-F survivor — with one exception:

- `wp21_prereg.md:443`, a shell-comment header in §9's command block: `# T-F,
  BEFORE wave one, the box otherwise idle`. This ordinal predates the amendment
  (present identically in `c68e69e` at line 418 and in `e3497b9` at line 436;
  `git show c68e69e:docs/experiments/wp21_prereg.md | sed -n '418p'` and `git
  show e3497b9:… | sed -n '436p'` both confirm it). Round 1 did not cite it
  either. It is stale phrasing left over from the two-wave design — three lines
  later the same command block was actually swept (line 465's "before the wave
  is launched" reads correctly) — but it asserts nothing about a wave two, gates
  no criterion, and a reader who has read §1 and §6.1 first will read it as "the
  one wave, first of the shell script's phases" rather than as a schedule claim.
  **minor**: a leftover the "swept as a class" claim missed, not a
  contradiction.

**"cached"/"cache"** (`/usr/bin/grep -n -i "cached\\|cache"` over both files,
~90 hits read in full): every remaining occurrence is either (a) the OFF-shape
run-log mechanics, (b) §4.4/§4.5's verified-capability framing (consistently
"after the wave," never "on the critical path"), or (c) unrelated uses of the
word ("cache" as the subject of D-576's design, the label-cache count tool,
etc.). No occurrence asserts a tranche of *this* sweep runs cached.

**stale revision numbers**: `grep -n "revision [0-9]"` over both files — every
digit following "revision" that names either document reads 9 (own header,
sibling's citation of `wp21_prereg.md`) or correctly still 10
(`wp21_label_cache_design.md`, untouched and not amended by this remedy) or 3
(`matrix_label_cache_key.md`, likewise untouched). No stale 7 or 8 survives.

**D-586's flip clause, §5's new paragraph — checked against the code, not just
the prose**. The claim: "its two collision counters are printed only by the ON
shape." Read `crates/pistol-arena/src/label_cache.rs`:

```rust
pub fn line(&self, mode: LabelCache) -> String {
    match mode {
        LabelCache::Off => format!(
            "arena: label cache off: asks {} records {}",
            self.asks, self.records
        ),
        LabelCache::On => format!(
            "arena: label cache on: asks {} records {} hits {} key_pos_collisions {} \
             key_full_collisions {} fold_ms {}",
            ...
        ),
    }
}
```

`Off`'s format string carries no `key_pos_collisions`/`key_full_collisions`
field at all — confirmed **TRUE**, and stronger than the prose even states:
`Memo::insert` (the only place either counter is incremented) returns
immediately under `LabelCache::Off` before touching either counter (`if
self.mode == LabelCache::Off { return; }`), so an uncached tranche not only
fails to print the counters, it never computes them. The document's claim that
"the per-tranche floors… are read by no tranche here" is therefore sound: there
is no ON-shape line anywhere in an uncached sweep for a floor to be read
against.

No other class of leftover ("critical path," "lower bound," a stray "gate" or
"between the waves" outside the deliberate T-F survivor) turned up anything not
already covered in the F-1..F-6 table above.

---

## WHAT THIS REVIEW DID NOT LOOK AT

- Round 1's own confirmed territory, not re-run: the full 93-file sha256 sweep
  for C1, the C2 152-record spot-check, and the wall-block's capture/play/
  replay/cold arithmetic as a *pass/fail* matter (I did re-derive the same
  numbers, but only to check the F-6 remedy's new prose against them, not to
  re-audit C3/C4's bars or the N = 16 selection rule itself).
- Rounds 3–5's territory (T-A/T-A2/T-B/T-C/T-D substance, §2's partition
  arithmetic, the assembly instrument, §8's toolchain/digest table, §9.1's dry
  run record) — untouched by this remedy and out of scope again.
- `wp21_label_cache_design.md` and `matrix_label_cache_key.md` themselves —
  cited, digest-checked by `governing_citation_check.sh`, not re-opened.
- Whether N = 16 is the right engineering choice, or whether T-F's weakened
  guarantee at N = 16 (registered honestly in the T-F row and in
  `arc3_ledger.md`'s F-4.1) should instead have prompted reconsidering the slot
  — that is a design question this review surfaces via F-2's disposition but
  does not adjudicate.
- The sweep was not run and lever A was not re-run, per the dispatch.
