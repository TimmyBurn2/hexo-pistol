# WP-1.9b — PRE-REGISTRATION: the flip bench, the landing bench, and the identity leg

**Committed before any of the three runs it governs.** Registered numbers never
move (D-374). A run landing outside a bracket is reported against the bracket; the
bracket is not edited afterwards.

Design under measurement: `wp19b_o3_design.md`, which carries no number (D-483).

---

## 0. The instrument, with its governing revision (docs/process.md)

| Artefact | Revision | Note |
|---|---|---|
| `tools/bench_delta.sh` | `ab369b0` | UNCHANGED since the WP-1.9 runs this package's comparand comes from. Its review carries; no amendment reopens it. |
| `configs/instrument_v0.toml` | `e4bb5bf` | Radius 2 (D-194). Pinned by the script. |
| `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` | `70cc465` | 24 positions, both bands. |

`bench_delta.sh` reads the LIVE tree's config and fixture for both sides (its own
header states this), so the three revisions above are the instrument for every
run below regardless of which revision is being measured.

**No second instrument is registered, and the ground is stated rather than
implied** (docs/process.md, cost/replication rule): no doubt is raised against
`bench_delta.sh` here. It is on D-289's DRIVEN list, its governing revision is the
one WP-1.9's registered runs used and whose review passed, and it carries its own
three self-checks — a per-position node-identity assertion, an IQR gate, and a
refusal of two identical binary digests. Replication is the 5 reps the terms
already fix. What the dry run in §1.3 adds is ATTRIBUTION, against a referent this
session did not produce.

**Cost of the governed runs**, on this workstation: each `bench_delta.sh` side is
a `--release --locked` build in a throwaway worktree, and a 5-rep run over 24
positions at two budgets is on the order of ten minutes after the builds. Three
runs plus the identity leg's two binaries is roughly an hour of machine time and
no operator attention beyond reading the receipts.

---

## 1. The FLIP bench (D-501's terms, quoted; this document adds nothing)

### 1.1 The terms, which may not move

> - Implement a probing table over the same packed key.
> - `tools/bench_delta.sh rev:a5c5661 rev:<O-3> 5`, config
>   `configs/instrument_v0.toml`.
> - Comparand: **O-2 at 1.783 / 1.909** (`wp19/mx-O2`), which does not move.
> - **Flips only if O-3 exceeds those in BOTH bands** by more than the within-run
>   IQR. One band is a finding, not a flip.
> - Never run: the debt stays open. Silence does not discharge it.

`<O-3>` is this package's O-3 revision, which is the storage written INLINE in
`handcrafted.rs` — the comparand `wp19/mx-O2` is an inline revision too, so the
two sides differ in the store and not in where it is written.

### 1.2 What the outcome does, registered before the run

| Outcome | Reading | Consequence |
|---|---|---|
| **A — flips** | O-3 exceeds 1.783 AND 1.909 by more than the within-run IQR | O-3 is the landing candidate; D-501's selection is superseded, not amended |
| **B — one band only, or neither** | a FINDING, not a flip | O-2 is confirmed; the landing candidate is O-2 moved inline |
| **either** | | D-501 is DISCHARGED. Silence was the only non-discharge and this run forecloses it |

The comparand is a number from another session's artifacts and is NOT re-measured
here: re-measuring it would replace a registered comparand with a fresh one, which
is the bracket move D-374 forbids.

### 1.3 Dry run (docs/process.md dry-run discipline)

**Input, of the same KIND as the registered workload and differing only in
identity:** `tools/bench_delta.sh rev:wp19/mx-base rev:wp19/mx-O1 1` — two
revisions of this engine differing only in the eval's window store, which is what
the registered workload is, with O-1 in the candidate seat instead of O-3 and one
rep instead of five. It is not the registered workload and does not consume the
first governed run.

**Criterion, with the defect class it excludes.** The defect class is
MIS-ATTRIBUTION: the command as spelled measuring something other than the pair of
revisions it names — sides resolved in the wrong order, a stale or wrong binary
measured, or a silently substituted position set. The criterion is an EXTERNALLY
DERIVED REFERENT, not an internal-agreement property: the run's nps ratio must
land within **±0.08 in each band** of O-1's already-recorded pair **1.198 / 1.242**
(`artifacts/wp19_mx_bench_O1_fmt_v1.txt`, another session's run at the same
revision on the same instrument), with per-position node identity holding and exit
0. A swapped-sides defect would answer near 0.83, a same-binary defect is refused
by the script's digest check, and a substituted fixture moves the ratio off the
referent — none of the three can pass this.

