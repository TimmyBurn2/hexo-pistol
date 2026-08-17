#!/usr/bin/env bash
#
# The perft oracle gate: pair-move generation against the independently written
# brute-force reference, in release, including the wider second-level sample
# that a debug build cannot afford (CLAUDE.md rule 7, docs/decisions.md D-12).
#
# `cargo test` already runs the same comparison in debug, minus that sample:
# every fixture position at every depth the fixture states, and the turn SETS
# (not just the counts) at the first level. What this script adds is depth —
# more first turns divided on — bought with a release build.
#
# Usage: tools/perft_check.sh
# Exit:  0 the two generators agree everywhere, 1 they do not.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "perft_check: pistol-core perft tests, release, including the ignored wide sample"
cargo test --release --locked --package pistol-core --test perft_tests -- --include-ignored
