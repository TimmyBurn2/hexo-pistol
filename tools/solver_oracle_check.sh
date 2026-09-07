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

# Programs are resolved through the resolver, never `command -v` — item 8's
# table and docs/decisions.md D-683 say why. This script keeps its own class.
REQUIRE_TOOL="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/require_tool.sh"
[ -x "$REQUIRE_TOOL" ] || void "the tool resolver is missing beside this script: $REQUIRE_TOOL"

"$REQUIRE_TOOL" cargo >/dev/null || void "cargo is not usable"

# SCRATCH, BEFORE THE WORK (tools/SHELL_CHECKLIST.md item 12 obligation 2). A
# scratch FILE is scratch: the `mktemp -d` sweep that preflighted this gate's
# siblings was keyed on `-d` and did not reach here.
PREFLIGHT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/scratch_preflight.sh"
[ -x "$PREFLIGHT" ] || void "the scratch preflight is missing beside this script: $PREFLIGHT"
for SCRATCH_FS in "${TMPDIR:-/tmp}" "$ROOT"; do
	"$PREFLIGHT" "$SCRATCH_FS" ||
		void "no scratch room; the lines above name the filesystem"
done

OUT="$(mktemp)" || void "mktemp refused"
trap 'rc=$?; rm -f "$OUT" || echo "warning: scratch not removed: $OUT" >&2; exit "$rc"' EXIT

# Release: the fixture's deep cases and gate (c)'s sigma sweep need it; the
# debug cost is minutes per gate against seconds here (the tactical gate's
# split, D-54's precedent).
# --ignored: the four gates are release-only (the debug workspace run in
# CI gate 3 would take hours on gate (c); the tactical gate's split).
if ! cargo test --release -p pistol-solver --test solver_oracle_tests -- --ignored --nocapture >"$OUT" 2>&1; then
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
