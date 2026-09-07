# DECISION-RED-TEAM — `matrix_wp22_phase2_eval.md` revision 2

## 1. Header

**Named revision**: `0641504294e5ef0bd4f0a1cadeecad3b8a49ff93` (`dev`), the revision
this review was dispatched against.

**Match with HEAD**: HEAD is **`a5687d69b0071750eba56b50fe69be765b312077`** and the
tree is clean (`git status --porcelain` empty). **The tree moved under this review
and the SUBJECT did not**: `git diff --quiet 0641504 HEAD --
docs/experiments/matrix_wp22_phase2_eval.md` returns 0 — the matrix is
byte-identical at HEAD. Three commits landed:

```
a5687d6 docs(enum): the round-three census replicates the round-two one on all two hundred shared cells
6238a05 docs(enum): the census receipt is wired and the two-round stop is marked superseded by round three
fbb6301 docs(enum): round three replaces the vacuous criterion with a stone-count referent, and it fires on every affordable rung
```

touching `docs/decisions.md`, `docs/experiments/hex_threat_enum_v1.md`,
`docs/experiments/wp22_phase2a_STOP_E.md` and four files under `tools/hex_enum/`.

**WHAT MOVED AND WHETHER THE FINDINGS STILL APPLY.** Every finding below was
derived against the matrix's own bytes and every one still applies, because the
matrix did not change. **One finding is CREATED by the move** and is recorded as
BLOCKING-4: D-709 granted four rounds per review gate, Stage E reopened at round
3, and the STOP record the matrix's banner applies its default fork from is now
marked *"SUPERSEDED … READ THIS AS A RECORD, NOT AS A DISPOSITION"* in its own
first line. Round 3 built the criterion §2.5 says could not fire, and it fired.
Two findings below (MAJOR-9, MAJOR-10) are **strengthened** by round 3's
full-corpus measurement, which I re-derived independently and which confirms what
I had inferred from the 3 000-position scope alone.

**Instruments I built** (scratch, `/tmp/claude-1000/rt/`, nothing written to the
live tree but this report; no `cargo`, no `CARGO_TARGET_DIR`, no worktree):

| instrument | what it is | what it answers |
|---|---|---|
| `walk.py` | an independent corpus walk — my own manifest parse, my own move→stone parser, my own three-axis line enumeration, and R6's predicate re-written from D-622's sentence. Shares no code with `tools/hex_enum` or `tools/texel`. | §1.1's four counts |
| `power.py` | an independent SPRT power simulator: numpy `PCG64` seed 12345 against the shipped `SplitMix64` seed 1, vectorised cumulative moments against a per-pair loop, Python against Rust. Same tilt model, no shared code. | every power figure in §1.6 and §5 |
| `purity.py` | re-derives every purity ratio from the census artifacts at **both** units, all four lengths and all four rungs — 32 cells where the matrix reports 16 | §2 item 4 |
| `paired.py` | the seed pilot's eight splits treated as PAIRED, which is how they were drawn | §3 reading 3 |
| `nps.py` | the throughput-bracket model re-implemented and re-run at four profile shares and two `c` ranges | §4/§5's registered floors |
| `r3.py` | re-derives round 3's §7.4 criterion table from `census_r3` | the HEAD supersession |

I ran `sha256sum -c` on all six receipt directories rather than trusting printed
digests. All six verify clean; the seventh (`census_r3`, added at HEAD) also
verifies.

---

## 2. Re-derivation ledger

Every command is mine. Where a number is reproduced only by the document's own
instrument it is marked so, per `docs/process.md`.

### 2.1 Receipts — verified, not trusted

| directory | my command | result |
|---|---|---|
| `census` | `cd artifacts/wp22_phase2a/census && sha256sum -c RECEIPT_census.sha256` | 8/8 **OK** |
| `clockfix_confirm` | same shape | 6/6 **OK** |
| `n6_mutants` | same shape | 2/2 **OK** |
| `nps_seat` | same shape | 6/6 **OK** |
| `seed_pilot` | same shape | 2/2 **OK** |
| `sprt_power` | same shape | 4/4 **OK** |
| `census_r3` (HEAD only) | same shape | 8/8 **OK** |
| receipt digests | `sha256sum artifacts/wp22_phase2a/*/RECEIPT_*.sha256` | §10's five digests all match; **§1.6's does not** — see MAJOR-2 |

### 2.2 §1.1 — the corpus, re-derived by an independent walk

`/usr/bin/python3 /tmp/claude-1000/rt/walk.py`, scope = the deduped manifest at
`/home/tom/Projects/pistol-corpus/arc3r-sweep/assembly/deduped_manifest.txt`
joined to all sixteen tranche corpora; my own R6 predicate.

| claim | matrix | my command's answer | reproduced |
|---|---|---|---|
| manifest digest | `00f61780cc…f35968` | `sha256sum` on **both** copies (corpus tree and `artifacts/arc3r_sweep_deduped_manifest.txt`) → identical | **YES** |
| deduped positions | 89 805 | 89 805 (89 819 lines − 14 header) | **YES** |
| `eval` rows | 74 672 | 74 672 | **YES** |
| quiet rows under R6 | 45 271 | 45 271 | **YES** |
| quiet share | 60.63 % | 60.6265 % | **YES** |
| distinct games | 3 487 | 3 487 (and 3 487 among quiet rows too) | **YES** |

This is the strongest section of the document and it reproduces under an
implementation that shares nothing with either census.

### 2.3 §1.6 and §5 — power, re-derived by a second instrument

`/usr/bin/python3 /tmp/claude-1000/rt/power.py 20000` — numpy 2.5.2, PCG64,
seed 12345, 20 000 runs, buckets `2,3,8,7,4`. MC standard error at p ≈ 0.9 is
0.0021.

| cell | matrix | my instrument | Δ | reproduced |
|---|---|---|---|---|
| cap 8000, elo1 10, truth 10 | h1 0.9045 | 0.9072 | +0.0027 | **YES** |
| cap 4000, elo1 10, truth 10 | h1 0.6970 | 0.6992 | +0.0022 | **YES** |
| cap 3554, elo1 10, truth 10 | h1 0.6446 | 0.6435 | −0.0011 | **YES** |
| cap 5233, elo1 10, truth 10 | h1 0.8045 | 0.8026 | −0.0019 | **YES** |
| cap 500, elo1 30, truth 30 | h1 0.7430 | 0.7459 | +0.0029 | **YES** |
| cap 500, elo1 30, truth 0 | h0 0.7385 | 0.7391 | +0.0006 | **YES** |
| cap 500, elo1 40, truth 40 | h1 0.9067 | 0.9093 | +0.0026 | **YES** |
| cap 500, elo1 50, truth 50 | h1 0.9467 | 0.9459 | −0.0008 | **YES** |
| cap 1000, elo1 30, truth 30 | h1 0.9189 | 0.9251 | +0.0062 | **YES** |
| cap 1000, elo1 30, truth 0 | h0 0.9172 | 0.9169 | −0.0003 | **YES** |
| mean pairs at cap 8000 | (not in the matrix; `book_v3_registration` §R1 says 3102.1) | **3114.9** | +0.4 % | **YES**, and see MINOR-5 |

**The power arithmetic is sound.** What is done with it is not — BLOCKING-3.

### 2.4 §2 — the census, and the sixteen cells that are thirty-two

