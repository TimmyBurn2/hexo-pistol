#!/usr/bin/env bash
#
# The determinism gate, in the form CLAUDE.md rule 4 asks for: the same positions
# and the same budgets, in TWO SEPARATE PROCESSES, compared line for line. Any
# difference is a failure (docs/decisions.md D-7).
#
# Four engine runs making two comparisons, because they catch different things:
#
#   A vs B   the same input to two processes. This is the gate proper: two runs of
#            the same positions under the same reproducible budget must agree on
#            the move, the node count, the score, the depth and the whole
#            principal variation.
#   C vs D   the same positions under one budget, C with one process per position
#            and D with all of them in one session. A and B share a layout, so
#            neither can see whether `newgame` really clears everything the
#            previous position left in the table; this pair is that question.
#
# What is NOT compared: `nps` and `time`. They measure the machine, not the
# search. Every other field is reproducible and is compared.
#
# A diff is not the only way this can fail. Two processes that both refused every
# line would produce identical transcripts and prove nothing, so the transcripts
# are also checked for positive content: one `bestmove` per `go`, no `error` line
# anywhere, and a completed depth on every answer.
#
# Usage: tools/determinism.sh
# Exit:  0 the runs agree and did real work, 1 they do not.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# The config the gate runs at, and why it is not the deployment config: at
# candidate radius 3 a completed depth of 4 turns costs hours per position, which
# is the measured Stage-0 floor recorded in that file's own comment table
# (CLAUDE.md rule 5). The determinism law is about whether two runs agree, not
# about how strong they are.
CONFIG="configs/gate_v0.toml"

# The fixture the positions come from. Its sha pin is enforced by the test that
# reads it (docs/decisions.md D-37), so an edited fixture is already a red
# `cargo test`; this gate compares two runs against each other and cannot be
# wrong about a position it was given.
FIXTURE="crates/pistol-cli/tests/fixtures/tactical_v0.txt"

# The budgets, both reproducible. A wall-clock budget could not be compared at
# all, and instrument mode refuses to be given one (docs/decisions.md D-22).
BUDGETS=("depth_turns 4" "nodes 200000")

# The budget run one-process-per-position as well. The node budget, because it is
# the cheaper of the two and it exercises the interrupted-iteration path.
LAYOUT_BUDGET="nodes 200000"

