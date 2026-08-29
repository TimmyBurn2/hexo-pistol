#!/usr/bin/env bash
#
# The rule-5 bench sweep every pre-registration since WP-1.7 §7 has copied as a
# shell block into its own document, made into one instrument with one governing
# revision (docs/process.md, "Instrument governing revision").
#
# WHY THIS IS A SCRIPT AND NOT A BLOCK IN A DOCUMENT. The copied block
# substitutes the EMPTY BOARD for any entry the engine refuses, and exits 0
# doing it: `error <kind>: <why>` goes to STDOUT (docs/decisions.md D-88), the
# engine exits 0, `sed -n 's/^info totals //p'` drops the error line, and the
# `go` that follows measures whatever position was last set — the empty board
# after `newgame`. MEASURED at 4be496a: a malformed entry returns a well-formed
# totals line with a pv beginning at the origin. That is
# `tools/SHELL_CHECKLIST.md` item 3's EXIT-0-WRONG-ANSWER class arriving in the
# machinery a pre-registration copies verbatim, so the fix belongs where the
# copying stops.
#
# The block was also INOPERABLE on the second committed bench fixture:
# `crates/pistol-cli/tests/fixtures/spread_v1.txt` states whole `position …`
# protocol lines interleaved with bare `stones N` lines, where
# `bench_positions_v1.txt` states position-verb TAILS with ` # …` commentary.
# The copied block prefixes `position ` unconditionally, so on `spread_v1.txt`
# it emitted `position position start moves …` and took `error Protocol:` on
# every entry — again at exit 0, again with no data. `--grammar` names which
# shape the fixture has; there is no sniffing, because a fixture whose shape is
# guessed is a fixture whose refusals are guessed too.
#
# ONE INVOCATION PER (ENTRY, REP), never one session over the whole fixture:
# the defect above is a refused entry LEAKING into the next entry's number, and
# per-invocation isolation plus a per-invocation refusal check is what removes
# the class rather than narrowing it.
#
# This script takes NO measurement decisions. It states no budget, no rep count,
# no config and no fixture of its own — every one of them is the caller's,
# because every one of them is a number a pre-registration registers, and a
# default here would be a tunable living outside the one schema place
# (CLAUDE.md rule 1). It prints one record line per (entry, rep) and a summary;
# banding, medians, IQR gating and ratios are the caller's, from these lines.
#
# Usage:
#   tools/bench_block.sh --engine PATH --config PATH --fixture PATH \
#                        --grammar tail|line --budget 'KIND N' --reps N [--label TEXT]
#
# Exit:  0 every entry loaded and every rep produced exactly one totals line
#        1 a refusal: an entry the engine would not load, a malformed fixture
#          line, a bad argument, or an invocation that produced no totals line
#        2 THE RUN IS VOID — no measurement was taken (the engine is missing or
#          unrunnable, a path is unreadable, or there is no scratch room)
#          (tools/SHELL_CHECKLIST.md item 12)

set -euo pipefail

# Pinned for the sort and for `$EPOCHREALTIME`'s decimal separator; the guards
# below are allow-lists, so the pin makes them refuse MORE, never less
# (tools/SHELL_CHECKLIST.md item 4).
export LC_ALL=C

say() { printf 'bench_block: %s\n' "$*"; }

fail() {
	printf 'bench_block: FAIL: %s\n' "$*" >&2
	exit 1
}

void() {
	printf 'bench_block: RUN VOID: %s\n' "$*" >&2
	exit 2
}

usage() {
	printf 'bench_block: usage: %s\n' "$*" >&2
	printf 'bench_block: usage: tools/bench_block.sh --engine PATH --config PATH --fixture PATH --grammar tail|line --budget %s --reps N [--label TEXT]\n' "'KIND N'" >&2
	exit 1
}

ENGINE=""
CONFIG=""
FIXTURE=""
GRAMMAR=""
BUDGET=""
REPS=""
LABEL=""

while [ "$#" -gt 0 ]; do
	case "$1" in
	--engine | --config | --fixture | --grammar | --budget | --reps | --label)
		[ "$#" -ge 2 ] || usage "$1 wants a value"
		case "$1" in
		--engine) ENGINE="$2" ;;
		--config) CONFIG="$2" ;;
		--fixture) FIXTURE="$2" ;;
		--grammar) GRAMMAR="$2" ;;
		--budget) BUDGET="$2" ;;
		--reps) REPS="$2" ;;
		--label) LABEL="$2" ;;
		esac
		shift 2
		;;
	*) usage "unknown argument \`$1\`" ;;
	esac
