#!/usr/bin/env bash
#
# The judge itself, end to end, on every CI run. Hard Rule 6 makes SPRT over
# paired openings the judge of every search and eval change, so an arena that
# has quietly stopped working would not fail anything else in this suite — every
# other gate is about the engine.
#
# It is a SELF-MATCH, and that is what makes it assertable rather than merely
# plausible. Two identical deterministic engines play the same game whichever
# seat they sit in, so the expected answer is known in advance:
#
#   * every opening is played from both seats            -> n = 2 x openings
#   * both games of a pair are the same game move for move -> distinct_n = n/2
#   * every pair therefore scores 1-1                    -> pentanomial p2 = pairs
#   * no pair-to-pair variation exists                   -> no LLR is defined
#   * so the verdict is `inconclusive_degenerate`, which is the CORRECT answer
#     for two identical configurations and not a failure
#
# A gate whose expected output is "something plausible" is not a gate, which is
# why every one of those is asserted exactly.
#
# The second thing this gate does is the arena's OWN determinism, which nothing
# else covers: the run is repeated and the two verdict blocks are compared byte
# for byte. That is `tools/determinism.sh`'s shape applied to the instrument
# every strength claim comes from. The comparison is on the verdict block and
# not the whole file because the timing block is machine-dependent BY DESIGN and
# says so on its own marker line (docs/decisions.md D-161).
#
# Cost, pre-registered (CLAUDE.md rule 5): three arena runs at depth_turns 1 and
# candidate radius 1 over 4 openings, which is 24 games and about 480 engine
# searches. Expected well under a minute on the development machine; the cost
# grows with `turn_cap` because the candidate set grows with the stone count, so
# a change to that key in the config is a change to this gate's runtime.
#
# Usage: tools/arena_smoke.sh
# Exit:  0 the gate holds, 1 it does not.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CONFIG="configs/arena_smoke_v0.toml"

fail() { printf 'arena_smoke: FAIL: %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo is not on PATH"
[ -f "$CONFIG" ] || fail "no arena config at $CONFIG"

# Never under the repository: match logs are artifacts and artifacts are not
# committed (CLAUDE.md rule 8).
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "arena_smoke: building the engine and the arena (release, locked)"
# THE BINARY THIS GATE RUNS IS THE BINARY CARGO BUILT: the path comes from cargo's
# artifact stream, never from a literal here. `CARGO_TARGET_DIR`, `[build]
# target-dir` and `[build] target` each move the artifact, and a hardcoded
# `target/release/pistol` then runs whatever STALE binary sits at that path while
# the build goes elsewhere — a gate that passes for a binary nobody built
# (REPRODUCED on tools/tactical_check.sh; docs/decisions.md D-250).
BUILD_LOG="$(cargo build --release --locked --quiet --bin pistol --bin arena \
	--message-format=json-render-diagnostics)" || fail "the engine and the arena do not build"
# Two bins, so each executable is taken by the name cargo gave the FILE rather
# than by the order the artifact stream happens to emit them in.
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
ENGINE=""
ARENA=""
for path in ${BUILT[@]+"${BUILT[@]}"}; do
	case "${path##*/}" in
	pistol) ENGINE="$path" ;;
	arena) ARENA="$path" ;;
	esac
done
# ONE REFUSAL PER REASON (tools/SHELL_CHECKLIST.md item 8): a bin cargo named no
# executable for, and then a named path that is absent, is not a regular file, or
# carries no `+x` — the last being the case `command -v` admits and exec answers
# with 126. A helper that must refuse is called as a statement, never inside a
# command substitution, where `fail` would exit only the subshell (item 1).
usable() { # $1 = the bin name, $2 = the path cargo named for it
	[ -n "$2" ] || fail "cargo built no executable for --bin $1"
	[ -e "$2" ] || fail "cargo named \`$2\` for --bin $1 and nothing is there"
	[ -f "$2" ] || fail "cargo named \`$2\` for --bin $1 and it is not a regular file"
	[ -x "$2" ] || fail "cargo named \`$2\` for --bin $1 and it is not executable"
}
usable pistol "$ENGINE"
usable arena "$ARENA"

# What the config says, read from the config rather than restated here: a gate
# that hard-coded the numbers would pass after somebody changed the document.
TAKE="$(sed -n 's/^openings_take = \([0-9]*\).*/\1/p' "$CONFIG" | head -1)"
WORKERS="$(sed -n 's/^n_workers = \([0-9]*\).*/\1/p' "$CONFIG" | head -1)"
[ -n "$TAKE" ] || fail "$CONFIG states no openings_take"
GAMES=$((TAKE * 2))
echo "arena_smoke: $TAKE openings, $GAMES games, $WORKERS workers, config $CONFIG"

run_arena() {
	local out="$1" config="$2"
	"$ARENA" --config "$config" --out "$out" >"$out.stdout" 2>"$out.stderr" ||
		fail "the arena exited nonzero:
$(tail -20 "$out.stderr")"
	[ -s "$out" ] || fail "no report was written to $out"
}