fail() { printf 'determinism: FAIL: %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo is not on PATH"
[ -f "$CONFIG" ] || fail "no config at $CONFIG"
[ -f "$FIXTURE" ] || fail "no fixture at $FIXTURE"

# Never under the repository: transcripts are artifacts and artifacts are not
# committed (CLAUDE.md rule 8).
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "determinism: building the engine (release, locked)"
cargo build --release --locked --quiet --bin pistol || fail "the engine does not build"
ENGINE="$ROOT/target/release/pistol"
[ -x "$ENGINE" ] || fail "no engine binary at $ENGINE"

# The positions, exactly as the fixture spells them: a fixture `position` line IS
# a protocol line, so nothing here has to know what a position looks like.
mapfile -t POSITIONS < <(sed -n 's/^position //p' "$FIXTURE")
[ "${#POSITIONS[@]}" -gt 0 ] || fail "the fixture states no positions"
# One position per case, cross-checked: an extraction that quietly matched fewer
# lines than the fixture has cases would shrink the gate while CI stayed green,
# which is the same failure as a gate that passes on refusals (docs/decisions.md
# D-90).
CASES="$(grep -c '^case ' "$FIXTURE" || true)"
[ "${#POSITIONS[@]}" -eq "$CASES" ] ||
	fail "extracted ${#POSITIONS[@]} positions from $FIXTURE but it states $CASES cases"
echo "determinism: ${#POSITIONS[@]} positions, ${#BUDGETS[@]} budgets, config $CONFIG"

# One session over every position and every budget. `newgame` before each so that
# a position's answer does not depend on the ones before it.
SCRIPT="$WORK/session.txt"
: >"$SCRIPT"
GOES=0
for budget in "${BUDGETS[@]}"; do
	for position in "${POSITIONS[@]}"; do
		printf 'newgame\nposition %s\ngo %s\n' "$position" "$budget" >>"$SCRIPT"
		GOES=$((GOES + 1))
	done
done
echo "quit" >>"$SCRIPT"

# `nps` and `time` are the only fields two runs may disagree about.
normalize() { sed -E 's/ nps [0-9]+ time [0-9]+//'; }

check_content() {
	local transcript="$1" expected_goes="$2" what="$3"
	local moves errors depths
	errors="$(grep -c '^error ' "$transcript" || true)"
	[ "$errors" -eq 0 ] || fail "$what: $errors error line(s); a refused run proves nothing
$(grep -m 5 '^error ' "$transcript")"
	# Counted by SHAPE, not just by prefix: this pins that every answer is a turn
	# token — one cell, or two in canonical order — and not an empty or malformed
	# line that a prefix count would accept (docs/decisions.md D-5).
	local turn='-\?[0-9]\+,-\?[0-9]\+'
	moves="$(grep -c "^bestmove $turn\(/$turn\)\?\$" "$transcript" || true)"
	[ "$moves" -eq "$expected_goes" ] ||
		fail "$what: $moves bestmove lines carrying a turn token, for $expected_goes searches"
	# A completed depth is at least 1 and has no upper bound worth assuming.
	depths="$(grep -c '^info totals depth_turns [1-9][0-9]* ' "$transcript" || true)"
	[ "$depths" -eq "$expected_goes" ] ||
		fail "$what: $depths completed-depth totals lines for $expected_goes searches"
}

for run in A B; do
	echo "determinism: run $run (one process, every position)"
	"$ENGINE" --config "$CONFIG" <"$SCRIPT" >"$WORK/raw.$run" ||
		fail "run $run exited nonzero"
	check_content "$WORK/raw.$run" "$GOES" "run $run"
	normalize <"$WORK/raw.$run" >"$WORK/run.$run"
done

diff -u "$WORK/run.A" "$WORK/run.B" >"$WORK/diff.ab" ||
	fail "two processes disagreed on the same input:
$(head -40 "$WORK/diff.ab")"
echo "determinism: runs A and B agree ($(wc -l <"$WORK/run.A") lines)"

# One process per position, under one budget, and the same answers expected.
echo "determinism: run C (one process per position, go $LAYOUT_BUDGET)"
: >"$WORK/raw.C"
for position in "${POSITIONS[@]}"; do
	printf 'position %s\ngo %s\nquit\n' "$position" "$LAYOUT_BUDGET" |
		"$ENGINE" --config "$CONFIG" >>"$WORK/raw.C" || fail "run C exited nonzero"
done
check_content "$WORK/raw.C" "${#POSITIONS[@]}" "run C"
normalize <"$WORK/raw.C" >"$WORK/run.C"

# The same positions and budget in ONE process, to compare C against. Replaying
# them is cheaper and clearer than parsing that budget's slice back out of run A,
# and it is the same comparison.
: >"$WORK/session.layout"
for position in "${POSITIONS[@]}"; do
	printf 'newgame\nposition %s\ngo %s\n' "$position" "$LAYOUT_BUDGET" >>"$WORK/session.layout"
done
echo "quit" >>"$WORK/session.layout"
"$ENGINE" --config "$CONFIG" <"$WORK/session.layout" >"$WORK/raw.D" ||
	fail "the one-process layout run exited nonzero"
check_content "$WORK/raw.D" "${#POSITIONS[@]}" "run D"
normalize <"$WORK/raw.D" >"$WORK/run.D"

diff -u "$WORK/run.D" "$WORK/run.C" >"$WORK/diff.cd" ||
	fail "a position answered differently on its own than in a session with the others,
so newgame does not clear everything it must:
$(head -40 "$WORK/diff.cd")"
echo "determinism: per-position processes agree with the session"

printf 'determinism: ok — %d searches, %d positions, no difference outside nps/time\n' \
	"$GOES" "${#POSITIONS[@]}"