done

for pair in "engine:$ENGINE" "config:$CONFIG" "fixture:$FIXTURE" \
	"grammar:$GRAMMAR" "budget:$BUDGET" "reps:$REPS"; do
	[ -n "${pair#*:}" ] || usage "--${pair%%:*} is required and has no default"
done

# Item 9: every one of these reaches a printed record a reader and a test parse.
# A newline in one INJECTS RECORD LINES, and a control character makes the
# record unreadable in a way nobody notices. The guard is an allow-list so the
# `LC_ALL=C` pin above widens it.
for pair in "engine:$ENGINE" "config:$CONFIG" "fixture:$FIXTURE" \
	"grammar:$GRAMMAR" "budget:$BUDGET" "reps:$REPS" "label:$LABEL"; do
	case "${pair#*:}" in
	*[![:print:]]*) fail "--${pair%%:*} holds a character that is not printable text" ;;
	esac
done

case "$GRAMMAR" in
tail | line) ;;
*) usage "--grammar reads \`tail\` or \`line\`, got \`$GRAMMAR\`" ;;
esac

# Item 8: validate the SPELLING, not just the value — bash reads `010` as octal
# and the engine would read decimal 10 while the record quoted `010`.
case "$REPS" in
0 | *[!0-9]* | 0*) usage "--reps reads a positive decimal integer with no leading zero, got \`$REPS\`" ;;
esac

# The budget is passed to `go` verbatim, so its shape is checked here rather
# than discovered as a protocol refusal that this script would then have to tell
# apart from a refused POSITION — two reasons, two refusals (item 8).
BUDGET_KIND="${BUDGET%% *}"
BUDGET_VALUE="${BUDGET#* }"
case "$BUDGET_KIND" in
depth_turns | nodes | movetime) ;;
*) usage "--budget reads \`depth_turns N\`, \`nodes N\` or \`movetime N\`, got \`$BUDGET\`" ;;
esac
case "$BUDGET_VALUE" in
'' | *[!0-9]* | 0*) usage "--budget's value reads a positive decimal integer with no leading zero, got \`$BUDGET\`" ;;
esac
[ "$BUDGET" = "$BUDGET_KIND $BUDGET_VALUE" ] ||
	usage "--budget reads exactly two space-separated tokens, got \`$BUDGET\`"

[ -f "$ENGINE" ] || void "the engine \`$ENGINE\` is not a file"
[ -x "$ENGINE" ] || void "the engine \`$ENGINE\` is not executable"
[ -r "$CONFIG" ] || void "the config \`$CONFIG\` is not readable"
[ -r "$FIXTURE" ] || void "the fixture \`$FIXTURE\` is not readable"

SCRATCH="${TMPDIR:-/tmp}"
PREFLIGHT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/scratch_preflight.sh"
[ -x "$PREFLIGHT" ] || void "tools/scratch_preflight.sh is not executable beside this script"
"$PREFLIGHT" "$SCRATCH" ||
	void "no scratch room under \`$SCRATCH\` (tools/scratch_preflight.sh said so)"

WORK="$(mktemp -d "${SCRATCH%/}/pistol-testscratch-bench-block.XXXXXX")" ||
	void "a scratch directory could not be made under \`$SCRATCH\`"
# Item 7: one trap; the first statement takes the status the trap must return
# and the last returns it, so a housekeeping failure cannot turn a completed
# run into a failed one.
cleanup() {
	local rc=$?
	rm -rf -- "$WORK"
	return "$rc"
}
trap cleanup EXIT

# Item 1: the substitution's status is not the `printf`'s argument — take the
# value, check its shape, refuse by name.
ENGINE_DIGEST="$(sha256sum -- "$ENGINE" | cut -d' ' -f1)" ||
	void "the engine's digest could not be taken"
case "$ENGINE_DIGEST" in
[0-9a-f][0-9a-f][0-9a-f][0-9a-f]*) ;;
*) void "the engine's digest is not a hex digest: \`$ENGINE_DIGEST\`" ;;
esac
CONFIG_DIGEST="$(sha256sum -- "$CONFIG" | cut -d' ' -f1)" ||
	void "the config's digest could not be taken"
FIXTURE_DIGEST="$(sha256sum -- "$FIXTURE" | cut -d' ' -f1)" ||
	void "the fixture's digest could not be taken"