`/usr/bin/python3 /tmp/claude-1000/rt/purity.py`, scope = all four
`census_L*.txt`, **both** printed units, all four rungs.

| claim | matrix | mine | reproduced |
|---|---|---|---|
| `k` ladder at L11 | 357 / 121 / 47 / 36 | 357 / 121 / 47 / 36 | **YES** |
| `C(k+2,3)` at L11 T4 | 7 647 059 | 7 647 059 | **YES** |
| L13 T4 classes / codes | 232 vs 231; 108 075 vs 108 074 | 232 vs 231; 108 075 vs 108 074 | **YES** |
| every rung's ω² lower at 13 than 11 | asserted | true at **both** units, 8/8 | **YES** |
| T4 retains 62 % of the ceiling | 62 % | 0.005504 / 0.008842 = 0.6225 | **YES** |
| T2 observed codes / median / ≤4 / <20 | 1 533 / 41 / 322 / 620 | 1 533 / 41 / 322 / 620 | **YES** |
| T2 growth over 9× the data | 35 % | 1530/1132 = 1.3516 | **YES** |
| purity "MET at all sixteen cells, by 2.0x to 3.6x" | 16 cells, 2.0–3.6 | **window unit only**: 16 cells, 2.003–3.589. **Code unit**: 16 more cells, **1.704–2.806**, four of them below §2.5's disqualifying 1.77x | **NO — see MAJOR-10** |

### 2.5 §3 — the seed pilot, re-derived as a PAIRED design

`/usr/bin/python3 /tmp/claude-1000/rt/paired.py`, scope = `pilot_L11.txt`'s 24
per-seed rows rather than its 3 summary rows.

| claim | matrix | mine | reproduced |
|---|---|---|---|
| val MSE means / spreads / worst-weight dev | table | transcribes `pilot_L11.txt` exactly | **YES** |
| "±3.5 %" | reading 1 | = half-range / mean = 3.46 %. sd = 2.2 %; SE of the mean of 8 = 0.75 % | arithmetic **not on the page** — MINOR-3 |
| "the gain 9→65 params is 5.6 %" | reading 3 | 5.606 % | **YES** |
| "an architecture comparison by offline loss is not separable from split noise at a single seed" | reading 3, *"the one that prices the rows"* | **FALSE.** K=64 beats K=32 beats K=8 on **8 of 8 shared splits** in all three pairwise comparisons; paired t = 14.86 / 12.85 / 12.05; sign-test p = 0.0039 each; sd of the paired difference 7 021 against a mean gain of 36 879 | **NO — BLOCKING-2** |

### 2.6 §1.2 / §1.3 — the throughputs, checked against the receipt they cite

`cat artifacts/wp22_phase2a/nps_seat/nps_instrument.txt` and `nps_play.txt`.

