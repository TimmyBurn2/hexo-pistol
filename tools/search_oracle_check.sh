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
# than one turn of each side, plus a fourth turn on the one position that can
# carry one. A third turn costs a quarter of a million reference nodes on the
# smallest position with a branching factor at all, and the mate distance of
# three costs millions. The same split `tools/perft_check.sh` uses for the
# movegen oracle (docs/decisions.md D-54).
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

# The universe and the refusals: what the agreement assertions stand on. And the
# reference's enumeration checking itself. Split from the file above for
# CLAUDE.md rule 9's soft cap, not because they are different gates, so they run
# here too.
cargo test --release --locked --package pistol-search --test search_oracle_universe_tests ||
	fail "the reference's universe or its refusals are not the search's"

cargo test --release --locked --package pistol-search --test search_oracle_dedupe_tests ||
	fail "the reference's deduped enumeration is not its both-orderings enumeration"

# A ride-along, and not an oracle assertion: `[profile.release] overflow-checks =
# true` is a flag that reverts silently, and its test has to run in the profile
# it is about — a debug build has overflow checks by default and would pass it
# without saying anything. This script owns the only release `cargo test` in the
# gate set, so the alternative is a gate of its own for one test
# (docs/decisions.md D-127).
cargo test --release --locked --package pistol-search --test build_profile_tests ||
	fail "the release profile no longer keeps the checks it is configured to keep"

echo "search_oracle_check: the depths a debug build cannot afford"
cargo test --release --locked --package pistol-search --test search_oracle_deep_tests -- \
	--include-ignored --nocapture ||
	fail "the search and the reference disagree at depth 3 or 4"