**Registered consequence of a miss:** the flip run is NOT taken. The disagreement
is recorded, the package STOPs, and the instrument or the machine is the subject
of the next session, not O-3.

---

## 2. The LANDING bench (registered here BEFORE the run, D-483)

The winner of §1, landed inline, measured against the revision it replaces.

- **Instrument:** `tools/bench_delta.sh rev:3c9e28b rev:<landed> 5`, config
  `configs/instrument_v0.toml`.
- **Baseline `3c9e28b`:** the shipped WP-1.9 module version. `dev` at this
  package's start is `e299b0e`, which is `3c9e28b` plus two documentation-only
  commits; `3c9e28b` is named because it is the revision the closure and D-502
  cite.
- **Direction:** the landed inline winner is FASTER in BOTH bands. A ratio at or
  below 1.00 in either band is the abort.

### 2.1 The brackets

| Outcome | Bracket, both bands | Ground |
|---|---|---|
| **B — O-2 inline** | **[1.10, 1.30]** | the inverse of the head-to-head inline-vs-module pair 0.844 / 0.828 in `artifacts/wp19_bench_inline_vs_module_v1.txt`, which is 1.185 / 1.208, widened for run-to-run drift |
| **A — O-3 inline** | **[1.10, 1.30] does NOT apply.** The bracket is a COMPOSITION criterion: the landing ratio must equal the flip ratio divided by the shipped ratio (1.508 early, 1.579 late, D-502) to within **±0.05 per band**, AND be at least **1.10** in both bands | the three WP-1.9 runs composed to within 0.002, so ±0.05 is the drift allowance and not a fitted margin |

**Abort, both outcomes:** a ratio **< 1.00** in either band. The landed inline
winner would then be slower than the module version it replaces, which contradicts
the measurement this package exists to recover. Consequence: the landing does NOT
go to `dev`, the package STOPs, and the finding is that the inline-vs-module cost
did not reproduce.

**Below bracket but above abort** is a FINDING, exactly as WP-1.9's registration
read its own miss: the record says how much of the measured cost the landing
recovered, and no bracket moves.

**Time-to-depth is the declared cross-check and not independent evidence** —
nodes-to-depth are identical by search identity, so its ratio is the nps ratio over
the depth-2 node mix. Rule 5 requires both and it is reported.

`bench_delta.sh`'s own printed `VERDICT` is against ITS `[1.4, 2.5]` thresholds,
which descend from D-220's package and are not this document's brackets. Its
wording is quoted with the numbers (D-327) and read against the table above.

---

## 3. The IDENTITY leg (Track E, D-495), and why no SPRT is owed

The landing is a container swap behind an unchanged `Eval` seam, so the claim is
BIT-IDENTITY and the oracle is stronger than a strength run: D-495 says a package
proving identity takes NO SPRT, and a single mismatch flips it to the full SPRT
track mechanically.

**Procedure, the WP-1.9 one verbatim** (`artifacts/wp19_byte_identity_v2.txt`
records its shape): two `--release --locked` binaries built in their own detached
worktrees — baseline `3c9e28b`, candidate the landed revision — run over 44
positions (`tactical_staged_v0.txt` 20 under `configs/tactical_staged_v0.toml`,
`bench_positions_v1.txt` 24 under `configs/instrument_v0.toml`) at both
determinism budgets (`depth_turns 4`, `nodes 200000`), `nps` and `time` elided
because both measure the machine, everything else compared.

**Criterion:** the two normalised transcripts are IDENTICAL and share one sha256
digest, with a positive-content check — one `bestmove` per `go`, zero `error`
lines — so that two mutually-refusing runs cannot pass as agreement. The defect
class excluded is an eval answer that changed somewhere the unit suite does not
reach; a same-binary comparison would pass vacuously, so the artifact records BOTH
revisions' binary digests and they must differ.

**Any mismatch = STOP.** No argumentation, and the landing is wrong.

---

## 4. What this package does NOT register

No strength claim, no SPRT, no arena run, no book. Nothing here consumes
`random_openings_v1.txt`, which D-505 retired for governed use.