field() { sed -n "s/^$2 //p" "$1" | head -1; }

echo "arena_smoke: run 1"
run_arena "$WORK/a.txt" "$CONFIG"

# --- the self-match's knowable answer -----------------------------------------

KIND="$(head -1 "$WORK/a.txt" | awk '{print $1}')"
[ "$KIND" = "arena_report" ] ||
	fail "the run was abandoned: the report is an \`$KIND\`, not a verdict-carrying one"

PLAYED="$(grep -c '^game ' "$WORK/a.txt" || true)"
[ "$PLAYED" -eq "$GAMES" ] ||
	fail "played $PLAYED games, expected $GAMES (every opening from both seats)"

COUNTS="$(field "$WORK/a.txt" counts)"
case "$COUNTS" in
*"n $GAMES distinct_n $TAKE"*) ;;
*) fail "expected n $GAMES and distinct_n $TAKE, got: $COUNTS
Two identical deterministic engines must produce identical games; anything else
means the dedupe, the seating or the determinism law has moved." ;;
esac
case "$COUNTS" in
*"forfeits 0"*) ;;
*) fail "a self-match forfeited a game, which means the engine broke its own protocol:
$COUNTS
$(grep '^refusal ' "$WORK/a.txt" | head -5)" ;;
esac

PENTANOMIAL="$(field "$WORK/a.txt" pentanomial)"
case "$PENTANOMIAL" in
*"p2 $TAKE"*) ;;
*) fail "expected every one of the $TAKE pairs to score 1-1, got: $PENTANOMIAL" ;;
esac

VERDICT="$(field "$WORK/a.txt" verdict)"
[ "$VERDICT" = "inconclusive_degenerate" ] ||
	fail "expected \`inconclusive_degenerate\` — two identical configurations give a
sample with no variance and therefore no likelihood ratio — got \`$VERDICT\`"
[ "$(field "$WORK/a.txt" verdict_unit)" = "pair" ] ||
	fail "the verdict is read off the PAIR unit (docs/decisions.md D-154)"

# Per-side compute was recorded, not merely printed. A driver that billed
# nothing would still emit the fields.
while read -r line; do
	for key in nodes_a nodes_b depth_a depth_b; do
		value="$(echo "$line" | sed -n "s/.* $key \([0-9]*\).*/\1/p")"
		[ -n "$value" ] && [ "$value" -gt 0 ] ||
			fail "$key is zero or missing, so per-side compute was not recorded: $line"
	done
done < <(grep '^game ' "$WORK/a.txt")

echo "arena_smoke: $GAMES games, distinct-n $TAKE, verdict $VERDICT, compute recorded"

# --- the arena's own determinism ----------------------------------------------

echo "arena_smoke: run 2 (same config, compared to run 1)"
run_arena "$WORK/b.txt" "$CONFIG"

# Everything before the timing marker is the verdict block, which is
# worker-invariant and machine-invariant by design.
verdict_block() { sed '/^# timing/,$d' "$1"; }
verdict_block "$WORK/a.txt" >"$WORK/a.block"
verdict_block "$WORK/b.txt" >"$WORK/b.block"
[ -s "$WORK/a.block" ] || fail "the verdict block is empty, so the diff below proves nothing"
diff -u "$WORK/a.block" "$WORK/b.block" >"$WORK/diff.repeat" ||
	fail "two runs of the same config disagreed:
$(head -40 "$WORK/diff.repeat")"

# --- and its independence from the worker count -------------------------------

echo "arena_smoke: run 3 (one worker, compared to $WORKERS)"
sed "s/^n_workers = .*/n_workers = 1/" "$CONFIG" >"$WORK/single.toml"
run_arena "$WORK/c.txt" "$WORK/single.toml"
verdict_block "$WORK/c.txt" >"$WORK/c.block"
diff -u "$WORK/a.block" "$WORK/c.block" >"$WORK/diff.workers" ||
	fail "one worker and $WORKERS workers disagreed on the verdict block, which is the one
thing scheduling may never touch (CLAUDE.md rule 4, docs/decisions.md D-161):
$(head -40 "$WORK/diff.workers")"

grep -q '^timing n_workers 1 ' "$WORK/c.txt" ||
	fail "the one-worker run does not record its worker count, so the comparison above
could have passed against a report that simply omits it"
grep -q "^timing n_workers $WORKERS " "$WORK/a.txt" ||
	fail "the $WORKERS-worker run does not record its worker count either"

printf 'arena_smoke: ok — %d games over %d openings, distinct-n %d, verdict %s,\n' \
	"$GAMES" "$TAKE" "$TAKE" "$VERDICT"
printf 'arena_smoke:      three runs agree on the verdict block at 1 and %d workers\n' "$WORKERS"