| claim | matrix prints | the receipt holds | reproduced |
|---|---|---|---|
| instrument seat, all bands | **531 548** (min 531 037, max 532 574) | `band all: nps median 529255 (min 526479, max 531293)` | **NO** |
| instrument seat, early | 570 722 (566 960 / 571 806) | 564 304 (559 064 / 568 030) | **NO** |
| instrument seat, late | 491 110 (489 672 / 495 473) | 492 073 (491 110 / 494 010) | **NO** (491 110 is the receipt's *min*, printed as the *median*) |
| `depth_turns 2` | 450 233, 73 388 nodes in **163 ms** | 453 012, 73 388 nodes in **162 ms** | **NO** |
| the replicate | "529 255 nps, 0.43 % from the first" | 529 255 — the receipt **is** the replicate | the headline run is unreceipted |
| `play_v0.toml` nps | **172 627 / 172 665 / 172 211** | **171 851 / 171 625 / 172 499** | **NO** |
| `play_staged_v0.toml` nps | **478 718 / 480 449 / 482 105** | **482 274 / 481 548 / 482 127** | **NO** |
| the 2.8x | 2.773x | 2.806x from the receipt | the ratio survives; the numbers do not — MAJOR-1 |

### 2.7 §1.8 and the registered nps floors

`/usr/bin/python3 /tmp/claude-1000/rt/nps.py`; profile shares from
`docs/audit/repo_audit_2026-09.md` A-02's own entry list.

| claim | matrix | mine | reproduced |
|---|---|---|---|
| `WINDOWS_PER_CELL` = 18 | 18 | `Axis::ALL.len()` 3 × `WINDOW_LEN` 6 = 18 (`crates/pistol-core/src/window.rs:6,9`) | **YES** |
| codebook bracket at p = 0.3177 | [341 617, 546 003] | [341 617, 546 003] | **YES** |
| R-H-EXT bracket at p = 0.3177 | [403 391, 499 791] | [403 391, 499 791] | **YES** |
| "the eval at **31.77 %** of the profile" | 31.77 % | A-02's own list also carries `HandcraftedV0::undo` **6.64 %** and `HandcraftedV0::apply` **6.29 %**. The eval's share is **44.70 %** | **NO — MAJOR-3** |
| the floor at 44.70 % | — | **298 245 (0.561x)**, not 341 617 (0.643x) | the registered abort threshold is 14.5 % too lenient |
| the floor at the receipted 529 255 | — | 340 144 | the floor also depends on an unreceipted run |

### 2.8 §1.6's books arithmetic and the D-653 reading

| claim | matrix | mine | reproduced |
|---|---|---|---|
| closed form at Δ=10 | 3 554 / 5 233 | `2·ln(19)/t1²` and `ln(19)²/t1²` with `t1 = 10·ln10/800·√2` → 3 554.4 / 5 233.0; matches D-628 and `1046535/100/2` | **YES** |
| `ceil_to_500(P+500)` at every row of the table | 15 000/21 500, 4 500/6 000, 1 500/2 000, 1 000/1 500, 1 000/1 000, 1 000/1 000 | all six pairs reproduce | **YES** |
| "cap by the openings rule" = N − 500 | 8 000 on 8 500; 500 on 1 000 | the inversion is arithmetically right | **YES** |
| what the `+500` IS | not stated | `book_v2_registration.md:145-146`, verbatim: *"**`+ 500`** is the **Stage-3 detector's own standing slice**, which must not compete with it for the same openings."* A **book-sizing** term, not a consumption margin | **NO — BLOCKING-3** |
| "D-653's two clauses do not both hold" | §5 | D-653 writes `elo1 **>=** 30`. At elo1 = 40 both clauses hold — as the matrix's **own §5 row and §6.1** assert three lines away | **NO — BLOCKING-3** |
| the holdout at the 1 000 pairs it physically seats | not computed | h1 **0.9251**, h0 at truth 0 **0.9169** (mine); the receipt's own 0.9189 / 0.9172 | the "26 % veto failure" is **8 %** at 1 000 pairs |

### 2.9 Quotations and citations

| claim | my command | result |
|---|---|---|
| §1.5's `book_v3` reuse rule "verbatim" | `sed -n '16,19p' docs/book_v3_ledger.md` | **exact** |
| §1.5's D-653 quote | `sed -n '1372p' docs/decisions.md` | text exact; **the flip clause is dropped** — MINOR-10 |
| §5's "D-705, **whole**" | `sed -n '1476p' docs/decisions.md \| fold -w 100` | **one sentence of five paragraphs** — BLOCKING-1 |
| §8: "round 4's report … returns nothing" | `git log --all --oneline -- docs/experiments/matrix_wp22_quiet_scale_REDTEAM_round4.md`; `git log --all --diff-filter=A --name-only` | **TRUE** — only rounds 1, 2, 3 ever existed |
| §8: "every row `eval_families` §7 lists is present" | `grep -o 'R-[A-Z0-9-]*' docs/research/eval_families_2026-09.md \| LC_ALL=C sort -u` | **TRUE** — all seven, plus R-H-EXT which §7 does not list |
| R-H-EXT's calculus citations | `sed -n '64,66p;153,154p;158p;177p' docs/research/threat_calculus_v1.md` | all four resolve at the quoted lines |
| §1.7's 2.046 s | `sed -n '250,256p' docs/experiments/matrix_wp22_quiet_scale.md` | **exact** |
| §1.7's 1 040 answers / 100 games | `grep -n '1040\|100 games' docs/experiments/sealbot_anchor_v7_protocol.md` | **both resolve** (`:170`, `:393`) |
| §1.4's refusal | `cat crates/pistol-arena/src/validate.rs`; `git grep -n '\.validate()'` | **unconditional** — first check in `validate()`, called on the load path at `config.rs:207`, pinned by `config_tests.rs` |
| "§11 is the failed-precedent check" | `grep -n '^#\{2,3\} ' docs/experiments/matrix_wp22_phase2_eval.md` | the document has **no §11** and ends at §10 — MINOR-1 |
| "§6.5's purity criterion" (×2) | same | the document has **no §6.5** — MINOR-2 |
| "this workstation's `python3` has no `numpy`" | `/usr/bin/python3 -c "import numpy"`; `pacman -Qq \| grep '^python-numpy$'` | **numpy 2.5.2 is installed** — MAJOR-8 |
| §1.3 "two committed play seats" | `git ls-files 'configs/*' \| LC_ALL=C sort` | **three** `play_*` configs — MINOR-4 |

### 2.10 The HEAD supersession, re-derived

`/usr/bin/python3 /tmp/claude-1000/rt/r3.py`, scope = `census_r3`'s four files.

| claim (HEAD, `hex_threat_enum_v1.md` §7.4) | mine | reproduced |
|---|---|---|
| criterion MET in 4 of 16 cells | 4 of 16, cell for cell | **YES** |
| all sixteen ratios | 1.014 … 0.378, every one to 3 dp | **YES** |
| count-only class counts 23/38/52/69 | 23/38/52/69 | **YES** |
| ceilings 0.005171/0.006837/0.008842/0.010542 | identical | **YES** |
| **the enum's gain over stone counting on the FULL corpus, T4** | **L7 +1.4 %, L9 +10.8 %, L11 +12.5 %, L13 −16.6 %** | the matrix's transferable "**4 %**" is none of these — MAJOR-9 |

---

## 3. Findings

### BLOCKING-1. §5 asserts something D-705 says, while claiming to quote D-705 whole — and §1.6's headline finding is the ADR's own premise

§5 opens *"D-705, **whole**"* and quotes one sentence. It then states:

> **AND ONE THING D-705 DOES NOT SAY IS TRUE OF THE COMMITTED BOOKS.** Its
> acceptance rule needs two arms; §1.6 measures that `book_v3` funds one at
> D-653's power floor…

**Minimal reproducer**: `sed -n '1476p' docs/decisions.md | fold -w 100 -s`. D-705
contains, verbatim:

> **AND THE ASYMMETRY IS DELIBERATE**: two `h1` bars would double the openings a
> row costs, and D-568's reservation plus `book_v3`'s 8 500 cannot fund that for
> every row.

D-705 says exactly the thing §5 says it does not say, and says it as the **reason**
the veto's bar is "not h0" rather than "h1". Three consequences:

1. §5's sentence is false about a quoted ADR — the class of round-1 M-8, in the
   document whose §8 claims *"the two ADRs this matrix leans on hardest — D-653
   and D-705 — are quoted whole rather than glossed."* Neither is.
2. §1.6's *"THREE THINGS FOLLOW AND NONE OF THEM IS A PREFERENCE"* — point 1,
   "`book_v3` funds exactly ONE acceptance arm" — is not a finding against the
   standing rule. It is the standing rule's stated ground, now measured. That is
   worth having; presenting it as a gap in the ADR misstates what is known.
3. **§1.6's split arithmetic prices a design D-705 explicitly rejected.**
   "Splitting it between two arms drops each to 0.6970" models two `h1` arms at
   Δ = 10 on `book_v3`. D-705's second arm is a **veto at "not h0" on the play
   seat**, chosen precisely so it does not cost an `h1`'s worth of openings. The
   question D-705 poses — what does a "not h0" veto cost in openings, and what is
   its power to fire? — is never asked. §1.6 has half the answer (h0 0.7385 at
   500 pairs) and attaches it to the wrong argument.

Also omitted, and load-bearing on §5's own table: D-705's last sentence, *"a row
without one is not testing what decides the outcome,"* said of the nodes/sec
floor. §5's table registers no floor for four of eight rows.

### BLOCKING-2. §3's reading 3 — the reading §3 itself says "prices the rows" — is false on the pilot's own receipted data

§3 states:

> **The gain from 9 parameters to 65 is 5.6 %, and the seed spread is 6.9 %.** So
> on this corpus **an architecture comparison by offline loss is not separable
> from split noise at a single seed**

**Minimal reproducer**: `/usr/bin/python3 /tmp/claude-1000/rt/paired.py`, over the
24 per-seed rows of the receipted `pilot_L11.txt`.

```
K=8 vs K=32: K32 better on 8/8 seeds; paired mean diff 23472.2 sd  5167.6 t=12.85  sign p=0.0039
K=8 vs K=64: K64 better on 8/8 seeds; paired mean diff 36878.5 sd  7020.7 t=14.86  sign p=0.0039
K=32 vs K=64: K64 better on 8/8 seeds; paired mean diff 13406.3 sd 3145.8 t=12.05  sign p=0.0039
```

The eight splits are the **same eight splits** for all three K, and this is the
linchpin, so it is checked at the instrument rather than assumed —
`sed -n '218,232p' tools/hex_enum/seed_pilot.py`:

```
for k in ks:
    ...
    for seed in range(seeds):
        order = list(games)
        random.Random(1000 + seed).shuffle(order)
```

The shuffle is keyed on `1000 + seed` and on nothing else; `games` is fixed
before the `k` loop. **Seed `s` is the identical train/validation partition at
every K by construction.**

So the 6.9 % spread is *common mode* across conditions and cancels in the
comparison: the sd of the paired difference is 7 021 against a mean gain of
36 879 — 5.25 sigma. The comparison the sentence makes — a between-condition mean
gain against a within-condition half-range — is unpaired arithmetic on a design
the instrument deliberately paired.

**What it licenses, and this is why it is BLOCKING.** §3 says this reading "is the
one that prices the rows", and it does:

- §4 R-A2's kill: *"`eval_families` §7: 'it does not beat A3's validation loss…' —
  and §3 measures that the first half is not separable from split noise at 65
  parameters, **so the kill as written cannot fire cleanly**."* It fires at t = 12.
- §3's own *"which is D-614 arrived at from a direction D-614 did not take"* —
  D-614 stands on its own ground; this is not corroboration of it.
- §6 rank 4 for R-A2 rests partly on *"the row must choose one by an offline
  metric"* being unworkable. The Rapfi-Table-2 half of that argument survives
  (offline loss ranks nets wrongly *for strength*); the split-noise half does not.

Reading 2 — *"individual weights move by up to 65 % of the table's scale between
splits"* — is TRUE and untouched. The document conflates weight instability with
loss non-separability; they are different facts and only one of them holds.

### BLOCKING-3. §5's "D-653's two clauses conflict" is a misreading, and the cap that produces it is the matrix's own inversion of a book-SIZING rule

§5 closes:

> **D-653 designates the holdout as the screening book at `elo1 ≥ 30` and requires
> computed power ≥ 0.9, and at `elo1 = 30` … the computed power is 0.7430.** That
> is D-628's shape — a registered test that cannot return its registered outcome
> — found in a standing ADR.

Two independent reasons this is wrong.

**(a) `>=` is not `=`.** `sed -n '1372p' docs/decisions.md`: D-653 reads *"the
large-effect screening book (**elo1 >= 30**, power computed per D-628) … no run
spends the holdout below computed power 0.9."* The conjunction is satisfiable —
at `elo1 = 40`, power 0.9067. **The matrix's own §5 table row for R-H-EXT and its
own §6.1 both use exactly that cell.** The document asserts an ADR is
unsatisfiable and satisfies it three lines away. That is round-2 M-3 / round-3
MAJOR-1's defect class — a claim its own adjacent column refutes — reappearing in
the section §8 credits with confronting it.

**(b) The 500-pair cap is not a registered constraint.** §1.6's table column is
headed *"cap by the openings rule"* and inverts `n_openings = ceil_to_500(P+500)`
into `P ≤ N − 500`. **Minimal reproducer**: `sed -n '132,150p'
docs/experiments/book_v2_registration.md`. The rule's `+500` term is defined
there:

> **`+ 500`** is the Stage-3 detector's own standing slice, which must not compete
> with it for the same openings.

That is a **sizing** rule for building a new book with a named other claimant's
slice inside it. For `book_v3` — a book actually built by the rule — the inversion
happens to give the right answer (8 000). For the v2 holdout, a 1 000-opening
residual nothing sized by this rule, the inversion silently reserves another 500
for the Stage-3 detector and never says so. A 1 000-opening holdout physically
seats 1 000 pairs, and at 1 000 pairs the matrix's **own receipt** measures
`elo1 = 30` at **h1 0.9189** and `h0 0.9172` at truth 0 (mine: 0.9251 / 0.9169).
So:

- D-653's clauses hold at `elo1` exactly **30** on the holdout's own 1 000
  openings. There is no conflict to report.
- §1.6 point 2's *"the veto **fails to fire in 26 %** of the cases it exists
  for"* is **8 %** at the openings the holdout has. That number is quoted again
  in §6.1 as the thing a careless matrix would sell.

The inversion may still be the right policy — the detector's claim is real. But
the matrix reports the consequences of its own unstated policy as defects in a
standing ADR, and hands the architect a false finding against D-653.

### BLOCKING-4 (created by the tree's move; does not impugn the named revision)

The matrix's first act is applying the dispatch's default fork on the two-round
STOP. At HEAD, `docs/experiments/wp22_phase2a_STOP_E.md:3-15` reads *"**SUPERSEDED
BY D-709 AND BY ROUND 3's RESULT — READ THIS AS A RECORD, NOT AS A DISPOSITION.**"*
D-709 granted four rounds per review gate; Stage E reopened; round 3 replaced the
vacuous criterion with a stone-count referent and ran it on the whole 45 271-row
population. I re-derived its table from `census_r3` (receipt verified, digest
`9bb8fc6f…`): **MET in 4 of 16 cells**, and `L = 11` T2 — the rung §2 item 3 calls
*"the rung the corpus supports"* — sits at **0.637** of a quotient that cannot see
a window.

The matrix's conclusion (R-A4-CLASS unpriced) survives. Every sentence it reaches
it by is superseded:

| matrix sentence | HEAD says |
|---|---|
| §2 item 4: *"The registered purity criterion is MET at all sixteen cells, by 2.0x to 3.6x"* | that is the **random** referent — the FLOOR — which §7.4 now says *"says only that the enum is not noise"*. The registered criterion fails at 12 of 16 |
| §2.5: *"revision 1's kill condition **could not fire** rather than declining to"* | a criterion that could fire was built and **did** |
| §2 item 3: T2 at L=11 *"is the rung the corpus supports"* | 0.637 |
| §6's closing 4 % | see MAJOR-9 |

The fix is mechanical: §2, §2.5 and §6's closing re-point at `hex_threat_enum_v1.md`
§7.4. But the matrix as it stands is the document an architect reads, and it
currently licenses "the criterion was vacuous and the enum is worth 4 % over stone
counting" where the tree says "the criterion fired and the enum loses to stone
counting at twelve of sixteen cells".

### MAJOR-1. §1.2's and §1.3's headline throughputs are not in the receipt they cite, and a pre-registered abort threshold is computed from one of them

**Minimal reproducer**: `cat artifacts/wp22_phase2a/nps_seat/nps_play.txt` — none
of §1.3's six numbers appears. `cat nps_instrument.txt` — `band all` is
**529 255**, which §1.2 calls the *replicate*; the headline **531 548** and its
min/max, and the `depth_turns 2` row's 163 ms, are from a run no receipt holds.
Ledger table 2.6 has all eight rows.

Under hard rule 8 and §10's own sentence — *"an artifact directory is evidence
only while a tracked document carries its receipt's own digest. **This table is
that anchor**"* — the headline figures have no anchor at all. This is D-469's loss
shape, which §8 congratulates itself for recording about the *other* document, and
it is round-2 M-5's class (*"the play-change receipt could not be reproduced from
what it recorded"*), which §8's own row claims this matrix does not repeat.

**It is load-bearing.** 531 548 is the base of every nps floor the matrix
pre-registers under hard rule 5. At the receipted 529 255 the codebook floor is
**340 144**, not 341 617. The 2.8x conclusion in §1.3 survives (2.806x from the
receipt against 2.773x printed); the numbers do not.

### MAJOR-2. §1.6 prints a receipt digest for `sprt_power/` that matches nothing

**Minimal reproducer**:
```
git grep -n '37a86fe1991ec1c54b5d7e299a92075dc4115d0594cd00f6b34715a1e404030d' 0641504
  → only the matrix line 191
find artifacts/wp22_phase2a -type f -exec sha256sum {} \; | grep 37a86fe1  → nothing
sha256sum artifacts/wp22_phase2a/sprt_power/RECEIPT_sprt_power.sha256
  → d82fce37add53453423e264a6220e2b47636302c7f2980c4a60f6c600005709b
```
§10 prints `d82fce37…` for the same directory. Two digests, one directory, and the
one printed beside the numbers is dead. It has been wrong since revision 1
(`200542f`); the author's arithmetic pass checked arithmetic and not digests.

### MAJOR-3. §1.8 understates the eval's profile share, and every registered nps floor inherits it

§1.8 is headed *"The eval's share of the profile"* and §4's cells say *"the eval at
31.77 % of the profile"*. **Minimal reproducer**: `grep -n 'A-02'
docs/audit/repo_audit_2026-09.md` — the same `perf report`, same 2 105 samples,
lists `31.77% HandcraftedV0::delta`, **`6.64% HandcraftedV0::undo`** and
**`6.29% HandcraftedV0::apply`**. CLAUDE.md's own map makes apply/undo the eval's
incremental contract; a codebook replaces all three. The eval's share is **44.70 %**.

`/usr/bin/python3 /tmp/claude-1000/rt/nps.py`, same model, corrected share:

| row | matrix floor | at 44.70 % |
|---|---|---|
| codebook rows (1.833x traffic, c ≤ 1.5) | 341 617 (0.643x) | **298 245 (0.561x)** |
| R-H-EXT (1.0x traffic, c ≤ 2.0) | 403 391 (0.759x) | 367 345 (0.691x) |

Every pre-registered abort threshold is ~14 % too lenient — a row that should
abort on cost passes. Hard rule 5's abort threshold is the one number in a bench
registration that must not be optimistic.

**And the `c` brackets are an unstated per-row judgement.** The codebook rows get
`c ∈ [0.5, 1.5]`; R-H-EXT gets `c ∈ [1.2, 2.0]`. Nothing on the page says why a
codebook lookup may be *half* the cost of the incumbent table while three hand
integers are *always* dearer. Priced with the codebook rows' own bracket, R-H-EXT's
floor is **458 686 (0.863x)**. The bracket, not a measurement, is what separates
the two families' floors.

### MAJOR-4. The "observations per parameter" column is at least four currencies, and the rank-2-versus-rank-3 comparison crosses them

The matrix admits **one** such case — R-A5's 811 versus 143 — and calls it "D-477's
own class". It is not the only one. **Minimal reproducer**: the last block of
`/usr/bin/python3 /tmp/claude-1000/rt/nps.py`.

| row | what the column actually reports | denominator |
|---|---|---|
| R-A4-CLASS | scored **cells** per nominal parameter | 8 174 025 |
| R-A5-TOPK | **window observations** per code | 12 980 519 / 16 613 729 |
| R-A3, R-A1 | **positions** per parameter (`45 271 / 38 983 = 1.16`) | 45 271 |
| R-A2 | **positions** per generator parameter, and says so | 45 271 |

The same 38 983-parameter table reads **1.2** in R-A3's row and **333** at the unit
R-A5's row is priced at. §6 then compares them directly: rank 3 is justified by
*"its real parameter count is A1's killed 1.2 observations each"* while rank 2 is
justified by *"both counts are affordable against 12 980 519 window observations"*.
Swap the currency and the two rows swap places.

Worse, **Buro's line is applied in two currencies three rows apart**: R-A4 and
R-A5 count *occurrences* against it (*"322 at or below Buro's ≤ 4 line"*, *"4 730
codes at or below"*), while R-A1 compares **1.2 positions/parameter** to *"Buro's
'the weight is set to 0' below ~4 **matches**"*. `eval_families` §A1's Logistello
7.3 IS positions-per-weight, so that half is apples-to-apples; Buro's 4 and 20 are
not. The honest density statement for A1/A3 is the one §4's kill row already
carries — *median 7 observations, 56.8 % under ten* — and §6 quotes the other one.

### MAJOR-5. §5 declares the discriminating column dead, then ranks on the axis that column would have measured

§5: *"**THE COLUMN THAT WAS MEANT TO SEPARATE THE ROWS IS CONSTANT** … none of them
is a property of any row. **So the two-arm rule cannot rank this field**."*

The three *reasons* the arm is unavailable are indeed common to every row. Its
*discriminating power* is not. D-705's published ground is precisely that *"a
node-matched SPRT cannot see the throughput half of an eval change, and
throughput is exactly what an incremental codebook trades accuracy for."* The
field splits cleanly on that axis:

| traffic | rows |
|---|---|
| 1.00x, 18 touches | R-H-EXT, R-C-SPSA |
| 1.83x, 33 touches | R-A5, R-A3, R-A2 (and R-A4 had it) |

At 1.83x the veto is live; at 1.00x it is nearly vacuous. That is the definition
of a discriminating column. §6 then ranks **#1 the 1.00x row**, whose stated
advantage over the field is *"traffic unchanged at 18 touches per stone"* — the
exact quantity the arm it declared non-discriminating exists to test. The matrix
takes the missing measurement's likely verdict and awards it without the
measurement. §6's own closing concedes the shape (*"an ADR moving the arena's
movetime refusal makes §5's column real, at which point the throughput brackets
start discriminating"*) — which is an admission that the column discriminates, in
the section that ranked on it.

### MAJOR-6. §6/§7 price R-H-EXT's cost-before-runnable as "none", and it is the whole of the only corpus-disjoint holdout

§7: `R-H-EXT | cost before it can be run | **none beyond writing the terms**`.
§6.1: *"without spending `book_v3` at all."*

**Minimal reproducer**: §1.6's own table — *"time-matched screen, Δ = 30 |
`book_v2` holdout, **1 000 openings** | cap 500 pairs"*. A 500-pair screen at the
matrix's own sizing takes the entire holdout. Then
`sed -n '97,105p' docs/book_v2_ledger.md`:

| claimant | status |
|---|---|
| The Stage-3 detector's SPRT | **SCHEDULED** |
| The WP-1.5d ±21.5 resolution run | **LICENSED, NOT SCHEDULED** |

and D-628, an ADR §1.6 cites for its own 3 554: *"the run would have spent **400 of
the 1 000 openings D-568 reserves for governed runs, on which two packages have
standing claims**."* And `docs/book_v2_ledger.md:56-67`: WP-2.2 Phase 1 registered
400 of that reservation, the row *"stood for two commits"*, and it was **withdrawn**
— *"Spending 400 of a reservation two other packages have claims on … has been put
to the operator instead of taken."*

The matrix ranks #1 a proposal to spend **all 1 000** of the same reservation, on
the same ranking axis it calls "cost before it can be run", and mentions neither
the claimants nor the precedent. On the matrix's own axis, R-H-EXT is the most
expensive row in the field, not the cheapest: it is the only one whose
cost-before-runnable is an irreplaceable, never-labelled, pre-committed sample
that two other packages are waiting on.

### MAJOR-7. "the only row that can use the screening book" is asserted, not measured, and the matrix's own citations cut against it

§5 and §6.1 both turn on it: *"**AND IT IS THE ONLY ROW THAT CAN USE THE SCREENING
BOOK**: a hand term either moves play a lot or it does not, so it is a large-effect
question."*

D-653's registered property of the holdout is that it is *"the only corpus-disjoint
v2 slice"* — which is exactly what a **corpus-trained** row needs, and what D-644
spends `book_v3`'s whole filter to buy. Nothing bars a learned row from a
`elo1 = 40` screen there. The only gate is effect size, and nothing measures that a
learned eval is a Δ=10 question rather than a Δ≥30 one. The matrix's own §4 cites,
against itself: Rapfi Mixnet Large at **−82 / −58 / −45 / −38** Elo; figrid's
1 024-accumulator net losing **23 points**; and D-705's *"300-400 ELO"* at equal
wall clock. Those are large effects from eval changes.

If a learned row can be screened on the holdout, R-H-EXT's decisive advantage
evaporates and the "one funded acceptance run" premise weakens to "one `book_v3`
acceptance plus one holdout screen, and the screen is not reserved".

### MAJOR-8. "this workstation's `python3` has no `numpy`" is false, and it is a priced cost for the row ranked last

**Minimal reproducer**:
```
/usr/bin/python3 -c "import numpy; print(numpy.__version__)"   → 2.5.2
pacman -Qq | grep -E '^python-numpy$'                          → python-numpy
```
What has no numpy is the *mise-shimmed* `python3` first on this shell's PATH. The
claim is about a PATH, presented as a property of the workstation — `docs/process.md`'s
named defect, *"a claim CHECKED AGAINST THE WRONG POPULATION"*, in a sentence
tagged as a cost. torch is genuinely absent, so *"let alone torch"* survives; the
first clause does not. §7 prices *"a dependency"* in R-A2's cost column and §6
ranks it last partly on *"a trainer that does not exist here"*. Half of that
dependency is installed.

### MAJOR-9. §6's transferable "4 %" is a 3 000-position, L = 7 subsample figure generalised to "this corpus", and it is wrong at every length

§6 closes:

> **on this corpus, through `ω²`, at the window unit, threat structure beyond
> stone counting is worth 4 %**, and any future row that proposes to learn quiet
> structure from these labels is priced against that number.

§2.5's own table says the measurement is *"Window unit, `L = 7`, **3 000
positions**"* — 6.6 % of the population, at a length the same document says is not
the covering length. §6 keeps "through ω²" and "at the window unit" and silently
upgrades the scope to "this corpus".

**Minimal reproducer** (scope disclosed by the subsample itself): at L = 7 the
3 000-position sample gives the enum ω² = 0.005031; the full-population figure in
the receipted census is **0.003573** — 41 % away. So a 4 % gap measured at one
scope was never a full-population statement.

**And at HEAD it is measured.** `/usr/bin/python3 /tmp/claude-1000/rt/r3.py`, over
the whole 45 271 rows:

| L | enum T4 ω² | count-only ω² | gain |
|---|---|---|---|
| 7 | 0.003573 | 0.003525 | **+1.4 %** |
| 9 | 0.004606 | 0.004158 | **+10.8 %** |
| **11** | **0.005504** | **0.004894** | **+12.5 %** |
| 13 | 0.004668 | 0.005600 | **−16.6 %** |

4 % is not the corpus's number at any length, and at the length the matrix prices
(11) the enum beats stone counting by three times the figure the matrix hands
forward. The sentence that carries it prices *"any future row that proposes to
learn quiet structure from these labels"*, which §7.4 explicitly refuses:
*"**It does not transfer to the WINDOW-indexed rows.** `R-A5-TOPK`, `R-A2-L11F`
and `R-A3-L11F+F7` are not quotients of this tuple and are not measured here."*

§2.5 itself says *"a matrix may not rank a row on a number that cannot
discriminate"*. §6 then prices the entire future field on that number.

### MAJOR-10. "MET at all sixteen cells" is one of the two units the instrument prints, and the other straddles §2.5's own disqualifying ratio

**Minimal reproducer**: `/usr/bin/python3 /tmp/claude-1000/rt/purity.py`. The
census prints `window` and `code` for four rungs at four lengths — **32 cells**.

| unit | 16 cells | range |
|---|---|---|
| window | the ones §2 reports | 2.003 – 3.589 |
| code | not reported | **1.704 – 2.806** |

Five code-unit cells sit below 1.9x and **L = 13 T3 is 1.704 — below the 1.77x that
§2.5 calls the value-free quotient's disqualifying pass**. §2 says "at all sixteen
cells" without naming a unit, which reads as exhaustive. `wp22_phase2a_STOP_E.md`'s
own N-9 records the defect — *"the criterion binds one unit while the instrument
prints two"* — under "recorded and NOT repaired", and the matrix imports the
receipts without importing that finding.

Two further STOP_E findings against the enum are absent from the matrix while §2
asserts the contrary of one:

- **N-4**: *"T1's 'expressible only if 1, 2, 3 and 4 stay apart' is false … giving
  `k = 16` and `C(18,3) = **816**` — the imported figure, out of the rung's own
  cited law. What keeps the shipped clip off it is an unjustified extra split."*
  §2's headline is **"`k` is COMPUTED and no cell of it is 816."** That is true of
  the shipped table only because of a split the STOP record calls unjustified. The
  matrix's banner says *"§2's counts stand"* on the ground that no arithmetic error
  was found — but "no cell is 816" is a non-import claim, not an arithmetic one,
  and N-4 denies it.
- **N-5**: T3's clip merges 7 807 `t`-inconsistent patterns, 28 % more than the
  full tuple's 6 099. Absent.

### MAJOR-11. The field's completeness is inherited, and the boundary that excludes the densest affordable row is an unrun theorem the matrix's own §9 lists as open

§8's answer to round-1 M-4 is *"every row `eval_families` §7 lists is present"* —
completeness relative to another document's list. But the matrix was willing to
**add** R-H-EXT, which §7 does not list, and rank it first. So the list is not the
boundary; a judgement is.

**Minimal reproducer**: `sed -n '/### 0.1/,/### 0.3/p'
docs/research/eval_families_2026-09.md`. §0.2 measures, at L = 8 folded:

| | value |
|---|---|
| parameters | **2 920** |
| median observations per cell | **273** |
| cells under ten observations | **24 (0.8 %)** |
| window touches per stone (§A6) | **24**, = **1.33x** |
| positions per parameter | 15.5 (against A1's 1.2) |
| nps floor on the matrix's own model | **403 391 (0.76x)** — R-H-EXT's, 18 % above the codebook rows' |

That row dominates R-A5-TOPK on every column the matrix prices: denser, cheaper in
traffic, higher floor, closed-form, no training seed, no new dependency. It is
excluded solely by §0.1's covering minimum — and `eval_families` §8 tags the
sufficiency question that would settle it: *"THM-WINDOW asks whether a shorter
window still separates structures of different value once summed over overlaps.
**Minutes of compute, not run (D-291)**."*

So a whole row is excluded from a priced field by an estimate its own source marks
as a D-291 finding, and the matrix's §9 lists *"Nothing about `THM-WINDOW`"* as an
open item **without noting that it is what bounds the field**. An omission that
changes a reading.

---

## 4. MINOR findings

**m-1. "§11 is the failed-precedent check" — there is no §11.** Line 31. The
document's sections run §1–§10; the failed-precedent check is §8. A pointer to a
section that does not exist is round-1 M-7's own class (*"a pointer to a section
carrying nothing became a pointer to a section that did not exist"*), in the
paragraph that introduces the section invoking it. Reproducer:
`grep -n '^#\{2,3\} ' docs/experiments/matrix_wp22_phase2_eval.md`.

**m-2. Two "§6.5" references have no referent in this document** (lines 14 and
603). They mean `hex_threat_enum_v1.md` §6.5 and do not say so. The matrix is not
on `tools/governing_citation_check.sh`'s GOVERNING array, so gate 21 does not
catch either of these.

**m-3. "±3.5 %" is tagged MEASURED and its arithmetic is not on the page.** It is
half the max−min of eight draws divided by the mean (3.46 % at K = 32 and K = 64).
The sd is 2.2 % of the mean and the SE of the mean of eight is 0.75 %. Half-range
from n = 8 is a badly-behaved statistic to carry into §4's rows as *"(MEASURED, §3)"*.

**m-4. §1.3 says "There are two committed play seats"; there are three.**
`configs/play_staged_solver_v0.toml` is committed, `mode = "play"`, and differs from
`play_staged_v0.toml` in exactly one key. Its own header calls it a MEASUREMENT
seat rather than a deployment config, so the sentence is defensible — but §1.7's
answers-per-game estimate is taken from `sealbot_anchor_v7_protocol.md`, whose runs
are on that third seat, and §1.3's throughput table does not price it. The spread of
candidate time-matched seats is wider than the 2.8x measured.

**m-5. §1.7 gives two wall costs for the same run, eight lines apart, and
reconciles neither.** Bullet 1: 8 500 openings is **4.83 h**. Bullet 3 quotes
`training_pipeline` §6: *"one acceptance SPRT at Δ = 10 is **2-3 hours** at the
instrument seat"*. The reconciliation is in `book_v3_registration.md` §R1, which the
matrix cites for other things: *"At cap 8000 the instrument measures `mean_pairs
3102.1` — the expected pairs a run actually plays, not the cap."* My instrument
gives 3 114.9. 4.83 h is the cap's cost; ~1.8 h is the expected cost. Neither is
labelled.

**m-6. §1.7 uses the configured 500 ms per answer where §1.3's own receipt has a
measured one.** `nps_play.txt`: 11 502 ms / 24 answers = **479 ms** at `play_v0`,
10 978 / 24 = **457 ms** at `play_staged`. D-291's class, with the measurement
already inside the same document.

**m-7. The arithmetic pass lists two rows under "The 28 that reproduced" whose own
two columns disagree.** `SPSA openings against book_v3 | 7.176x | 7.1x` and
`SPSA wall | 34.67 h | 34.4 h`. Both re-derived values are the pre-correction ones
(61 000 openings); the matrix's are right for 60 500. A "reproduced" table with two
non-matching rows.

**m-8. "sixteen cells" contains a duplicate.** At L = 7 the ladder collapses —
§2's own table gives `k = 25` for both T4 and T3 — and the census returns identical
ω² (0.003573) for both. Fifteen distinct partitions, counted as sixteen.

**m-9. §8's "every instrument here is committed under `tools/hex_enum/`" is false
of three of §10's five.** `measure_nps.sh`, `measure_play.sh` and `confirm_wedge.sh`
exist only inside gitignored `artifacts/` directories and have no test driving the
shipped script — against `docs/process.md`'s coverage rule, *"any tools/ script that
produces a recorded number carries at least one test driving the shipped script …
`tools/` is where such artefacts usually live; living there is not what makes the
rule apply."* `measure_nps.sh` produces §1.2's headline figure, which sets every
registered nps floor (MAJOR-1, MAJOR-3).

**m-10. D-653 is not quoted whole either.** Its flip clause — *"flips if `book_v3`'s
extension makes a holdout redundant"* — is dropped, and §6's *"a second acceptance
book moves R-A2 up"* is that very flip, unconnected.

---

## 5. QUESTIONS

1. **What is the time-matched arm's ALPHA?** The arm is a veto at "not h0", so its
   damaging error is a **false h0** — killing a good row. §1.6 computes the beta
   side (*"fails to fire in 26 %"*) and calls it *"the veto's own failure"*. Nothing
   computes the rate at which the veto wrongly fires on a row that is not slower.
   For a veto that is the number that decides whether the rule is safe.
2. **Is `openings = pairs`?** The whole cap arithmetic assumes one pair per opening.
   I found no committed statement of it and did not verify it in `schedule.rs`. If a
   run may replay an opening, every "cap" in §1.6 moves.
3. **Where is the fitted-v0 row?** WP-2.2 Phase 1 produced six admissible tables at
   18 touches, closed form, zero new capability (D-637). The matrix carries only the
   one-integer slice of that question (R-D-W1) and its kill. The full row — *deploy
   one of the six* — is not in the field. It may be correctly absent under D-704's
   reasoning; the matrix does not say so.
4. **Does D-644's "acceptance-only" bar R-H-EXT from `book_v3`?** §5's R-H-EXT row
   offers *"Δ = 10 on `book_v3`, **or** a large-effect screen"*. `book_v3`'s ledger
   says the R7 acceptance SPRT is *"THE REASON THIS BOOK EXISTS"* and needs all
   8 000 pairs, and D-704 defines R7 as *"the selected family's acceptance SPRT"*.
   If R-H-EXT is not a family, is `book_v3` available to it at all?
5. **What does §7.4's "T4 is the only rung that beats stone counting, and it is the
   rung §7.1 prices at 1.07 observations per nominal parameter" do to the matrix's
   `L = 13` finding?** At L = 13 T4 is now *below* stone counting (0.834). §2's *"`L =
   13` buys nothing"* is unaffected, but §2 item 4's framing of T4's 62 %-of-ceiling
   retention as evidence is.

---

## 6. What I attacked and it SURVIVED

I want to be specific here, because a great deal of this document is right and the
findings above are about what it licenses, not about its arithmetic.

**§1.1 is the best section in the package.** I wrote an entirely independent walk —
my own manifest parser, my own move-string-to-stone decoder with turn-parity
colouring, my own three-axis line enumeration, and R6's predicate re-derived from
D-622's one-sentence statement — and got **89 805 / 74 672 / 45 271 / 60.6265 % /
3 487**, every one exact. The manifest digest matches on both copies. Nothing in
§1.1 moved under a second implementation.

**Every power figure survives a genuinely independent instrument.** Different
language, different RNG family (PCG64 against SplitMix64), different seed,
vectorised rather than sequential. All ten cells within 0.0062 of the printed
value, most within 0.003, against an MC standard error of 0.0021. The instrument
also independently recovers `mean_pairs ≈ 3 100`. The closed-form cross-checks are
real anchors and not self-references: 3 554 is D-628's independently-derived number
and 5 233 is fishtest's `1046535/Δ²` — I checked both by hand. `ceil_to_500(P+500)`
is applied correctly at every one of the twelve cells in §1.6's table, including
the two that are already multiples of 500 — the exact class the author's own
arithmetic pass caught itself on.

**All six receipts verify `sha256sum -c` clean**, and so does the seventh added at
HEAD. That is not nothing: the matrix's whole evidentiary claim is that a gitignored
directory is anchored by a digest in a tracked file, and for five of six directories
that anchor holds exactly.

**§2's census transcription is exact.** I re-derived the `k` ladder, the code
counts, the observed-class counts at both lengths, the medians, the Buro-line
counts, the growth curves, the 62 %-of-ceiling ratio and the L13-vs-L11 comparison,
and every number is what `census_L*.txt` prints. The claim *"every rung's ω² is
LOWER at 13 than at 11"* — which the matrix asserts without showing — holds at
**both** units, 8 cells out of 8. §3's table is `pilot_L11.txt` transcribed to the
digit.

**§1.4 is correct and I tried hard to break it.** `validate_budget` is the first
call in `ArenaConfig::validate`, the refusal is an unconditional `if let` return
with no escape, `validate()` is called on the load path at `config.rs:207`, and the
refusal is pinned by a test in `config_tests.rs`. I checked whether `tools/sealbot`
offers a back door — it runs pistol at movetime and now reports its own engine time
— and it is not a paired A/B SPRT harness, so it does not make a time-matched arm
runnable. §1.4 stands as written.

**The nps bracket model is one formula, applied consistently.** I re-implemented it
from scratch and reproduced both ends of both brackets to the digit
(341 617.3 / 546 003.4 and 403 390.8 / 499 791.3). My objections are to the profile
share it is fed and to the undocumented `c` ranges — not to the model, which is the
right shape for the question.

**§1.8's geometry is right.** `WINDOWS_PER_CELL = Axis::ALL.len() * WINDOW_LEN` is
3 × 6 = 18 at `crates/pistol-core/src/window.rs:6,9`, and 31.77 % / 13.45 % are the
audit's own figures at the receipt it names.

**Every calculus citation resolves at the quoted line.** RULE-EXACT at `:64-66`,
the exact-`t` counters spanning `:153-154`, E-INIT at `:158`, ADOPT-TEMPO at
`:177`, the 2.046 s at `matrix_wp22_quiet_scale.md:252-253`, the 1 040 answers at
`sealbot_anchor_v7_protocol.md:170` and the 100 games at `:393`. I checked each with
`sed -n` at the named lines rather than by search.

**§8's honesty about the round-4 report is real.** `git log --all` over the path and
a full `--diff-filter=A` sweep of every file ever added confirm the tree has only
rounds 1, 2 and 3. Recording that rather than leaving it to be rediscovered is
exactly right, and it is the reason MAJOR-2's dead digest is worth flagging: the
document sets that standard for itself.

**Every row `eval_families` §7 lists is present, killed rows shown with their
kills.** Round-1 M-4's sentence is discharged. My MAJOR-11 is about the list's own
boundary, not about fidelity to it.

**Showing R-A4-CLASS with its stop rather than deleting it was the right call**, and
so was publishing the arithmetic pass with an explicit *"pick different cells than
it checked"* invitation. I did, and the cells it checked were right; the one
mismatch it self-reported was real and correctly fixed. Round 3's §7.4, which I
re-derived line by line from `census_r3`, is exact — sixteen ratios to three
decimals, four class counts, four ceilings, all mine agreeing.

**And the document's central structural act is right.** D-708's separation of
matrix from selection, applied here with a genuine HANDUP and no ADR of selection,
is what makes this review possible at all. The matrix says *"NOTHING IS SELECTED
HERE"* and means it.

---

## 7. VERDICT

# FAIL

Four BLOCKING and eleven MAJOR. None of them is arithmetic — as this project's
record predicts. Every one is about what the document licenses.

### The shortest route to a fix, and it is deletion and correction rather than argument

**Deletions** (nothing is measured to make these):

1. **DELETE** §5's paragraph beginning *"AND ONE THING D-705 DOES NOT SAY…"* and
   §8's *"quoted whole rather than glossed"* clause. Re-point §1.6 point 1 at
   D-705's own asymmetry sentence, as a **confirmation** of the ADR's stated ground
   rather than a finding against it. *(BLOCKING-1)*
2. **DELETE** §3's reading 3 and the two sentences that rest on it — §4 R-A2's *"so
   the kill as written cannot fire cleanly"* and §3's *"which is D-614 arrived at
   from a direction D-614 did not take"*. Replace reading 3 with the paired figure:
   *8 of 8 splits in the same direction, sign-test p = 0.0039; offline loss orders
   these three architectures cleanly, and what does not survive a split is the
   individual weight, which is reading 2.* *(BLOCKING-2)*
3. **DELETE** §5's closing D-653 paragraph. If the 500-pair cap is kept, state it
   as the matrix's own reservation of the detector's slice and quote
   `book_v2_registration.md` §4's *"the Stage-3 detector's own standing slice"* —
   which turns a false finding against an ADR into a true statement of a policy
   choice. *(BLOCKING-3)*
4. **DELETE** *"the only row that can use the screening book"* from §5 and §6.1, or
   measure the claim behind it. *(MAJOR-7)*
5. **DELETE** the numpy clause from R-A2's determinism cell. *(MAJOR-8)*
6. **DELETE** *"and any future row that proposes to learn quiet structure from these
   labels is priced against that number"* from §6, keeping the measurement with its
   real scope. *(MAJOR-9)*

**Corrections** (each is arithmetic already on this page):

7. §1.2 and §1.3 → the receipted numbers, or receipt the runs that produced the
   printed ones. Recompute every floor from whichever survives. *(MAJOR-1)*
8. §1.6's digest → `d82fce37…`. *(MAJOR-2)*
9. §1.8 → **44.70 %**, and every registered floor with it: the codebook floor
   becomes **298 245 (0.56x)**, R-H-EXT's **367 345 (0.69x)**. State the `c`
   brackets' ground or use one bracket for every row. *(MAJOR-3)*
10. §4's density column → one currency, named in the header, with each row's other
    currencies in the cell. *(MAJOR-4)*
11. §7's R-H-EXT cost cell → *"the whole of `book_v2`'s 1 000-opening holdout, on
    which two packages hold standing claims (D-628, `book_v2_ledger.md`), and whose
    last would-be claimant withdrew and put it to the operator."* *(MAJOR-6)*
12. §2 item 4 → name the unit, and print both. Carry STOP_E's N-4 and N-5 beside
    §2's *"no cell of it is 816"*. *(MAJOR-10)*
13. §2, §2.5 and §6's closing → re-point at `hex_threat_enum_v1.md` §7.4.
    *(BLOCKING-4)*

**One addition, and it is the only one:** price the L = 8 folded row, or say in §9
that the field's lower boundary is `THM-WINDOW` and that `eval_families` §8 marks
settling it as minutes of compute not taken. *(MAJOR-11)*

### Would I rank the field differently — and on what

**Yes, and on the matrix's own axis, corrected.**

`R-H-EXT` is not the cheapest row. On "what a row costs before it can be run at
all", it is the **most** expensive: its screen consumes an irreplaceable,
never-labelled, pre-committed 1 000-opening slice that is the only corpus-disjoint
v2 sample and that two packages are waiting on, and the last package to ask for
400 of it withdrew and referred the question to the operator. `R-A5-TOPK` costs a
fit and no openings at all until acceptance.

So on the stated axis: **R-A5-TOPK first, R-H-EXT second and only after the
operator rules on the holdout** — a ruling the matrix should ask for rather than
pre-empt, since §6 is a HANDUP.

Two further changes I would make and would defend:

- **Ranks 3 and 4 are not settled by the tie-breaks §6 declares.** §6 says ties
  break on *"the measured content of §2 and §3"* — §2 is a census of a stopped row
  and §3 is one row's pilot. Neither touches A3-versus-A2. What actually separates
  them is a citation (Stockfish's caveat), a currency error (MAJOR-4), and a false
  dependency claim (MAJOR-8). Strip those and the two rows are unranked.
- **The L = 8 folded row should be priced before the order is handed up.** On every
  column this matrix prices it dominates R-A5-TOPK, and the only thing keeping it
  out of the field is a theorem `eval_families` §8 says costs minutes and nobody has
  run. A field bounded by an unrun estimate is not a priced field.

**The single strongest thing I could not break**: §1.1. Four counts, one
independent implementation, exact. If the rest of the document were held to that
standard this review would have been short.
