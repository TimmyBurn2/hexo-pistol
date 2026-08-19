#!/usr/bin/env bash
#
# The Eval::delta bench (CLAUDE.md rule 5; docs/decisions.md D-192, D-215).
#
# Pre-registered BEFORE implementation, in the WP dispatch and restated here so
# the script and its verdict rule travel together:
#
#   hotspot          the eval apply/undo roundtrip — 85.6% of stacks, 76.3%
#                    under move ordering (WP-1.3 Run 2, D-192)
#   gain bracket     1.4x to 2.5x nps AND time-to-depth ratio >= 1.4, both
#                    bands of bench_positions_v1, fixed-node instrument runs,
#                    IQR-gated, >= 5 repetitions
#   abort threshold  < 1.15x nps — the change is reverted and the numbers
#                    recorded as a finding
#
# The config is PINNED: configs/instrument_v0.toml (radius 2, D-194). Named
# caveat, recorded rather than hidden: the bracket descends from a profile
# taken at radius 3 (D-192), so the ordering-eval share at radius 2 is
# unmeasured — a miss whose mechanism is the radius confound is a finding
# naming it, never a threshold move.
#
# Time-to-depth here is NOT independent evidence: the change is
# search-identical, so nodes-to-depth are identical and the ttd ratio is the
# nps ratio over the depth-2 node mix — a cross-check, printed with its
# deviation from the fixed-node nps ratio.
#
# The script NEVER builds: it measures exactly the two binaries the caller
# pinned, and prints both their digests. Two identical digests are refused —
# a same-binary-twice run would report ratio 1.0 as a silent false ABORT.
#
# Usage: tools/bench_delta.sh BASELINE_BINARY CANDIDATE_BINARY [REPS]
# Exit:  0 measured and verdict printed, 1 a precondition or the run failed.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CONFIG="configs/instrument_v0.toml"
FIXTURE="crates/pistol-cli/tests/fixtures/bench_positions_v1.txt"
NODES=50000
DEPTH=2
# The early band is centre 15 width 2; everything above is the late band
# (centre 35 width 5) — the fixture states stones per entry.
EARLY_MAX=17
# A rep that runs longer than this is a finding, not a wait: 7 of 24 fixture
# positions do not complete depth 2 within 50k nodes, so their depth-2 cost
# is unmeasured and this is the loud bound on it.
SESSION_TIMEOUT=900

fail() { printf 'bench_delta: FAIL: %s\n' "$*" >&2; exit 1; }

[ "$#" -ge 2 ] || fail "usage: tools/bench_delta.sh BASELINE_BINARY CANDIDATE_BINARY [REPS]"
BASE="$1"; CAND="$2"; REPS="${3:-5}"
[ -x "$BASE" ] || fail "baseline binary not executable: $BASE"
[ -x "$CAND" ] || fail "candidate binary not executable: $CAND"
[ "$REPS" -ge 5 ] 2>/dev/null || fail "REPS must be an integer >= 5 (pre-registered), got: $REPS"
[ -f "$CONFIG" ] || fail "no config at $CONFIG"
[ -f "$FIXTURE" ] || fail "no fixture at $FIXTURE"

BASE_SHA="$(sha256sum "$BASE" | cut -d' ' -f1)"
CAND_SHA="$(sha256sum "$CAND" | cut -d' ' -f1)"
echo "bench_delta: baseline  $BASE ($BASE_SHA)"
echo "bench_delta: candidate $CAND ($CAND_SHA)"
[ "$BASE_SHA" != "$CAND_SHA" ] || fail "baseline and candidate are the same binary"
echo "bench_delta: config $CONFIG, nodes $NODES, depth_turns $DEPTH, reps $REPS"

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# Positions and their bands, in fixture order.
grep -v '^#' "$FIXTURE" | grep . >"$WORK/entries"
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
echo "bench_delta: $COUNT positions ($(grep -c early "$WORK/bands") early, $(grep -c late "$WORK/bands") late)"

# One session script per budget: newgame/position/go for every position.
for budget in "nodes $NODES" "depth_turns $DEPTH"; do
	name="${budget%% *}"
	: >"$WORK/session.$name"
	while IFS= read -r entry; do
		position="${entry%% #*}"
		printf 'newgame\nposition %s\ngo %s\n' "$position" "$budget" >>"$WORK/session.$name"
	done <"$WORK/entries"
	echo quit >>"$WORK/session.$name"
done

# Run one session; emit one "band nodes time" line per position, in order.
run_session() {
	local binary="$1" budget_name="$2" out="$3"
	timeout "$SESSION_TIMEOUT" "$binary" --config "$CONFIG" \
		<"$WORK/session.$budget_name" >"$WORK/raw" ||
		fail "$binary on $budget_name: exited nonzero or exceeded ${SESSION_TIMEOUT}s"
	# `grep -c` exits 1 on a zero count, which is the GOOD case here.
	[ "$(grep -c '^error ' "$WORK/raw" || true)" -eq 0 ] ||
		fail "$binary on $budget_name: error lines in the transcript"
	awk '/ totals /{
		nodes=""; time="";
		for (i=1; i<NF; i++) {
			if ($i=="nodes") nodes=$(i+1);
			if ($i=="time") time=$(i+1);
		}
		if (nodes=="" || time=="") { print "PARSE" > "/dev/stderr"; exit 1 }
		print nodes, time;
	}' "$WORK/raw" >"$WORK/totals" || fail "$binary on $budget_name: unparseable totals line"
	[ "$(wc -l <"$WORK/totals")" -eq "$COUNT" ] ||
		fail "$binary on $budget_name: $(wc -l <"$WORK/totals") totals lines for $COUNT positions"
	paste -d' ' "$WORK/bands" "$WORK/totals" >"$out"
}

