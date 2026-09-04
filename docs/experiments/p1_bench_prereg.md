# P1 — PRE-REGISTRATION: the landing bench, the identity leg, and the oracle gates (revision 3)

**THE GATE'S HISTORY IS HERE, AND BELOW IT APPEARS ONLY WHERE A REGISTERED
TERM'S SHAPE IS THE FINDING THAT PRODUCED IT.** Round 2 found this document
narrating its own rounds in six places, and the one pair of narrations that
diverged was a self-contradiction about what a landing does (its B3), so the
sections below state what is registered, this paragraph states how it got there,
and a section names a round only where a reader would otherwise re-introduce the
defect. **Round 1** (`p1_bench_prereg_REVIEW.md`): FAIL on B1 — §1.2's dry run
was registered and never taken, so `rev:` mode, the mode the only banked number
comes from, had never been exercised here — and B2, an instrument table missing
two files the run reads; with M1 (one bracket for two bands whose grounds
differ), M2 (a receipt that could not print `idle`), M3 and ten minors.
**Round 2** (`p1_bench_prereg_REVIEW_rev2.md`): every one of those fifteen
verified discharged by execution, and FAIL on four new ones — B3, the
contradiction above; M4, a recorded block digest that was of an extraction
wrapper rather than of the block; M5, a drift figure the newly added run itself
falsifies; M6, an ADR line citing the withdrawn bracket — with nine minors.
Revision 3 takes all thirteen.

**Committed before any run it governs.** Registered numbers never move (D-374).
A run landing outside a bracket is reported against the bracket; the bracket is
not edited afterwards. Design under measurement: `p1_design.md` (no numbers,
D-483); option measured: `matrix_P1_threat_state.md` §4.1's O-E.

**Slots filled at launch and nowhere else (D-427)**: `<landed>` is the SHA of
the landing commit on `dev`, and the two binary digests the runs print are
recorded in the results document beside it. Nothing else in this document is a
slot.

---

## 0. The instrument, with its governing revisions (docs/process.md)

| artefact | revision | note |
|---|---|---|
| `tools/bench_delta.sh` | `ab369b0` | UNCHANGED since WP-1.9b's registered runs (`wp19b_bench_prereg.md` §0 names the same revision); its review carries |
| `configs/instrument_v0.toml` | `e4bb5bf` | pinned by the script |
| `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` | `70cc465` | 24 positions, both bands |
| `configs/eval_v0_weights.toml` | `64c336f` | READ LIVE by the harness (`bench_delta.sh:93`, digested at `:272`) and named by every seat config's `weights_file`; a change here moves every nps number and every `bestmove` line (B2) |
| `tools/ci.sh` | `9390b48` | §3's gate numbers and its `GATE_TOTAL=20` are facts about this revision |
| `configs/tactical_staged_v0.toml` | `e4bb5bf` | the identity leg's second seat |
| `configs/gate_staged_solver_v0.toml` | `e4bb5bf` | the identity leg's third seat, the solver armed — `tools/determinism.sh:76`'s own |
| `crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt` | `538b3e5` | 20 cases, keyed: its positions are its `position` lines |
| `tools/determinism.sh` | `7fbc1ff` | its `:76` (the solver-armed seat and its budgets) and `:204` (how a keyed fixture's positions are read) are cited by §2; run as §3's gate 9 |
| `tools/tactical_check.sh` | `0a80a7b` | §3's gate 8 |
| `tools/search_oracle_check.sh` | `55e4b56` | §3's gate 10 |
| `tools/staged_soundness_check.sh` | `8ce13ff` | §3's gate 11 |
| `tools/solver_oracle_check.sh` | `e668dfa` | §3's gate 12 |
| `tools/solver_determinism.sh` | `3916afd` | §3's gate 13. All six gate scripts are named because §0's completeness sentence reaches §3, and each revision is the answer of `git log -1 --format=%h ffc5c10 --` for that file, and not a hand-written prefix |
| the identity leg's command block, §2.1 | this document's own revision, extracted and digested as §2.1 registers | a command block a document prints is an instrument and is named with its revision (docs/process.md) |

**ONE FIXTURE SET, AND THE REASON IS THE INSTRUMENT'S.** The arc dispatch's
perf rule asks for *"both bands, both fixture sets"*, and `bench_delta.sh` has
exactly one fixture compiled into it (`FIXTURE=…/bench_positions_v1.txt`,
`:94`), split into the two bands at `EARLY_MAX=17`. The bands are registered;
the second fixture set the dispatch names is not reachable through this
instrument and is not registered here. It IS exercised, for identity rather than
for speed, as the identity leg's first and third seats (§2).

**THIS DOCUMENT IS NOT ON GATE 20's LIST**: `tools/governing_citation_check.sh`
names the governing documents whose citations CI re-checks, and this one is not
among them, so its `file:line` citations are checked by its reviewers and by
nothing mechanical. Adding it is a commitment this package does not make while
the document is still moving; the closure may.

`bench_delta.sh` reads the LIVE tree's config and fixture for both sides (its
own header), so the revisions above are the instrument for every run below
regardless of which revision is being measured.

