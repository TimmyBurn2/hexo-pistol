#!/usr/bin/env bash
#
# The D-263 BASELINE bench: the cover arithmetic (`blocking_covers`,
# `min_hitting_set_exceeds`) on the per-node path, measured as it lands
# (docs/decisions.md D-263; `docs/wp15b_impl_prompt.md` §4).
#
# D-263 pre-registers the HOTSPOT and states plainly that it carries no
# bracket, no abort threshold, and no bench of its own — those are IMPL's to
# set, from THIS commit's own measurement, before the first remedy is written
# (CLAUDE.md rule 5). This script is that measurement: ONE side, ONE tree
# state, no ratio — the number a later remedy's bracket and abort threshold
# are derived from, not a verdict on anything.
#
# THE INSTRUMENT, chosen to match tools/bench_delta.sh's already-vetted shape
# rather than invent a second one: nps AND time-to-depth, IQR-gated, over
# crates/pistol-cli/tests/fixtures/bench_positions_v1.txt's two bands ("both
# stone counts the arena plays from", §4) — the SAME sha-pinned corpus D-192's
# bench already uses, because the positions are policy-agnostic (a `position`
# line names cells, not a candidate policy) and reusing a corpus already
# measured at this scale is cheaper than re-deriving a second one nobody has
# looked at. What is NEW here is the CONFIG: configs/instrument_staged_v0.toml,
# so every node this bench counts is one that actually reaches
# `crate::staged::filtered` and so actually pays the cover arithmetic when the
# node's row is FILTERED — "at the candidate counts its own generator
# produces, not at counts chosen to make a curve" (§4).
#
# NOT AN A/B COMPARISON, so none of tools/bench_delta.sh's two-sided machinery
# applies here: no worktree resolution, no digest-equality refusal, no
# handshake guard between two binaries (there is only one). The engine is
# built live from the tree this script runs in, following
# tools/tactical_check.sh's and tools/staged_soundness_check.sh's own
# artifact-path pattern (D-250): the binary CARGO built, from cargo's own
# artifact stream, never a literal `target/release/pistol` path.
#
# A REMEDY COMMIT'S RATIO IS NOT COMPUTED BY THIS SCRIPT. This script always
# measures ONE tree state, absolutely — the BASELINE commit had nothing to
# compare against, so a live two-binary comparator (tools/bench_delta.sh's
# `rev:` vs `rev:` shape) would have been dead code until a remedy existed
# (CLAUDE.md: no design for a hypothetical requirement). A remedy commit runs
# this SAME script again, on its own build, and the caller derives the ratio
# by hand from the two runs' own printed numbers — both taken with the
# identical instrument (config, fixture, REPS), each independently IQR-gated
# clean, which is what makes the two numbers comparable at all.
#
# Usage: tools/staged_cover_bench.sh [REPS]   (REPS defaults to 5, >= 5)
# Exit:  0 a clean measurement was taken (bands may still be individually
#          NOISY — see the per-band line), 1 a precondition or the run failed,
#          2 THE RUN IS VOID — no measurement was taken
#          (tools/SHELL_CHECKLIST.md item 12).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CONFIG="configs/instrument_staged_v0.toml"
FIXTURE="crates/pistol-cli/tests/fixtures/bench_positions_v1.txt"
NODES=50000
DEPTH=2
EARLY_MAX=17
SESSION_TIMEOUT=900
BUILD_TIMEOUT=900

fail() { printf 'staged_cover_bench: FAIL: %s\n' "$*" >&2; exit 1; }
void() { printf 'staged_cover_bench: RUN VOID: %s\n' "$*" >&2; exit 2; }

REPS="${1:-5}"
case "$REPS" in
*[!0-9]* | "") fail "REPS must be an integer >= 5 (pre-registered), got: \`$REPS\`" ;;
0* | +*) fail "REPS must be spelled as a plain decimal integer with no leading zero or sign, got: \`$REPS\`" ;;
esac
[ "$REPS" -ge 5 ] || fail "REPS must be an integer >= 5 (pre-registered), got: $REPS"
[ -f "$CONFIG" ] || fail "no config at $CONFIG"
[ -f "$FIXTURE" ] || fail "no fixture at $FIXTURE"

for SCRATCH in "${TMPDIR:-/tmp}" "$ROOT"; do
	tools/scratch_preflight.sh "$SCRATCH" ||
		void "no measurement was taken; the lines above name the filesystem"
done

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

digest() { # path what -> prints "path sha"
	local line
	line="$(sha256sum -- "$1" 2>/dev/null)" ||
		fail "cannot read $2 at $1 to digest it"
	printf 'staged_cover_bench: instrument %s %s\n' "$1" "${line%% *}"
}
digest "$CONFIG" "the instrument config"
digest "$FIXTURE" "the bench fixture"

echo "staged_cover_bench: build: cargo build --release --locked -p pistol-cli --bin pistol"
BUILD_LOG="$(timeout "$BUILD_TIMEOUT" cargo build --release --locked --quiet --bin pistol \
	--message-format=json-render-diagnostics)" || fail "the engine does not build"
mapfile -t BUILT < <(sed -n 's/.*"executable":"\([^"\\]*\)".*/\1/p' <<<"$BUILD_LOG")
NAMED="$(grep -c '"executable":"' <<<"$BUILD_LOG" || true)"
case "$NAMED" in
*[!0-9]* | "") fail "the artifact-record count is not a number: \`$NAMED\`" ;;
esac
[ "$NAMED" -eq "${#BUILT[@]}" ] ||
	fail "cargo named $NAMED executables and this script could read ${#BUILT[@]} of them"
[ "${#BUILT[@]}" -eq 1 ] ||
	fail "cargo named ${#BUILT[@]} executables for --bin pistol: ${BUILT[*]}"
