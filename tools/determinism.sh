#!/usr/bin/env bash
#
# The determinism gate, in the form CLAUDE.md rule 4 asks for: the same positions
# and the same budgets, in TWO SEPARATE PROCESSES, compared line for line. Any
# difference is a failure (docs/decisions.md D-7).
#
# Four engine runs making two comparisons, PER SEAT, because they catch
# different things:
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
# FOUR SEATS, RADIUS, STAGED, STAGED-WITH-HEURISTICS AND
# STAGED-WITH-SOLVER (docs/decisions.md
# D-370 for the second; docs/experiments/wp17_design.md §5 for the third): the
# same one binary, built once and reused, run against each configuration in
# turn — each is a choice path this gate did not touch before it existed.
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

# The seats this gate runs at, and why each is not the deployment config: at
# candidate radius 3 a completed depth of 4 turns costs hours per position, which
# is the measured Stage-0 floor recorded in that file's own comment table
# (CLAUDE.md rule 5). The determinism law is about whether two runs agree, not
# about how strong they are. THREE SEATS, RADIUS, STAGED AND
# STAGED-WITH-HEURISTICS (docs/decisions.md D-370; WP-1.5b Phase 4 MINOR 5;
# WP-1.7): the staged generator is an entirely new choice path — a
# `ThreatState` carried and unwound in `Position`,
# tier extraction, `three_pairwise_disjoint_families` — that no run of this gate
# touched before, and CLAUDE.md rule 4 does not admit "verified by hand once" as
# a substitute for a gate every `tools/ci.sh` run repeats.
#
# Each seat is "name config fixture", space-separated: the fixture's sha pin is
# enforced by the test that reads it (docs/decisions.md D-37), so an edited
# fixture is already a red `cargo test`; this gate compares two runs against
# each other and cannot be wrong about a position it was given.
#
# A THIRD SEAT, STAGED WITH WP-1.7'S ORDERING HEURISTICS ON
# (docs/experiments/wp17_design.md §5): the heuristics add cross-search state
# (history, countermove) that persists within a game like the transposition
# table does, so the C-vs-D layout comparison below — one process per position
# against a session with `newgame` before each — is exactly the question that
# state makes live: a table `newgame` fails to clear shows up there as a
# position answered differently on its own than in the session.
SEATS=(
	"radius configs/gate_v0.toml crates/pistol-cli/tests/fixtures/tactical_v0.txt"
	"staged configs/gate_staged_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt"
	"staged-heuristics configs/gate_staged_heuristics_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt"
	# The solver seat carries its OWN budgets (WP-1.8b): its searches spend
	# real wall inside solver calls at every budget, and the standing
	# budgets would make this seat an hours-long run where its job is the
	# ON path's byte-identity under the D-7 law — reproducibility, not the
	# registered strength budgets (which the SPRT seat owns alone).
	"staged-solver configs/gate_staged_solver_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt depth_turns-2 nodes-10000"
	# The safety-net cap ARMED (docs/decisions.md D-478, D-482). Every other
	# seat runs it at its committed 0, so without this one the truncation and
	# its transposition store rule would be reproducible only by argument.
	"staged-safety-net-cap configs/gate_staged_snk_v0.toml crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt"
)

# The budgets, both reproducible. A wall-clock budget could not be compared at
# all, and instrument mode refuses to be given one (docs/decisions.md D-22).
BUDGETS=("depth_turns 4" "nodes 200000")

# The budget run one-process-per-position as well. The node budget, because it is
# the cheaper of the two and it exercises the interrupted-iteration path.
# Per-seat layout budgets where they differ (WP-1.8b): the solver seat's
# C-vs-D leg at the standing 200k budget costs minutes per position (the
# bench abort's own finding) and buys nothing over a smaller reproducible
# budget for the byte-identity law.
# Per-seat layout budgets where they differ (WP-1.8b): the solver seat's
# C-vs-D leg at the standing 200k budget costs minutes per position (the
# bench abort's own finding) and buys nothing over a smaller reproducible
# budget for the byte-identity law.
LAYOUT_BUDGET="nodes 200000"
SOLVER_LAYOUT_BUDGET="nodes 10000"
SOLVER_LAYOUT_BUDGET="nodes 10000"

