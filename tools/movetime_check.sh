#!/usr/bin/env bash
#
# The movetime-ceiling gate (WP-1.4, superseding docs/decisions.md D-95): in
# play mode, `go movetime N` answers within N + play.movetime_epsilon_ms, in
# RELEASE, on the sha-pinned D-95 reproducer class — spread-stone positions
# whose candidate count grows as fast as the rules allow per stone.
#
# Two layers, because they can fail differently:
#
#   1. `cargo test --release` over the movetime and fallback test binaries: the
#      in-process asserts (legality by pistol-core replay, N + epsilon on every
#      position x budget, honest depth reporting, fallback determinism) in the
#      profile the promise is about.
#   2. The real binary over the fixture at movetime 500 / 50 / 1: the measured
#      overshoot table, printed as this gate's own log output, each entry
#      checked against N + epsilon. The elapsed figure is the engine's own
#      `time` field from the totals line — the search's deadline-to-return
#      span, which is exactly what epsilon bounds.
#
# Raising play.movetime_epsilon_ms to green this gate is a post-hoc threshold
# move CLAUDE.md forbids; the value's measured domain is recorded beside the
# WP-1.4 decision lines in docs/decisions.md.
#
# Usage: tools/movetime_check.sh
# Exit:  0 the ceiling holds, 1 it does not.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CONFIG="configs/play_v0.toml"
FIXTURE="crates/pistol-cli/tests/fixtures/spread_v1.txt"
BUDGETS_MS=(500 50 1)

fail() { printf 'movetime: FAIL: %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo is not on PATH"
[ -f "$CONFIG" ] || fail "no config at $CONFIG"
[ -f "$FIXTURE" ] || fail "no fixture at $FIXTURE"

# The promise under test comes from the config, never from this script.
EPSILON="$(sed -n 's/^movetime_epsilon_ms = \([0-9]\+\)$/\1/p' "$CONFIG")"
[ -n "$EPSILON" ] || fail "$CONFIG states no movetime_epsilon_ms"
echo "movetime: epsilon ${EPSILON} ms from $CONFIG"

echo "movetime: release test layer (movetime_tests, fallback_tests)"
cargo test --release --locked -p pistol-cli --test movetime_tests ||
	fail "the release movetime tests do not hold"
cargo test --release --locked -p pistol-search --test fallback_tests ||
	fail "the release fallback tests do not hold"

echo "movetime: building the engine (release, locked)"
cargo build --release --locked --quiet --bin pistol || fail "the engine does not build"
ENGINE="$ROOT/target/release/pistol"

# The positions, exactly as the fixture spells them (a fixture `position` line
# IS a protocol line), with the stone counts they advertise.
mapfile -t POSITIONS < <(sed -n 's/^position //p' "$FIXTURE")
mapfile -t STONES < <(sed -n 's/^stones //p' "$FIXTURE")
[ "${#POSITIONS[@]}" -gt 0 ] || fail "the fixture states no positions"
[ "${#POSITIONS[@]}" -eq "${#STONES[@]}" ] ||
	fail "${#POSITIONS[@]} positions beside ${#STONES[@]} stone counts"

echo "movetime: overshoot table (engine-reported time, bound = N + ${EPSILON} ms)"
printf '%8s %12s %12s %10s\n' "stones" "movetime_ms" "elapsed_ms" "verdict"
WORST=0
for i in "${!POSITIONS[@]}"; do
	for budget in "${BUDGETS_MS[@]}"; do
		out="$(printf 'position %s\ngo movetime %s\nquit\n' "${POSITIONS[$i]}" "$budget" |
			"$ENGINE" --config "$CONFIG")" || fail "the engine exited nonzero"
		grep -q '^error ' <<<"$out" && fail "the engine refused a fixture search:
$(grep -m 3 '^error ' <<<"$out")"
		elapsed="$(sed -n 's/^info totals .* time \([0-9]\+\) .*/\1/p' <<<"$out")"
		[ -n "$elapsed" ] || fail "no totals time in the answer for ${STONES[$i]} stones"
		bound=$((budget + EPSILON))
		verdict="ok"
		if [ "$elapsed" -gt "$bound" ]; then verdict="OVER"; fi
		printf '%8s %12s %12s %10s\n' "${STONES[$i]}" "$budget" "$elapsed" "$verdict"
		over=$((elapsed - budget))
		[ "$over" -gt "$WORST" ] && WORST=$over
		[ "$verdict" = "ok" ] ||
			fail "movetime $budget on ${STONES[$i]} stones took $elapsed ms (bound $bound ms)"
	done
done

printf 'movetime: ok — %d searches, worst overshoot %d ms against epsilon %d ms\n' \
	$((${#POSITIONS[@]} * ${#BUDGETS_MS[@]})) "$WORST" "$EPSILON"