ENGINE="${BUILT[0]}"
[ -x "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and it is not executable"

IDENTITY="$(printf 'pistol\nquit\n' | timeout 60 "$ENGINE" --config "$CONFIG" 2>/dev/null |
	grep '^id ' || true)"
[ -n "$IDENTITY" ] || fail "the engine answered no \`id \` lines to the \`pistol\` handshake"
printf '%s\n' "$IDENTITY" | sed 's/^/staged_cover_bench: identity /'

echo "staged_cover_bench: config $CONFIG, nodes $NODES, depth_turns $DEPTH, reps $REPS"

grep -v '^#' "$FIXTURE" | grep . >"$WORK/entries" || true
COUNT="$(wc -l <"$WORK/entries")"
[ "$COUNT" -gt 0 ] || fail "the fixture states no positions"
: >"$WORK/bands"
while IFS= read -r entry; do
	stones="${entry##* stones }"
	case "$stones" in
	'' | *[!0-9]*) fail "entry without a stones count: $entry" ;;
	esac
	if [ "$stones" -le "$EARLY_MAX" ]; then echo early; else echo late; fi >>"$WORK/bands"
done <"$WORK/entries"
EARLY_COUNT="$(grep -c early "$WORK/bands" || true)"
LATE_COUNT="$(grep -c late "$WORK/bands" || true)"
echo "staged_cover_bench: $COUNT positions ($EARLY_COUNT early, $LATE_COUNT late)"

for budget in "nodes $NODES" "depth_turns $DEPTH"; do
	name="${budget%% *}"
	: >"$WORK/session.$name"
	while IFS= read -r entry; do
		position="${entry%% #*}"
		printf 'newgame\nposition %s\ngo %s\n' "$position" "$budget" >>"$WORK/session.$name"
	done <"$WORK/entries"
	echo quit >>"$WORK/session.$name"
done

run_session() { # budget_name out
	local budget_name="$1" out="$2"
	timeout "$SESSION_TIMEOUT" "$ENGINE" --config "$CONFIG" \
		<"$WORK/session.$budget_name" >"$WORK/raw" ||
		fail "engine on $budget_name: exited nonzero or exceeded ${SESSION_TIMEOUT}s"
	[ "$(grep -c '^error ' "$WORK/raw" || true)" -eq 0 ] ||
		fail "engine on $budget_name: error lines in the transcript"
	awk '/ totals /{
		nodes=""; time="";
		for (i=1; i<NF; i++) {
			if ($i=="nodes") nodes=$(i+1);
			if ($i=="time") time=$(i+1);
		}
		if (nodes=="" || time=="") { print "PARSE" > "/dev/stderr"; exit 1 }
		print nodes, time;
	}' "$WORK/raw" >"$WORK/totals" || fail "engine on $budget_name: unparseable totals line"
	[ "$(wc -l <"$WORK/totals")" -eq "$COUNT" ] ||
		fail "engine on $budget_name: $(wc -l <"$WORK/totals") totals lines for $COUNT positions"
	paste -d' ' "$WORK/bands" "$WORK/totals" >"$out"
}

for rep in $(seq 1 "$REPS"); do
	echo "staged_cover_bench: rep $rep/$REPS"
	run_session nodes "$WORK/nodes.$rep"
	run_session depth_turns "$WORK/depth.$rep"
done

band_metric() { # budget band metric-per-rep...
	local budget="$1" band="$2" rep
	for rep in $(seq 1 "$REPS"); do
		awk -v band="$band" -v budget="$budget" '$1==band { nodes+=$2; time+=$3 }
			END {
				if (time==0) { print "ZEROTIME" > "/dev/stderr"; exit 1 }
				if (budget=="nodes") printf "%.1f\n", 1000*nodes/time; else print time;
			}' "$WORK/$budget.$rep" || fail "zero band time ($budget $band)"
	done
}

stats() { # values on stdin -> "median iqr"
	sort -n | awk '{ v[NR]=$1 }
		END {
			q1=v[int((NR+3)/4)]; q3=v[NR+1-int((NR+3)/4)];
			m=(NR%2) ? v[(NR+1)/2] : (v[NR/2]+v[NR/2+1])/2;
			printf "%.1f %.1f\n", m, (q3>q1)?q3-q1:q1-q3;
		}'
}

echo "staged_cover_bench: ---- results (BASELINE — absolute numbers, no ratio, no verdict) ----"
NOISY_ANY=0
for band in early late; do
	nn=$(band_metric nodes "$band" | stats)
	nt=$(band_metric depth "$band" | stats)
	read -r nn_m nn_i <<<"$nn"
	read -r nt_m nt_i <<<"$nt"
	echo "band $band: nps median $nn_m (IQR $nn_i)"
	echo "band $band: time-to-depth-$DEPTH median ${nt_m} ms (IQR $nt_i)"
	noisy=$(awk -v a="$nn_i" -v am="$nn_m" -v b="$nt_i" -v bm="$nt_m" \
		'BEGIN { print (a>0.10*am || b>0.10*bm) ? 1 : 0 }')
	if [ "$noisy" = 1 ]; then
		echo "band $band: NOISY — an IQR exceeds 10% of its median; rerun before citing this band"
		NOISY_ANY=1
	fi
done
[ "$NOISY_ANY" = 0 ] ||
	fail "at least one band is NOISY; the numbers above are printed but not a clean BASELINE"
echo "staged_cover_bench: done — this is the D-263 BASELINE (rule 6: instrument $CONFIG, fixed nodes $NODES / depth_turns $DEPTH, $COUNT positions, $REPS reps); a remedy commit's bracket and abort threshold are derived from these numbers"
