#!/usr/bin/env bash
#
# The differential search oracle gate: `Searcher` against an independently
# written full-width negamax over the same move universe, in release, including
# the depths a debug build cannot afford (CLAUDE.md rule 7, docs/decisions.md
# D-106, D-120).
#
# `cargo test` already runs the always-on half in debug: every fixture at one
# turn, the cheap ones at two, the mate distances of one and two turns, the
# table-size independence check and the movegen cross-check. What this script
# adds is DEPTH — the third turn, where an exact mate distance greater than two
# first exists and where D-72's root-anchored re-basing is exercised across more
# than one turn of each side. A third turn costs at least a million reference
# nodes on the smallest position this game has, which is seconds in release and
# half a minute in a debug build. The same split `tools/perft_check.sh` uses for
# the movegen oracle (docs/decisions.md D-54).
#
# `--nocapture` is deliberate: each run prints the reference node count it cost,
# so the measured runtime this suite is budgeted against regenerates from a run
# rather than being remembered from the one that set it.
#
# Usage: tools/search_oracle_check.sh
# Exit:  0 the search and the reference agree everywhere, 1 they do not.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail() { printf 'search_oracle_check: FAIL: %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo is not on PATH"

echo "search_oracle_check: the always-on tier, in release"
cargo test --release --locked --package pistol-search --test search_oracle_tests -- --nocapture ||
	fail "the search and the reference disagree at depths 1 and 2"

# The universe and the refusals: what the agreement assertions stand on. Split
# from the file above for CLAUDE.md rule 9's soft cap, not because it is a
# different gate, so it runs here too.
cargo test --release --locked --package pistol-search --test search_oracle_universe_tests ||
	fail "the reference's universe or its refusals are not the search's"

echo "search_oracle_check: the depths a debug build cannot afford"
cargo test --release --locked --package pistol-search --test search_oracle_deep_tests -- \
	--include-ignored --nocapture ||
	fail "the search and the reference disagree at depth 3"
