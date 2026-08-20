#!/usr/bin/env bash
#
# The tactical gate: the sha-pinned fixture, every case, at the threshold the
# fixture pre-registered before the suite was first run (CLAUDE.md rule 7 and
# §Process).
#
# In release, and through the engine binary rather than through `cargo test`,
# because the deep cases need it — the DEBUG cost, not the release cost. Measured
# on the development machine: all twenty cases, each searched twice, cost 4.8 s
# here and 63 s as the ignored debug test. No case in the fixture runs at the
# shipping radius 3: the depth-3 cases run at configs/gate_v0.toml's radius 1 and
# the rest at the instrument radius 2, so the 84-100 s in that file's measurement
# table is why gate_v0 exists and is not what this script costs. `cargo test` runs
# the cheap half of the suite and the fixture's pin; this runs all of it. The same
# split `tools/perft_check.sh` uses for the movegen oracle (docs/decisions.md
# D-54).
#
# `selftest` also searches every case twice — once from a fresh engine and once
# from an engine that has played and been told `newgame` — and any disagreement
# fails the gate whatever the tactical threshold says: the determinism law is not a
# percentage (CLAUDE.md rule 4).
#
# Each case names the config it is a claim about, so this script names none: a
# threshold means nothing without the search that has to meet it, and putting the
# config on the command line would let the two drift apart.
#
# Usage: tools/tactical_check.sh
# Exit:  0 the suite meets its threshold, 1 it does not.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

FIXTURE="crates/pistol-cli/tests/fixtures/tactical_v0.txt"

fail() { printf 'tactical_check: FAIL: %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo is not on PATH"
[ -f "$FIXTURE" ] || fail "no fixture at $FIXTURE"

echo "tactical_check: building the engine (release, locked)"
cargo build --release --locked --quiet --bin pistol || fail "the engine does not build"

./target/release/pistol selftest --fixtures "$FIXTURE" ||
	fail "the tactical suite did not meet its pre-registered threshold"
