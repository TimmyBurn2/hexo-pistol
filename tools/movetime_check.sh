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

echo "movetime: release test layer (movetime_tests, fallback_tests, instrument golden)"
cargo test --release --locked -p pistol-cli --test movetime_tests ||
	fail "the release movetime tests do not hold"
cargo test --release --locked -p pistol-search --test fallback_tests ||
	fail "the release fallback tests do not hold"
# The FULL golden-transcript set (all 40 pinned instrument cases; the debug
# profile's cargo test runs a stride subset). It rides this gate because this
# is the release cargo test the WP that pinned the transcripts added — a
# REVIEW-impl round caught the full set running in release on no gate at all
# (docs/decisions.md D-213).
cargo test --release --locked -p pistol-cli --test instrument_golden_tests ||
	fail "instrument behavior diverged from the pinned golden transcripts"

echo "movetime: building the engine (release, locked)"
# THE BINARY THIS GATE RUNS IS THE BINARY CARGO BUILT: the path comes from cargo's
# artifact stream, never from a literal here. `CARGO_TARGET_DIR`, `[build]
# target-dir` and `[build] target` each move the artifact, and a hardcoded
# `target/release/pistol` then runs whatever STALE binary sits at that path while
# the build goes elsewhere — a gate that passes for a binary nobody built
# (REPRODUCED on tools/tactical_check.sh; docs/decisions.md D-250).
BUILD_LOG="$(cargo build --release --locked --quiet --bin pistol \
	--message-format=json-render-diagnostics)" || fail "the engine does not build"
# The artifact records that name an executable; a library artifact carries
# `"executable":null` and cannot match. `sed` answers 0 on no match, so an empty
# result is a VALUE to refuse below and never a status to test.
mapfile -t BUILT < <(sed -n 's/.*"executable":"\([^"\\]*\)".*/\1/p' <<<"$BUILD_LOG")
# What the stream NAMED, against what this gate could READ: a path carrying a
# quote or a backslash matches neither class above and must not be mistaken for a
# bin cargo built nothing for. `grep -c` prints 0 and STILL exits 1 on no match
# (tools/SHELL_CHECKLIST.md item 3), so the empty count is a legitimate answer and
# gets `|| true` rather than a death; its SPELLING is then checked, not just its
# value (item 8).
NAMED="$(grep -c '"executable":"' <<<"$BUILD_LOG" || true)"
case "$NAMED" in
*[!0-9]* | "") fail "the artifact-record count is not a number: \`$NAMED\`" ;;
esac
[ "$NAMED" -eq "${#BUILT[@]}" ] ||
	fail "cargo named $NAMED executables and this gate could read ${#BUILT[@]} of them: a quote or a backslash in a path"
# ONE REFUSAL PER REASON (tools/SHELL_CHECKLIST.md item 8): no executable at all,
# several executables, and then a named path that is absent, is not a regular
# file, or carries no `+x` — the last being the case `command -v` admits and exec
# answers with 126.
[ "${#BUILT[@]}" -ne 0 ] || fail "cargo built no executable for --bin pistol"
[ "${#BUILT[@]}" -eq 1 ] ||
	fail "cargo named ${#BUILT[@]} executables for --bin pistol: ${BUILT[*]}"
ENGINE="${BUILT[0]}"
[ -e "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and nothing is there"
[ -f "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and it is not a regular file"
[ -x "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and it is not executable"

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