# Interleaved reps: baseline then candidate inside each rep, to spread drift.
for rep in $(seq 1 "$REPS"); do
	for side in base cand; do
		binary="$BASE"; [ "$side" = cand ] && binary="$CAND"
		echo "bench_delta: rep $rep/$REPS $side"
		run_session "$binary" nodes "$WORK/$side.nodes.$rep"
		run_session "$binary" depth_turns "$WORK/$side.depth.$rep"
	done
done

# NODE IDENTITY: the change is search-identical, so per-position node counts
# must agree between the binaries, under both budgets, in every rep.
for rep in $(seq 1 "$REPS"); do
	for budget in nodes depth; do
		cut -d' ' -f1,2 "$WORK/base.$budget.$rep" >"$WORK/id.a"
		cut -d' ' -f1,2 "$WORK/cand.$budget.$rep" >"$WORK/id.b"
		diff -q "$WORK/id.a" "$WORK/id.b" >/dev/null ||
			fail "node counts differ between binaries (budget $budget, rep $rep) — the candidate is not the search-identical change this bench is registered for"
	done
done
echo "bench_delta: node identity holds per position, both budgets, all reps"

# Per rep and band: total nodes / total time -> nps (fixed-node runs), and
# total time (depth runs). Bands are summed BEFORE any ratio, so ms
# quantization on fast positions cannot inflate the spread.
band_metric() { # side budget band metric-per-rep...
	local side="$1" budget="$2" band="$3" rep
	for rep in $(seq 1 "$REPS"); do
		awk -v band="$band" -v budget="$budget" '$1==band { nodes+=$2; time+=$3 }
			END {
				if (time==0) { print "ZEROTIME" > "/dev/stderr"; exit 1 }
				if (budget=="nodes") printf "%.1f\n", 1000*nodes/time; else print time;
			}' "$WORK/$side.$budget.$rep" || fail "zero band time ($side $budget $band)"
	done
}

# median and IQR of a small sorted list, in awk.
stats() { # values on stdin -> "median iqr"
	sort -n | awk '{ v[NR]=$1 }
		END {
			q1=v[int((NR+3)/4)]; q3=v[NR+1-int((NR+3)/4)];
			m=(NR%2) ? v[(NR+1)/2] : (v[NR/2]+v[NR/2+1])/2;
			printf "%.1f %.1f\n", m, (q3>q1)?q3-q1:q1-q3;
		}'
}

VERDICT_OK=1
report_band() { # band
	local band="$1"
	local bn cn bt ct
	bn=$(band_metric base nodes "$band" | stats)
	cn=$(band_metric cand nodes "$band" | stats)
	bt=$(band_metric base depth "$band" | stats)
	ct=$(band_metric cand depth "$band" | stats)
	read -r bn_m bn_i <<<"$bn"; read -r cn_m cn_i <<<"$cn"
	read -r bt_m bt_i <<<"$bt"; read -r ct_m ct_i <<<"$ct"
	echo "band $band: nps baseline median $bn_m (IQR $bn_i), candidate median $cn_m (IQR $cn_i)"
	echo "band $band: time-to-depth-$DEPTH baseline median ${bt_m} ms (IQR $bt_i), candidate median ${ct_m} ms (IQR $ct_i)"
	# The IQR gate: a spread above 10% of the median is NOISY and the band's
	# verdict is withheld — rerun, never reinterpret.
	local noisy
	noisy=$(awk -v a="$bn_i" -v am="$bn_m" -v b="$cn_i" -v bm="$cn_m" \
		-v c="$bt_i" -v cm="$bt_m" -v d="$ct_i" -v dm="$ct_m" \
		'BEGIN { print (a>0.10*am || b>0.10*bm || c>0.10*cm || d>0.10*dm) ? 1 : 0 }')
	if [ "$noisy" = 1 ]; then
		echo "band $band: NOISY — an IQR exceeds 10% of its median; verdict withheld, rerun"
		VERDICT_OK=0
		return
	fi
	awk -v bn="$bn_m" -v cn="$cn_m" -v bt="$bt_m" -v ct="$ct_m" -v band="$band" '
		BEGIN {
			nps = cn/bn; ttd = bt/ct; dev = ttd-nps; if (dev<0) dev=-dev;
			printf "band %s: nps ratio %.3f, time-to-depth ratio %.3f (|deviation| %.3f)\n", band, nps, ttd, dev;
			if (nps < 1.15)      printf "band %s: VERDICT ABORT — nps ratio %.3f is below the pre-registered 1.15 abort threshold; the change is reverted and this number is the finding\n", band, nps;
			else if (nps < 1.4)  printf "band %s: VERDICT BELOW-BRACKET — nps ratio %.3f clears the abort line but misses [1.4, 2.5]; a finding, not an acceptance\n", band, nps;
			else if (nps > 2.5)  printf "band %s: VERDICT ABOVE-BRACKET — nps ratio %.3f exceeds [1.4, 2.5]; a finding to explain, not a bigger win to bank\n", band, nps;
			else if (ttd < 1.4)  printf "band %s: VERDICT TTD-MISS — nps in bracket but time-to-depth ratio %.3f < 1.4\n", band, ttd;
			else                 printf "band %s: VERDICT PASS — inside the pre-registered bracket\n", band;
		}'
}

echo "bench_delta: ---- results ----"
report_band early
report_band late
[ "$VERDICT_OK" = 1 ] || exit 1
echo "bench_delta: done — the verdict lines above are the citable output (rule 6: instrument $CONFIG, fixed nodes $NODES / depth_turns $DEPTH, $COUNT positions, $REPS reps)"
