#!/usr/bin/env bash
#
# The solver oracle gate (WP-1.8a, docs/experiments/wp18a_design.md §7):
# all four oracles over the sha-pinned fixture, in release (the deep cases
# need it), through the test target the gates live in.
#
# The four gates, each printed by its own test:
#   (a) differential   — solver value == R3' brute-force value, every case
#   (b) proof trees    — every Win's witness tree re-proved full-width
#   (c) RZ property    — the relevance-zone tolerance over the sigma class
#   (d) TT cross-check — full table vs a 32-entry table, same values
#
# A gate's failure is the finding (exit 1). A run where the question could
# not be asked at all is a VOID (exit 2, SHELL_CHECKLIST item 12) — never a
# pass and never a failure.
#
# Usage: tools/solver_oracle_check.sh
# Exit:  0 all four gates pass
#        1 at least one gate failed, named on stdout
#        2 the run is void — no gate was adjudicated

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail() { echo "solver_oracle_check: FAIL: $*" >&2; exit 1; }
void() { echo "solver_oracle_check: RUN VOID: $*" >&2; exit 2; }

command -v cargo >/dev/null || void "cargo is not on PATH"

OUT="$(mktemp)" || void "mktemp refused"
trap 'rm -f "$OUT"' EXIT

# Release: the fixture's deep cases and gate (c)'s sigma sweep need it; the
# debug cost is minutes per gate against seconds here (the tactical gate's
# split, D-54's precedent).
if ! cargo test --release -p pistol-solver --test solver_oracle_tests -- --nocapture >"$OUT" 2>&1; then
	# The test harness prints the failing gate's own FAIL block; surface it
	# verbatim rather than paraphrasing it (the receipts rule).
	grep -E "^gate \(|panicked|FAILED" "$OUT" || true
	fail "the oracle test target — the lines above are the gate's own"
fi

# Positive content: a gate that prints nothing proved nothing. Four PASS
# lines, one per gate, is what the target prints when it works.
PASSES="$(grep -c '^gate (.*) PASS' "$OUT" || true)"
case "$PASSES" in
4) ;;
*) fail "expected four gate PASS lines, found $PASSES — the target's output is above" ;;
esac

grep -E '^gate \(|^test result' "$OUT"
echo "solver_oracle_check: all four gates passed"
