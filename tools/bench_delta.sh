#!/usr/bin/env bash
#
# The Eval::delta bench (CLAUDE.md rule 5; docs/decisions.md D-192, D-215, D-226).
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
# NOTHING ABOVE MOVED when the revision mode landed. D-226 records that the
# measurement is untouched: same pinned config, same bands, same reps floor,
# same IQR gate, same per-position node identity assertion, same verdict rule.
# What changed is only how a side's BINARY is obtained.
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
# TWO WAYS TO NAME A SIDE, chosen PER SIDE and never guessed (CLAUDE.md rule 3):
#
#   <path>              an executable. The script does NOT build it. This is
#                       D-215's original contract and it is unchanged: the
#                       harness measures exactly the binary the caller pinned.
#   rev:<commit-ish>    a revision. The script builds it in a throwaway
#                       `git worktree`, copies the binary out, and removes the
#                       worktree before measuring.
#
# Per side rather than per run, because the A/B Stage 1 actually runs is an
# UNCOMMITTED CANDIDATE against a COMMITTED BASELINE, and a whole-run mode flag
# cannot say that. NAMED HAZARD, which is why the mixed form exists: `git stash
# create` silently omits UNTRACKED files, so a `rev:` side pointed at a stash
# revision measures a tree missing its newest source file. Measure uncommitted
# work by passing the BINARY you built, or `git add -A` first.
#
# EACH SIDE IS RESOLVED TO THE FILE THAT WILL ACTUALLY BE EXEC'D before it is
# digested (D-226). A bare name with no slash is PATH-resolved by the shell at
# exec time while `sha256sum` would read the cwd-relative file of the same name,
# so an unresolved digest can attest a file that never ran; `command -v` resolves
# it the way the shell will and `realpath` collapses the symlink and `..`
# spellings that let one engine appear as two. The resolved side must be a
# REGULAR file — a directory and a FIFO both pass `[ -x ]`, and the FIFO blocks
# the digest forever. NAMED RESIDUAL: a wrapper script that `exec`s another
# engine is a genuinely different file, so its digest is truthful and the
# same-digest refusal below still cannot see through it; that case is caught by
# reading the digests, not by the script.
#
# Two identical digests are refused in BOTH modes — a same-binary-twice run
# would report ratio 1.0 as a silent false ABORT. In revision mode the refusal
# also names both revisions, because "the same binary" is confusing when the
# caller passed two different SHAs (a docs-only diff does exactly this).
#
# THE HANDSHAKE GUARD, and precisely what it is worth (D-226): before measuring,
# each binary is launched once with the `pistol` verb and its own `id` lines are
# captured, then compared. This attests the bytes each ENGINE actually read
# (D-198) rather than digesting files on its behalf. What it CATCHES is two
# binaries PARSING THE SAME LIVE DOCUMENTS DIFFERENTLY — schema drift, a changed
# backend token, a changed default. What it CANNOT catch is the two revisions
# having shipped different instruments, because neither process ever reads its
# own revision's config: both read the live tree's. That is narrower than
# "instrument constancy" and is stated rather than implied. Cost: one extra
# process launch per side, about 25 ms each.
#
# WHAT THE GUARD COMPARES is the ENUMERATED INSTRUMENT SET and nothing else:
# `config`, `eval`, `tt_bytes`, `candidate_policy`, `weights_sha256`. The A/B
# this harness exists for is two REVISIONS, which differ in `name`/`version` as
# a matter of course, so comparing every `id` line turned a workspace version
# bump into a refusal of the bench (D-226). The unguarded lines are still
# printed when they differ, as a NOTE and never as a refusal.
#
# RULE9-JUSTIFICATION: one measurement over one pre-registration. The verdict
# rule, the instrument it is taken with and the two ways of naming a side are
# one responsibility and one argument — splitting the side resolution out would
# put the provenance of the measured bytes in a different file from the
# measurement whose citation depends on it, which is the seam this script exists
# to keep closed.
#
# Usage: tools/bench_delta.sh SIDE_A SIDE_B [REPS]
#        SIDE is a path to an executable, or rev:<commit-ish>.
# Exit:  0 measured and verdict printed, 1 a precondition or the run failed.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CONFIG="configs/instrument_v0.toml"
WEIGHTS="configs/eval_v0_weights.toml"
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
# A build that runs longer than this is a finding too, not a wait.
BUILD_TIMEOUT=900
# And so is a handshake that never answers: `[ -x ]` admits a FIFO, which would
# otherwise block this script forever with no output at all.
HANDSHAKE_TIMEOUT=60

