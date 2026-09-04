# P3 — REGISTRATION AND RESULTS: codegen flags, portable only

**Registered before the runs it governs.** D-577 licensed this package and set
its bounds in the architect's own words: *"portable flags only: LTO,
codegen-units, opt-level; `-C target-cpu=native` and PGO EXCLUDED from pinned
instrument binaries because a pinned binary must reproduce on any host"*. D-14
deferred codegen tuning *"until there is a bench to judge it by"*; that bench is
`tools/bench_delta.sh` and the condition is met.

## The starting point, read off the tree

`[profile.release]` at `1f60f87` sets `overflow-checks = true` and nothing else,
so the build runs at Rust's defaults: **`lto = false`, `codegen-units = 16`,
`opt-level = 3`**. `opt-level` is therefore already at its maximum and only the
first two are on the table.

**THE CLASSIC HAZARD IS ABSENT, AND IT IS CHECKED RATHER THAN ASSUMED.** LTO and
codegen-units can change floating-point results by permitting reassociation
across a boundary that previously separated them. `git grep -c 'f32\|f64'` over
the four crates on the search path returns exactly one file,
`crates/pistol-solver/src/bin/solver-cost.rs`, a cost-reporting binary that no
search links. So there is no floating-point on any path this package can reach,
which is why byte-identity is a REQUIREMENT here and not a hope.

## The variants, and why these

| variant | flags | why |
|---|---|---|
| base | the tree's own | the comparand |
| cgu1 | `codegen-units = 1` | one unit lets the optimiser see the whole crate; the cheapest of the three to build |
| thin | `lto = "thin"`, `codegen-units = 1` | cross-crate inlining at a build cost the project can pay on every landing |
| fat | `lto = "fat"`, `codegen-units = 1` | the whole-program form, the upper bound on what this axis offers |

Built by overriding the profile from the environment, so measuring a variant
edits no committed file. Only the winner's flags are written into `Cargo.toml`.

## What is registered, before the runs

- **Instrument:** `tools/bench_delta.sh` at `ab369b0`, PATH mode over the four
  binaries, 5 reps, idle receipts, `configs/instrument_v0.toml`.
- **EXPECTATION (ESTIMATED):** an engine whose hot path spans four crates —
  `pistol-core`'s window enumeration, `pistol-eval`'s delta, `pistol-solver`'s
  store, `pistol-search`'s loop — is the shape cross-crate inlining helps most.
  **1.05 to 1.20 nps for the LTO variants, 1.00 to 1.08 for `cgu1` alone.**
- **ACCEPTANCE:** the winner is the variant with the highest nps ratio in BOTH
  bands whose **search output is byte-identical** and whose build cost the
  project is willing to pay on every landing. A variant faster in one band only
  is a finding, not a winner.
- **ABORT:** no variant reaching **1.02** in both bands. The axis is then a
  measured floor and the package lands nothing (rule 5: a measured structural
  floor is a finding, not a failure).
- **IDENTITY:** the three-seat, 128-search block P1 registered, run for the
  winner against base. **Any mismatch = the variant is rejected**, whatever it
  measured — a codegen flag that changes an answer is not a codegen flag this
  project can use.
- **REBUILD MEANS RE-RECORD (D-577's costed limb):** every receipt in this
  project names a binary digest, and landing a profile change moves all of them.
  The closure re-takes them rather than citing the old ones.

## Results

**All three variants, 5 reps, idle receipts, node identity holding per position
at both budgets in every rep, exit 0** (`artifacts/p3_bench_{cgu1,thin,fat}_v1.txt`):

| variant | nps early | nps late | time-to-depth early / late |
|---|---|---|---|
| `codegen-units = 1` alone | **0.992** | **0.979** | 0.973 / 1.010 |
| **`lto = "thin"` + 1 unit** | **1.068** | **1.064** | 1.091 / 1.061 |
| `lto = "fat"` + 1 unit | 1.030 | 1.022 | 1.044 / 0.981 |

**MORE OPTIMISATION IS NOT BETTER HERE, AND THAT IS THE PACKAGE'S FINDING.**
One codegen unit with no cross-crate work is a LOSS in both bands: the optimiser
gives up more from worse inlining decisions inside a single huge unit than it
gains from seeing the whole crate. Thin LTO on top turns it into a **6.6 %** win,
because this engine's hot path spans four crates — `pistol-core`'s window
enumeration, `pistol-eval`'s delta, `pistol-solver`'s store, `pistol-search`'s
loop — and cross-crate inlining is the thing it actually wants. Fat gives half of
that back. The registered expectation was 1.05–1.20 for the LTO variants and
1.00–1.08 for `cgu1`; thin lands inside its band and `cgu1` lands **below** its,
which is recorded rather than smoothed.

**IDENTITY, and it is what licenses adopting a codegen flag at all**
(`artifacts/p3_identity_thin_v1.txt`): 128 `bestmove` and 0 `error` per side over
the three registered seats, `RESULT: IDENTICAL`, exit 0.

**ACCEPTED: `lto = "thin"`, `codegen-units = 1`.** Highest in both bands, output
identical, and 10 s to build against the base's 6 s on a warm cache — a cost the
project pays on every landing without noticing.

**THE NEW PINNED DIGEST (D-577's costed limb).** `target/release/pistol` at the
landing is
`619b81c90a31279fd06074911d168ee628e6c5422c15746fc736631bfe5d44be`, replacing
`1413698a22ffbb95fd008b2f27c533c88e3399b8806b62f71b4f827e30271e6d`. It is bit
for bit the `thin` variant this document benched, which is how the profile
written into `Cargo.toml` is tied to the measurement rather than to this
session's word. **Every receipt in this repository that names a binary digest
now names a superseded one**; they are records of runs at their own revisions
and are not rewritten.