**No second instrument is registered, and the ground is stated**: no doubt is
raised against `bench_delta.sh` — it is on D-289's DRIVEN list, its revision is
the one whose review passed for WP-1.9 and WP-1.9b, and it carries its own
three self-checks (per-position node identity under both budgets in every rep,
the IQR gate, the refusal of two identical binary digests). What replication
this document has, and what it does not, is §1.2's paragraph on drift: the 5
reps bound the noise WITHIN a run and are what the IQR gate reads, and the
between-run spread the brackets have to survive is measured there over every run
of the pair. What §1.2's dry run adds beyond either is ATTRIBUTION, against a
referent the script's `rev:` path did not produce.

**Cost of the governed runs**, on this workstation: a `rev:` side is a
`--release --locked` build in a throwaway worktree (the matrix's builds took
under a minute each on a warm cache, a cold one is several minutes), and a
5-rep run over 24 positions at two budgets took about a minute in the matrix
runs. §1.2's dry run and §2.4's are TAKEN and cost about
six minutes between them; what remains unspent is the landing bench, the
identity leg and its falsifier, under twenty minutes together and no operator
attention beyond reading the receipts. The six CI gates in §3 are the ordinary
CI cost.

---

## 1. The LANDING bench (registered BEFORE the run, D-483)

- **Instrument:** `tools/bench_delta.sh rev:ffc5c10 rev:<landed> 5`, config
  `configs/instrument_v0.toml`.
- **Baseline `ffc5c10`:** arc III's closing commit, the revision the arc began
  at and the one every P1 measurement names as baseline.
- **Direction:** the landed revision is FASTER in BOTH bands.
- **Seat:** the instrument seat only. The matrix (§6, attack 1) records an
  exploratory measurement under `configs/bench_wp18c_solver_on.toml` — one of
  the three committed configs that arm the solver — of about two per cent early
  and about none late; this document registers no bracket at any solver-armed
  seat and the package claims no gain there. The identity leg's third seat is a
  different one of the three, `configs/gate_staged_solver_v0.toml`, chosen
  because `tools/determinism.sh:76` already runs it (m4).

### 1.1 The bracket

| band | bracket, nps ratio | ground |
|---|---|---|
| early | **[1.207, 1.307]** | the matrix's O-E at `4298ecd`, **1.257** (`artifacts/p1_mx_bench_E_v1.txt`, sha256 `d0b1b384…`; the round-1 red team's idle re-measurement **1.256**, `artifacts/p1_rt_round1/bench_E.txt` `e3eaed80…`), ± **0.05** for run-to-run drift — five times the widest spread three runs of this byte-identical pair have shown (0.010, §1.2), and the allowance WP-1.9's landing registered — and because the landing differs from the prototype by a module split, tests and docs and by no mechanism |
| late | **[1.190, 1.290]** | the matrix's O-E, **1.240** (re-measured **1.241**), same artifacts, the same ±0.05 applied to the LATE band's own ground |

Each bracket is its own band's ground ± 0.05, unrounded. The maintenance share
the gain comes out of is AT MOST 35.67 % of wall (the matrix §2.1: the profile
cannot split query-side `masks` reads from the maintenance), which is why the
bracket is grounded on the measured whole-engine ratio and not derived from the
share.