fail() { printf 'bench_delta: FAIL: %s\n' "$*" >&2; exit 1; }

[ "$#" -ge 2 ] || fail "usage: tools/bench_delta.sh SIDE_A SIDE_B [REPS] (a SIDE is an executable path, or rev:<commit-ish>)"
SIDE_BASE="$1"; SIDE_CAND="$2"; REPS="${3:-5}"
[ "$REPS" -ge 5 ] 2>/dev/null || fail "REPS must be an integer >= 5 (pre-registered), got: $REPS"
[ -f "$CONFIG" ] || fail "no config at $CONFIG"
[ -f "$WEIGHTS" ] || fail "no weights at $WEIGHTS"
[ -f "$FIXTURE" ] || fail "no fixture at $FIXTURE"

WORK="$(mktemp -d)"
# An ARRAY and not a whitespace-joined string: `for wt in $WORKTREES` word-splits
# on the spaces a TMPDIR may contain, so the removal loop below would iterate over
# path fragments, remove nothing, and never reach its rule-3 WARNING either. No
# leak followed, because `rm -rf "$WORK"` precedes `git worktree prune` — but the
# safety came from the ORDERING and not from the loop that is supposed to provide
# it, which is not a property to leave resting on an accident.
WORKTREES=()
cleanup() {
	# The in-line removal in `resolve_side` is the mechanism; this is the
	# backstop for a build that died half way (docs/decisions.md D-217, D-219:
	# a round closes with `git worktree list` showing one checkout).
	#
	# ORDER MATTERS AND IS THE POINT. `git worktree prune` declines to prune an
	# entry whose directory still exists, so pruning BEFORE the removal and the
	# `rm -rf` left the admin entry behind every time. Remove the worktrees,
	# then the directory that held them, then prune what died half way.
	local wt
	for wt in ${WORKTREES[@]+"${WORKTREES[@]}"}; do
		[ -e "$wt" ] || continue
		git worktree remove --force "$wt" >/dev/null 2>&1 ||
			printf 'bench_delta: WARNING: could not remove the worktree at %s — it is in the listing below\n' "$wt" >&2
	done
	rm -rf "$WORK"
	git worktree prune >/dev/null 2>&1 || true
	# The invariant D-217 and D-219 close their rounds on, printed rather than
	# asserted, so a report can cite this script's own output for it.
	echo "bench_delta: worktrees at exit:"
	git worktree list | sed 's/^/bench_delta:   /'
}
trap cleanup EXIT