# THE FIXTURE READ. Both committed bench fixtures are covered and neither shape
# is inferred: `tail` is bench_positions_v1.txt's (a position-verb TAIL, then
# ` # …` commentary carrying `stones N`), `line` is spread_v1.txt's (whole
# `position …` lines, each preceded by a bare `stones N` line).
: >"$WORK/entries"
: >"$WORK/stones"
PENDING_STONES="-"
LINE_NO=0
while IFS= read -r raw || [ -n "$raw" ]; do
	LINE_NO=$((LINE_NO + 1))
	case "$raw" in
	'' | '#'*) continue ;;
	esac
	case "$GRAMMAR" in
	tail)
		entry="${raw%% #*}"
		stones="-"
		case "$raw" in
		*' stones '*)
			stones="${raw##* stones }"
			stones="${stones%% *}"
			;;
		esac
		;;
	line)
		case "$raw" in
		'stones '*)
			PENDING_STONES="${raw#stones }"
			continue
			;;
		'position '*) ;;
		*) fail "$FIXTURE:$LINE_NO under --grammar line is neither a \`position \` line nor a \`stones \` line: \`$raw\`" ;;
		esac
		entry="${raw#position }"
		stones="$PENDING_STONES"
		PENDING_STONES="-"
		;;
	esac
	# Trailing blanks would reach the protocol line and a `position ` with a
	# trailing space is a different string from one without; normalise here so
	# the record and the wire agree.
	entry="${entry%"${entry##*[![:space:]]}"}"
	[ -n "$entry" ] || fail "$FIXTURE:$LINE_NO states no position: \`$raw\`"
	case "$stones" in
	-) ;;
	'' | *[!0-9]*) fail "$FIXTURE:$LINE_NO carries a \`stones\` annotation that is not a decimal count: \`$stones\`" ;;
	esac
	printf '%s\n' "$entry" >>"$WORK/entries"
	printf '%s\n' "$stones" >>"$WORK/stones"
done <"$FIXTURE"

ENTRIES="$(wc -l <"$WORK/entries")"
[ "$ENTRIES" -gt 0 ] || fail "$FIXTURE states no entries under --grammar $GRAMMAR"

say "engine $ENGINE sha256 $ENGINE_DIGEST"
say "config $CONFIG sha256 $CONFIG_DIGEST"
say "fixture $FIXTURE sha256 $FIXTURE_DIGEST grammar $GRAMMAR entries $ENTRIES"
say "budget $BUDGET reps $REPS label ${LABEL:--}"

REFUSED=0
TOTALS=0
INDEX=0
while IFS= read -r entry <&3 && IFS= read -r stones <&4; do
	for rep in $(seq 1 "$REPS"); do
		# Both streams, because the refusal this script exists for is printed
		# on STDOUT and a future one may not be.
		printf 'newgame\nposition %s\ngo %s\nquit\n' "$entry" "$BUDGET" |
			"$ENGINE" --config "$CONFIG" >"$WORK/out" 2>&1 || true

		# THE PER-ENTRY GUARD. `grep` answers 1 on no match, which is the
		# healthy answer here, so its status is taken deliberately rather than
		# left to kill the pipeline under `pipefail` (item 3).
		errors="$(grep -c '^error ' "$WORK/out" || true)"
		if [ "$errors" -ne 0 ]; then
			REFUSED=$((REFUSED + 1))
			line="$(grep -m1 '^error ' "$WORK/out" || true)"
			fail "entry $INDEX rep $rep was REFUSED by the engine and the sweep is not aggregated: $line -- the entry was: \`$entry\`"
		fi

		count="$(grep -c '^info totals ' "$WORK/out" || true)"
		[ "$count" -eq 1 ] ||
			fail "entry $INDEX rep $rep produced $count \`info totals\` lines, wanted exactly 1 -- the entry was: \`$entry\`"

		totals="$(sed -n 's/^info totals //p' "$WORK/out")"
		[ -n "$totals" ] || fail "entry $INDEX rep $rep produced an empty totals line"
		printf 'bench_block: record entry %s stones %s rep %s %s\n' \
			"$INDEX" "$stones" "$rep" "$totals"
		TOTALS=$((TOTALS + 1))
	done
	INDEX=$((INDEX + 1))
done 3<"$WORK/entries" 4<"$WORK/stones"

WANTED=$((ENTRIES * REPS))
[ "$TOTALS" -eq "$WANTED" ] ||
	fail "$TOTALS record lines for $ENTRIES entries x $REPS reps (wanted $WANTED)"

say "done: $ENTRIES entries x $REPS reps = $TOTALS totals lines, $REFUSED refused"