**THE DISPOSITION: EACH BAND IS READ AGAINST ITS OWN BRACKET, AND THE PACKAGE
TAKES THE MORE SEVERE OF THE TWO ROWS.** Severity, most severe first: **ABORT**,
**ABOVE BRACKET**, **BELOW BRACKET**, **PASS**. Nothing here compares the two
bands' ratios to each other — their brackets have different floors, so a lower
ratio is not the worse result, and an earlier revision's three-way sentence
(*worst band*, *lower ratio*, *lower row*) came apart on exactly that (round
3's B4).

| nps ratio | disposition |
|---|---|
| **< 1.10** | **ABORT.** The landing does not go to `dev`, the package STOPs (the dispatch's STOP protocol: this package, not the arc), and the number is the finding. |
| **1.10 to below the band's bracket** | **BELOW BRACKET — a FINDING, and it LANDS.** The results document states what fraction of the prototype's gain the landing kept and accounts for the gap; a gap it cannot account for is a further finding and not a STOP. |
| **inside the band's bracket** | **PASS.** |
| **above the band's bracket** | **ABOVE BRACKET — a FINDING TO EXPLAIN.** Something other than the registered mechanism changed between `4298ecd` and `<landed>`; the results document names it, or the package STOPs. |

**No interval has two dispositions and none is unassigned**, which is what
round 2's B3 found wrong here: an earlier revision said of `[1.10, 1.15)` both
that it lands and that it STOPs, and left `[1.15, 1.190)` unsaid.

**THE HARNESS PRINTS ITS OWN VERDICT AND IT IS NOT THIS TABLE.**
`tools/bench_delta.sh:452` prints `VERDICT ABORT` below **1.15** and `:453`
prints `BELOW-BRACKET` below **1.4**, against D-220's thresholds and not this
package's. **Both of this document's brackets lie entirely below 1.4**, so the
harness will print `BELOW-BRACKET` or `ABORT` on every outcome this document
calls a PASS — as it did on all three runs of the prototype pair already
recorded here. The table above governs the disposition and the harness's line is
quoted beside the number, never read as a verdict on this package (D-327). **What that interval
means in the prototype's own terms, since the point of the row is that it is a
real result and not a near-miss**: a landing there keeps between **38.9 %** and
**62.5 %** of the gain the prototype measured (`0.10/0.257` to `0.15/0.240`), so
it is a change that works and under-performs, which the record can carry.

**Time-to-depth** is the declared cross-check and not independent evidence —
nodes-to-depth are identical by search identity, so its ratio is the nps ratio
over the depth-2 node mix. Rule 5 requires both and both are reported, with the
script's printed deviation.

`bench_delta.sh`'s own printed `VERDICT` lines are against ITS `[1.4, 2.5]` /
`1.15` thresholds, which descend from D-220's package and are not this
document's brackets. They are quoted with the numbers (D-327) and read against
the table above.

### 1.2 Dry run (docs/process.md dry-run discipline)

**Input, of the same KIND as the registered workload and differing only in
identity:** `tools/bench_delta.sh rev:ffc5c10 rev:4298ecd 5` — two revisions of
this engine differing only in the threat state's store, with the matrix's
measurement revision `p1/mx-E` in the candidate seat instead of the landing.
It is not the registered workload (no landing exists yet) and does not consume
the governed run.

**Criterion, with the defect class it excludes.** The defect class is
MIS-ATTRIBUTION: the command as spelled measuring something other than the two
revisions it names — a wrong or stale build in a `rev:` worktree, sides in the
wrong order, a substituted position set. The referent is EXTERNALLY DERIVED —
the matrix's PATH-mode number for the same source, **1.257 / 1.240**, produced
from a binary the script's `rev:` path never built — and the criterion is that
the dry run's nps ratio lands within **±0.03 of it in each band**, with node
identity holding and exit 0. A swapped-sides defect answers near 0.80, a
same-binary defect is refused by the digest check, a stale build lands off the
referent; none can pass.

**THE GROUND FOR ±0.03, MEASURED OVER EVERY RUN OF THIS BYTE-IDENTICAL PAIR
THERE IS.** Three runs exist — the matrix's (1.257 / 1.240), the round-1 red
team's idle re-measurement (1.256 / 1.241) and §1.2's own (1.258 / 1.231) — so
the observed between-run spread is **0.002 early and 0.010 late**, and ±0.03 is
three times the wider of them. An earlier revision put that spread at 0.001 in
both bands, which the third run falsified in the same section that added it
(round 2's M5). **Between-run drift is the only drift these allowances are
about, and the 5 reps cannot see it**: the reps bound the noise within one run
and the IQR gate withholds a band whose spread exceeds 10 % of its median, while
what a bracket has to survive is the machine on a different hour.

**Registered consequence of a miss:** the landing bench is NOT taken. The
disagreement is recorded, the package STOPs, and the instrument or the machine
is the next subject, not the landing.

**Artifact:** `artifacts/p1_dryrun_rev_mode_v1.txt`, sha256 `a2f20ad5…` (m5).

**THE RESULT, and the criterion holds on both bands.** Idle receipts `idle`
before and after; the script resolved and built both sides itself
(`rev:ffc5c10 -> ffc5c10f4d16356f574e3221a023f78399d2e3bb`,
`rev:4298ecd -> 4298ecd332b3b61c567a593a9eaaa8ae69eeb380`); node identity held
per position, both budgets, all five reps; exit 0.

| band | this run | referent | difference | criterion ±0.03 |
|---|---|---|---|---|
| early | **1.258** | 1.257 | **0.001** | holds |
| late | **1.231** | 1.240 | **0.009** | holds |

**AND THE RUN SETTLES SOMETHING THE MATRIX COULD ONLY ASSERT.** The two
binaries the script BUILT ITSELF, in throwaway worktrees from named revisions,
hash to `78a7600adcf0…` and `f333d1010f52…` — **the same two digests the
hand-built binaries carry**, which every number in the matrix was taken on. The
matrix's own fourth listed attack (*"binaries tied to sources by this session's
word"*) is closed here by the instrument rather than by argument, and the
attribution defect class this dry run exists to exclude is excluded on the
mode the landing will actually use.

### 1.3 The idle receipt

**THE LINE**, written inline so that §2.1's is the only fenced `sh` block this
document prints and its extraction is unambiguous (round 2's M4):
`receipt() { local out rc=0; out="$(pgrep -af '[c]argo|[r]ustc|[b]ench_delta|target/[r]elease/pistol')" || rc=$?; case $rc in 0) printf '%s\n' "$out" ;; 1) echo idle ;; *) echo "RECEIPT BROKEN: pgrep exited $rc" ;; esac; }`
— and §2.1's block prints the same three-way form.

**WHAT EACH PART IS FOR.** The bracket spelling: `pgrep -af` matches the full
command line of the shell that invokes it, so an unbracketed pattern reports
itself and can never print `idle`; `[c]argo` matches the string `cargo` and not
the literal `[c]argo` in the invoking shell's own command line. The three-way
`case`: `|| echo idle` prints `idle` for *no match* and for *no `pgrep`* alike,
which is `tools/SHELL_CHECKLIST.md`'s exit-0-wrong-answer shape — a receipt
whose output alphabet cannot distinguish "the box is quiet" from "I could not
ask" (round 2's n4). Exit 1 is the quiet box; anything else says so. **And the
capture is `out="$(…)" || rc=$?` rather than a bare call**: the block runs under
`set -euo pipefail`, where a bare `pgrep` returning 1 — *the quiet box* — kills
the script before the `case` can read it. That is not hypothetical; it is what
the first execution of this spelling did, exiting 1 after printing
`RESULT: IDENTICAL`, and it is `SHELL_CHECKLIST.md` item 12's shape in the one
place this document could least afford it: the receipt that says whether the
measurement is trustworthy.

**THE RECEIPT IS EVIDENCE AND NOT A GATE, AND IT MATCHES COMMAND LINES.** A
match may be a build, or it may be a shell whose command line merely names one
— round 2's run of the registered pattern returned a `bash -c` line containing
the words `cargo test`. So VOID is a judgement the reader makes from the
receipt, not a status the receipt returns, and it applies to **TIMING runs
only**: §1.2's dry run and §1.1's landing bench. **The identity leg is not a
timing run** — it compares bytes, and a busy box cannot change a `bestmove` —
so §2.1's block prints the receipt as context and a non-idle receipt there voids
nothing (round 2's n5).

A run whose receipt shows another build, test or engine process alive — **this
box is shared, and a run of an unrelated project was observed on it while this
document was being written** — is VOID and re-taken (D-592; the matrix's
round-1 red team found two of the session's matrix runs launched seconds after a
test link and re-measured them idle). A VOID spends nothing and moves no
bracket. Each run's receipt is printed into that run's own artifact:
`artifacts/p1_dryrun_rev_mode_v1.txt` for §1.2, `artifacts/p1_landing_bench_v1.txt`
for the landing bench, `artifacts/p1_identity_landing_v1.txt` for the identity
leg — the three names, in one place, and §2.2 no longer carries two of them.

---

## 2. The IDENTITY leg (D-495), and why no SPRT is owed

The state is a cache of the board behind an unchanged query surface, so the
claim is BIT-IDENTITY of search output **over the 128 searches this section
registers, at the three seats it names** — that is the ground D-495's no-SPRT
exemption rests on here, and neither the results document nor the ADR widens it
to search output simpliciter (m8). D-495: a package proving identity takes
NO SPRT, and a single mismatch flips it to the SPRT track mechanically. The
dispatch's STOP protocol: a byte-identity mismatch in tranche 1 is a STOP of
this package.

### 2.1 The procedure — WP-1.9's shape (`artifacts/wp19b_byte_identity_v2.txt`), as a command block

**HOW THE BLOCK IS EXTRACTED, REGISTERED because a document that prints a
command must say which bytes are the command** (round 2's M4, which found a
recorded digest that belonged to an extraction wrapper rather than to the block).
It is the ONLY fenced `sh` block in this document, so:

`sed -n '/^```sh$/,/^```$/p' docs/experiments/p1_bench_prereg.md | sed '1d;$d'`

and the digest below is of exactly that output. A revision that adds a second
`sh` fence must re-register this command.

Two `--release --locked` binaries, each built in its own detached worktree on
`/home` with its own `CARGO_TARGET_DIR` — baseline `ffc5c10`, candidate
`<landed>` — copied out as `$BASE` and `$CAND`. Three seats: WP-1.9's two, and
the committed solver-on seat `tools/determinism.sh` itself runs
(`configs/gate_staged_solver_v0.toml` over `tactical_staged_v0.txt` at that
gate's own budgets, `depth_turns 2` and `nodes 10000`, `determinism.sh:76`),
because the solver is the other caller of the contract this package narrows.
Then, from the live tree:

```sh
set -euo pipefail
# Run from the repository root: the SEATS heredoc names its configs and fixtures
# by relative path. Transcripts go to a scratch directory, never to the tree —
# `out.BASE`/`out.CAND` in the root are two untracked files that `.gitignore`
# does not name and `tools/artifact_check.sh` only sees once `git add -A` has
# already committed them, one step before the landing commit (m3).
OUT="$(mktemp -d)"
receipt() { local out rc=0; out="$(pgrep -af '[c]argo|[r]ustc|[b]ench_delta|target/[r]elease/pistol')" || rc=$?; case $rc in 0) printf '%s\n' "$out" ;; 1) echo idle ;; *) echo "RECEIPT BROKEN: pgrep exited $rc" ;; esac; }
receipt
# A fixture's positions, as the two fixture grammars spell them. A KEYED
# fixture (tactical_staged_v0.txt: `case` / `config` / `position` / `budget`
# / `expect` lines) states one `position` line per case, and that line IS a
# protocol line — tools/determinism.sh:204 reads it the same way and
# cross-checks the count against the cases. A BARE fixture
# (bench_positions_v1.txt) states one `position`-verb tail per line with
# commentary from ` #` on, which is how tools/bench_delta.sh reads it.
entries() {
  if grep -q '^case ' "$1"; then
    local n c; n="$(sed -n 's/^position //p' "$1" | wc -l)"; c="$(grep -c '^case ' "$1")"
    [ "$n" -eq "$c" ] || { echo "REFUSED: $1 has $n position lines for $c cases" >&2; exit 1; }
    sed -n 's/^position //p' "$1"
  else
    grep -v '^#' "$1" | grep . | sed 's/ #.*//'
  fi
}
for side in BASE CAND; do
  bin="${!side}"
  : >"$OUT/out.$side"
  while read -r config fixture b1 b2; do
    for budget in "${b1/-/ }" "${b2/-/ }"; do
      { entries "$fixture" | while IFS= read -r entry; do
          printf 'newgame\nposition %s\ngo %s\n' "$entry" "$budget"; done; echo quit; } \
        | "$bin" --config "$config" | sed -E 's/ nps [0-9]+ time [0-9]+//' >>"$OUT/out.$side"
    done
  done <<'SEATS'
configs/tactical_staged_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt depth_turns-4 nodes-200000
configs/instrument_v0.toml crates/pistol-cli/tests/fixtures/bench_positions_v1.txt depth_turns-4 nodes-200000
configs/gate_staged_solver_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt depth_turns-2 nodes-10000
SEATS
done
sha256sum "$BASE" "$CAND"
for side in BASE CAND; do
  printf '%s lines %s bestmove %s error %s\n' "$side" "$(wc -l <"$OUT/out.$side")" \
    "$(grep -c '^bestmove ' "$OUT/out.$side")" "$(grep -c '^error ' "$OUT/out.$side" || true)"
done
sha256sum "$OUT/out.BASE" "$OUT/out.CAND"
if cmp -s "$OUT/out.BASE" "$OUT/out.CAND"; then echo "RESULT: IDENTICAL"; else echo "RESULT: MISMATCH"; diff "$OUT/out.BASE" "$OUT/out.CAND" | head -40; fi
receipt
echo "transcripts kept at $OUT"
```

44 positions at the two WP-1.9 seats (20 + 24) under the determinism gate's
budgets (`depth_turns 4` and `nodes 200000`) plus 20 at the solver-on seat
under its own (`depth_turns 2` and `nodes 10000`) — `(20 + 24 + 20) × 2` =
**128 searches a side**; `nps` and `time` elided because both measure the
machine, everything else compared. **The block above was executed before this
revision was committed** (the matrix's round-3 red team executed the previous
spelling on a stub and found it read the keyed fixture's 115 lines as
positions, R3-M1; the spelling above reads 20, and §2.4 records the run).

### 2.2 Criterion

The two normalised transcripts are IDENTICAL and share one sha256, with the
positive-content check — **128 `bestmove` lines and 0 `error` lines on each
side** — so two mutually-refusing runs cannot pass as agreement; and the two
BINARY digests differ, so a same-binary comparison cannot pass vacuously. The
defect class excluded is a threat-state answer that changed somewhere the unit
suites and the oracle do not reach.

**Any mismatch = STOP.** No argumentation, and the landing is wrong.

**Artifact:** `artifacts/p1_identity_landing_v1.txt`; the three artifact names
are listed together in §1.3.

### 2.3 The leg's falsifier (its own dry run)

A leg whose answer is known before it runs is not exercised by running it on
the landing. Before the governed leg, the same command block is run with a
DELIBERATELY BROKEN candidate — the design's mutant M1 (the window read's shift
off by one), built in a throwaway worktree from `<landed>` — and must print
`RESULT: MISMATCH`. That run's output is recorded beside the governed leg's.
The defect class it excludes is a leg that cannot fail: a normalisation that
elides too much, a comparison of a file with itself, a fixture that asks
nothing. **Registered consequence:** the falsifier must print `RESULT: MISMATCH`.
**ANYTHING ELSE IS A DEFECTIVE LEG** — `RESULT: IDENTICAL`, or neither line,
which is a real outcome and not a quibble: the block runs under `set -euo
pipefail`, so a deliberately broken build that panics inside the pipeline kills
the script before either line is printed (m2). On any of those the governed leg
is not read and the package STOPs.

### 2.4 The block's dry run (docs/process.md dry-run discipline)

**Input, of the same KIND as the governed leg and differing only in
identity:** the §2.1 block with `BASE` = `pistol-ffc5c10` (`78a7600a…`) and
`CAND` = the matrix's measurement binary `pistol-E` (`f333d101…`, branch
`p1/mx-E` at `4298ecd`) — two binaries differing only in the threat state's
store, the prototype in the candidate seat instead of the landing. Not the
governed leg, and it consumes nothing.

**Criterion, with the defect class it excludes:** each side prints **exactly
128 `bestmove` lines and 0 `error` lines**, and the two binary digests differ.
The defect class is a block that asks the wrong questions — the previous
spelling read the keyed fixture's 115 lines as positions and sent 508 searches,
460 of them refused (`matrix_P1_threat_state_REDTEAM_round3.md` R3-M1) — and a
count of 128 is exactly what a keyed fixture read as its `position` lines plus
a bare one read as its entries yields, `(20 + 24 + 20) × 2`; a wrong reader
cannot land on it.

**Registered consequence of a miss:** the identity leg is not run at the
landing, the block is the subject of the next session rather than the store, and
the package STOPs (m1).

**Taken, before revision 1 was committed:**
`artifacts/p1_identity_dryrun_E_v1.txt` — `BASE lines 533 bestmove 128 error
0`, `CAND lines 533 bestmove 128 error 0`, digests `78a7600a…` ≠ `f333d101…`,
normalised transcripts both `284f70af…`, `RESULT: IDENTICAL`, exit 0. The
criterion holds, and this gate's round 1 re-ran the same block independently and
reproduced both the block's own digest and the transcript digest `284f70af…`.
**EVERY REVISION OF THE BLOCK IS RUN BEFORE THE REVISION THAT PRINTS IT IS
DISPATCHED, AND THE DIGEST BELOW IS OF THE BLOCK ITSELF** — extracted by the
command §2.1 registers, not by a wrapper, which is what round 2's M4 found wrong
in revision 2's receipt.

**Revision 3's block, TAKEN:** `artifacts/p1_identity_dryrun_E_v3.txt`, sha256
`09b615f2…`; block sha256 `2755b900…`; receipts **`idle`** at both ends;
`BASE lines 533 bestmove 128 error 0`, `CAND` the same; binary digests
`78a7600a…` ≠ `f333d101…`; both normalised transcripts `284f70af…`;
`RESULT: IDENTICAL`; **exit 0**; and `git status --porcelain` unchanged across
the run, which is the m3 hazard checked rather than argued.

**THE TRANSCRIPT DIGEST HAS NOT MOVED ACROSS THREE SPELLINGS OF THE BLOCK AND
TWO INDEPENDENT PARTIES** — revision 1's run, this gate's round-1 reviewer, its
round-2 reviewer, and revision 3's run all produce `284f70af…` — so what the
block asks is invariant under the rewrites, which changed only where output goes
and how the receipt fails.

**AND THE FIRST EXECUTION OF REVISION 3's RECEIPT FAILED, WHICH IS WHY THE
SPELLING CHANGED AGAIN**: it exited 1 after printing `RESULT: IDENTICAL`,
because a bare `pgrep` returning "no match" under `set -euo pipefail` kills the
script. §1.3 carries the fix and the reason. A registered block is run, not
read. (That the prototype is byte-identical to `ffc5c10` over the
128 searches is a preview of the governed leg's expected answer and not the
leg: the leg runs at `<landed>`.)

---

## 3. The oracle gates at `<landed>`

The dispatch requires the solver's oracle gates re-run because the solver reads
this state. At `<landed>`, each of these SIX prints its own gate line in
`tools/ci.sh`'s log and the results document cites those lines, never the
wrapper's exit status: `tactical fixture` (gate 8), `determinism` (gate 9),
`search oracle` (gate 10), `staged soundness` (gate 11), `solver oracle`
(gate 12), `solver determinism` (gate 13). The full `tools/ci.sh` at `<landed>`,
twenty gates, is the closure's receipt.

---

## 4. What this package does NOT register

No strength claim, no SPRT, no arena run, no book slice. Nothing here draws from
`random_openings_v2.txt`'s reserved holdout (`docs/book_v2_ledger.md`).