# Resolve one side to the executable that will ACTUALLY be exec'd, building it
# if it names a revision. Sets RESOLVED rather than printing the path: a command
# substitution is a subshell, so `WORKTREES` would never reach the parent and
# `fail`'s `exit 1` would leave only the subshell — which is how a failed build
# leaked a worktree into the live repository (D-226).
RESOLVED=""
resolve_side() { # label side -> sets RESOLVED
	local label="$1" side="$2" rev sha wt out real word
	word=baseline; [ "$label" = base ] || word=candidate
	RESOLVED=""
	case "$side" in
	rev:*)
		rev="${side#rev:}"
		[ -n "$rev" ] || fail "$word side \`$side\`: rev: needs a revision after the colon"
		sha="$(git rev-parse --verify --quiet "${rev}^{commit}" 2>/dev/null || true)"
		# Git's own refusal is "Needed a single revision", which does not name
		# what it refused. Rule 3 wants the input quoted back.
		[ -n "$sha" ] || fail "$word side \`$side\`: \`$rev\` is not a commit this repository can resolve"
		# The header's named hazard, DETECTED rather than only documented: a
		# `git stash create` commit has the index's parent beside HEAD's and a
		# `WIP on`/`index on` subject, and it carries nothing that was never
		# `git add`ed. A note and not a refusal — measuring a stash revision is
		# legitimate once `git add -A` has run.
		if [ "$(git rev-list --parents -n 1 "$sha" | wc -w)" -ge 3 ]; then
			case "$(git log -1 --format=%s "$sha")" in
			'WIP on '* | 'index on '*)
				echo "bench_delta: NOTE $word revision $sha is a \`git stash create\` commit — files never \`git add\`ed are NOT in it" ;;
			esac
		fi
		wt="$WORK/wt.$label"
		echo "bench_delta: $word revision $rev -> $sha"
		git worktree add --detach "$wt" "$sha" >/dev/null 2>&1 ||
			fail "$word side \`$side\`: cannot create a worktree at $sha"
		WORKTREES+=("$wt")
		echo "bench_delta: $word build: cargo build --release --locked -p pistol-cli --bin pistol (in $wt)"
		(cd "$wt" && timeout "$BUILD_TIMEOUT" cargo build --release --locked -p pistol-cli --bin pistol >/dev/null 2>&1) ||
			fail "$word side \`$side\`: the build at $sha failed or exceeded ${BUILD_TIMEOUT}s"
		[ -x "$wt/target/release/pistol" ] ||
			fail "$word side \`$side\`: the build at $sha produced no target/release/pistol"
		out="$WORK/$label.pistol"
		cp "$wt/target/release/pistol" "$out"
		# Removed here rather than at exit: the worktree's target directory is
		# hundreds of megabytes and /tmp is RAM on the machines this runs on.
		# A removal that FAILS is reported and the entry is KEPT, so `cleanup`
		# retries it and the listing it prints is the truth (CLAUDE.md rule 3).
		if git worktree remove --force "$wt" >/dev/null 2>&1; then
			local keep=() other
			for other in ${WORKTREES[@]+"${WORKTREES[@]}"}; do
				[ "$other" = "$wt" ] || keep+=("$other")
			done
			WORKTREES=(${keep[@]+"${keep[@]}"})
		else
			printf 'bench_delta: WARNING: could not remove the build worktree at %s; it is retried at exit\n' "$wt" >&2
		fi
		RESOLVED="$out"
		;;
	*)
		# `command -v` resolves the name the way the shell will at exec time;
		# `-f` refuses the directory and the FIFO that `[ -x ]` admits.
		real="$(command -v -- "$side" 2>/dev/null || true)"
		[ -n "$real" ] && [ -f "$real" ] && [ -x "$real" ] ||
			fail "$word binary not executable: $side (a revision is named \`rev:$side\`)"
		RESOLVED="$(realpath -- "$real")"
		;;
	esac
}

resolve_side base "$SIDE_BASE"; BASE="$RESOLVED"
resolve_side cand "$SIDE_CAND"; CAND="$RESOLVED"

BASE_SHA="$(sha256sum "$BASE" | cut -d' ' -f1)"
CAND_SHA="$(sha256sum "$CAND" | cut -d' ' -f1)"
echo "bench_delta: baseline  $SIDE_BASE -> $BASE ($BASE_SHA)"
echo "bench_delta: candidate $SIDE_CAND -> $CAND ($CAND_SHA)"
[ "$BASE_SHA" != "$CAND_SHA" ] ||
	fail "the two sides resolve to the same binary ($SIDE_BASE and $SIDE_CAND, digest $BASE_SHA) — a same-binary run reports ratio 1.0 as a silent false ABORT"

# The instrument, by the bytes ACTUALLY ON DISK — never `git show`, which would
# attest the committed bytes while the run reads the working tree's.
echo "bench_delta: instrument $CONFIG $(sha256sum "$CONFIG" | cut -d' ' -f1)"
echo "bench_delta: instrument $WEIGHTS $(sha256sum "$WEIGHTS" | cut -d' ' -f1)"
echo "bench_delta: instrument $FIXTURE $(sha256sum "$FIXTURE" | cut -d' ' -f1)"