fail() { printf 'determinism: FAIL: %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo is not on PATH"
for seat in "${SEATS[@]}"; do
	# The trailing `_` absorbs a seat's budget-override words.
	read -r _ seat_config seat_fixture _ <<<"$seat"
	[ -f "$seat_config" ] || fail "no config at $seat_config"
	[ -f "$seat_fixture" ] || fail "no fixture at $seat_fixture"
done

# Never under the repository: transcripts are artifacts and artifacts are not
# committed (CLAUDE.md rule 8).
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

echo "determinism: building the engine (release, locked)"
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

# One seat's whole determinism check, over the SAME `$ENGINE` binary — only the
# config and the fixture vary. `$WORK` files are prefixed by `$name` so two
# seats running in the same scratch directory never overwrite each other's
# transcripts.
run_seat() {
	local name="$1" config="$2" fixture="$3"
	shift 3
	# Per-seat budget overrides (WP-1.8b's solver seat): trailing words
	# spell budgets with `-` for the space (NOT `_`, which the budget
	# keywords themselves contain — `depth_turns-2`, not `depth_turns_2`,
	# which substituted into `depth turns 2` and refused twenty times).
	# Empty means the standing BUDGETS.
	# NOT "${@:-}": that default expands to ONE EMPTY STRING when no
	# override words were passed — an empty `go ` budget, twenty protocol
	# refusals, and a gate that fails on its own plumbing (caught on the
	# radius seat, which the override feature was never meant to touch).
	local -a budgets=()
	if (( $# )); then
		budgets=("$@")
		budgets=("${budgets[@]//-/ }")
	else
		budgets=("${BUDGETS[@]}")
	fi
	echo "determinism: seat $name: config $config"

	# The positions, exactly as the fixture spells them: a fixture `position`
	# line IS a protocol line, so nothing here has to know what a position
	# looks like.
	local -a positions
	mapfile -t positions < <(sed -n 's/^position //p' "$fixture")
	[ "${#positions[@]}" -gt 0 ] || fail "$name: the fixture states no positions"
	# One position per case, cross-checked: an extraction that quietly matched
	# fewer lines than the fixture has cases would shrink the gate while CI
	# stayed green, which is the same failure as a gate that passes on
	# refusals (docs/decisions.md D-90).
	local cases
	cases="$(grep -c '^case ' "$fixture" || true)"
	[ "${#positions[@]}" -eq "$cases" ] ||
		fail "$name: extracted ${#positions[@]} positions from $fixture but it states $cases cases"
	echo "determinism: seat $name: ${#positions[@]} positions, ${#BUDGETS[@]} budgets"

	# One session over every position and every budget. `newgame` before each
	# so that a position's answer does not depend on the ones before it.
	local script="$WORK/$name.session.txt"
	: >"$script"
	local goes=0
	local budget position
	for budget in "${budgets[@]}"; do
		for position in "${positions[@]}"; do
			printf 'newgame\nposition %s\ngo %s\n' "$position" "$budget" >>"$script"
			goes=$((goes + 1))
		done
	done
	echo "quit" >>"$script"

	local run
	for run in A B; do
		echo "determinism: seat $name: run $run (one process, every position)"
		"$ENGINE" --config "$config" <"$script" >"$WORK/$name.raw.$run" ||
			fail "$name: run $run exited nonzero"
		check_content "$WORK/$name.raw.$run" "$goes" "$name: run $run"
		normalize <"$WORK/$name.raw.$run" >"$WORK/$name.run.$run"
	done

	diff -u "$WORK/$name.run.A" "$WORK/$name.run.B" >"$WORK/$name.diff.ab" ||
		fail "$name: two processes disagreed on the same input:
$(head -40 "$WORK/$name.diff.ab")"
	echo "determinism: seat $name: runs A and B agree ($(wc -l <"$WORK/$name.run.A") lines)"

	# One process per position, under one budget, and the same answers
	# expected. The solver seat takes its own (cheaper, equally
	# reproducible) layout budget.
	local layout_budget="$LAYOUT_BUDGET"
	if [ "$name" = staged-solver ]; then
		layout_budget="$SOLVER_LAYOUT_BUDGET"
	fi
	echo "determinism: seat $name: run C (one process per position, go $layout_budget)"
	: >"$WORK/$name.raw.C"
	for position in "${positions[@]}"; do
		printf 'position %s\ngo %s\nquit\n' "$position" "$layout_budget" |
			"$ENGINE" --config "$config" >>"$WORK/$name.raw.C" || fail "$name: run C exited nonzero"
	done
	check_content "$WORK/$name.raw.C" "${#positions[@]}" "$name: run C"
	normalize <"$WORK/$name.raw.C" >"$WORK/$name.run.C"

	# The same positions and budget in ONE process, to compare C against.
	# Replaying them is cheaper and clearer than parsing that budget's slice
	# back out of run A, and it is the same comparison.
	: >"$WORK/$name.session.layout"
	for position in "${positions[@]}"; do
		printf 'newgame\nposition %s\ngo %s\n' "$position" "$layout_budget" >>"$WORK/$name.session.layout"
	done
	echo "quit" >>"$WORK/$name.session.layout"
	"$ENGINE" --config "$config" <"$WORK/$name.session.layout" >"$WORK/$name.raw.D" ||
		fail "$name: the one-process layout run exited nonzero"
	check_content "$WORK/$name.raw.D" "${#positions[@]}" "$name: run D"
	normalize <"$WORK/$name.raw.D" >"$WORK/$name.run.D"

	diff -u "$WORK/$name.run.D" "$WORK/$name.run.C" >"$WORK/$name.diff.cd" ||
		fail "$name: a position answered differently on its own than in a session with the others,
so newgame does not clear everything it must:
$(head -40 "$WORK/$name.diff.cd")"
	echo "determinism: seat $name: per-position processes agree with the session"

	printf 'determinism: seat %s: ok — %d searches, %d positions, no difference outside nps/time\n' \
		"$name" "$goes" "${#positions[@]}"
}

for seat in "${SEATS[@]}"; do
	read -r seat_name seat_config seat_fixture extra_budgets <<<"$seat"
	# shellcheck disable=SC2086 # the override words are the seat's own list
	run_seat "$seat_name" "$seat_config" "$seat_fixture" $extra_budgets
done

printf 'determinism: ok — %d seat(s), no difference outside nps/time in any of them\n' \
	"${#SEATS[@]}"
