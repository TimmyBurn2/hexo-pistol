#!/usr/bin/env bash
#
# The solver's determinism seat (WP-1.8a, design §7): the selftest binary,
# built once, run twice in separate processes over the registered fixture,
# full transcript diffed. The solver consults no clock and no hasher
# iteration order, so two runs must agree on every printed field — value,
# node count, seesaw, digest, zone status (CLAUDE.md rule 4, D-7).
#
# What is NOT compared: nothing is exempt. The transcript has no wall-clock
# field, which is what makes byte equality the bar.
#
# Usage: tools/solver_determinism.sh
# Exit:  0 the two runs agree
#        1 they do not, or a run refused
#        2 the question could not be asked (VOID)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail() { echo "solver_determinism: FAIL: $*" >&2; exit 1; }
void() { echo "solver_determinism: RUN VOID: $*" >&2; exit 2; }

command -v cargo >/dev/null || void "cargo is not on PATH"

FIXTURE="crates/pistol-solver/tests/fixtures/solver_v0.txt"
CONFIG="configs/solver_v0.toml"
[ -f "$FIXTURE" ] || void "no fixture at $FIXTURE"
[ -f "$CONFIG" ] || void "no config at $CONFIG"

BIN="target/release/solver-selftest"
# Not silenced: a refused build is a void whose cause cargo itself names.
if ! cargo build --release -p pistol-solver --bin solver-selftest; then
	void "the build failed — cargo's own words are above"
fi
[ -x "$BIN" ] || void "no binary at $BIN after a green build"

OUT="$(mktemp -d)" || void "mktemp refused"
trap 'rm -rf "$OUT"' EXIT

# Two SEPARATE processes, the gate proper (D-7's own form).
"$BIN" "$FIXTURE" "$CONFIG" >"$OUT/run-a" 2>"$OUT/err-a" || fail "run A refused: $(cat "$OUT/err-a")"
"$BIN" "$FIXTURE" "$CONFIG" >"$OUT/run-b" 2>"$OUT/err-b" || fail "run B refused: $(cat "$OUT/err-b")"

# Positive content: a transcript of refusals would agree and prove nothing.
# One line per case plus the summary is what a working run prints.
CASES="$(grep -c '^case ' "$OUT/run-a" || true)"
SUMMARY="$(grep -c '^summary ' "$OUT/run-a" || true)"
case "$CASES:$SUMMARY" in
0:*|*:0) fail "run A printed $CASES case lines and $SUMMARY summaries — nothing was solved" ;;
esac
if [ "$(grep -c '^case ' "$OUT/run-b" || true)" -ne "$CASES" ]; then
	fail "run B printed a different number of cases than run A"
fi

if ! diff -u "$OUT/run-a" "$OUT/run-b" >"$OUT/diff"; then
	cat "$OUT/diff" >&2
	fail "the two runs disagree — the diff is above"
fi

echo "solver_determinism: PASS — $CASES cases, byte-identical transcripts"