# THE HANDSHAKE GUARD. The `id` lines are emitted only in reply to the `pistol`
# verb, so this is one extra launch per side.
#
# The ENUMERATED INSTRUMENT SET, and nothing beyond it: these are the fields the
# design named, they are the ones the two processes derive from the live
# documents, and `name`/`version` are deliberately NOT among them because two
# revisions differ in them by construction (D-226).
GUARDED_ID_FIELDS="config eval tt_bytes candidate_policy weights_sha256"
handshake() { # binary -> its id lines
	printf 'pistol\nquit\n' | timeout "$HANDSHAKE_TIMEOUT" "$1" --config "$CONFIG" 2>/dev/null | grep '^id ' || true
}
guarded() { # id-file -> the guarded subset, in the engine's own order
	awk -v want=" $GUARDED_ID_FIELDS " 'index(want, " " $2 " ")' "$1"
}
handshake "$BASE" >"$WORK/id.base"
handshake "$CAND" >"$WORK/id.cand"
[ -s "$WORK/id.base" ] || fail "the baseline binary answered the \`pistol\` handshake with no id lines"
[ -s "$WORK/id.cand" ] || fail "the candidate binary answered the \`pistol\` handshake with no id lines"
GUARDED_COUNT="$(printf '%s\n' $GUARDED_ID_FIELDS | wc -l)"
for side in base cand; do
	guarded "$WORK/id.$side" >"$WORK/idg.$side"
	[ "$(wc -l <"$WORK/idg.$side")" -eq "$GUARDED_COUNT" ] ||
		fail "the $side binary's handshake is missing one of the guarded instrument fields ($GUARDED_ID_FIELDS)"
done
if ! diff -u "$WORK/idg.base" "$WORK/idg.cand" >"$WORK/id.diff"; then
	printf 'bench_delta: FAIL: the two binaries disagree on the instrument they read: %s\n' \
		"$(grep '^[-+]id ' "$WORK/id.diff" | awk '{ print $2 }' | sort -u | tr '\n' ' ')" >&2
	sed 's/^/  /' "$WORK/id.diff" >&2
	exit 1
fi
# The NOTE, written with the SAME IDIOM as the refusal above — the diff redirected
# into $WORK inside the `if !` CONDITION and the file read afterwards. The obvious
# spelling, `diff -u … | grep … | sed …` in the body, is a pipeline whose `diff`
# exits 1 whenever it has anything to say: `pipefail` makes the pipeline exit 1,
# it is the last command in the `then` body rather than a condition, and `set -e`
# then KILLS THE SCRIPT — silently, with no FAIL line, no identity block and no
# verdict, on exactly the difference this branch exists to tolerate. That is
# CLAUDE.md rule 3 broken by the code that was meant to stop breaking it, and
# `bench_delta_drives_to_a_verdict_when_the_handshakes_differ_outside_the_guarded_set`
# is the test that keeps it fixed.
if ! diff -u "$WORK/id.base" "$WORK/id.cand" >"$WORK/id.note"; then
	echo "bench_delta: NOTE the handshakes differ outside the guarded set, which two revisions do by construction:"
	sed -n 's/^\([-+]id .*\)$/bench_delta:   \1/p' "$WORK/id.note"
fi
sed 's/^/bench_delta: identity /' "$WORK/id.base"

echo "bench_delta: config $CONFIG, nodes $NODES, depth_turns $DEPTH, reps $REPS"

# Positions and their bands, in fixture order.
#
# NAMED EXCEPTION to the byte-identical measurement half (docs/decisions.md
# D-231): `|| true`. A fixture with no entries makes both greps exit 1, which
# under `pipefail` aborted this script with NO diagnostic at all, one line before
# the named refusal below — the same defect class as the NOTE branch above, in
# the half D-220 and D-226 certify unchanged. Emptiness is a verdict this script
# states, not a status it dies of, and on any fixture with an entry the two forms
# write the same bytes.
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
