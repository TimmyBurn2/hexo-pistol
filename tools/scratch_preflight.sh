#!/usr/bin/env bash
#
# Is there room under <directory> for the work about to be done there?
#
# Usage: tools/scratch_preflight.sh <directory>
# Exit:  0 there is room — the number is printed
#        2 THE RUN IS VOID: there is not, or the question could not be answered
#        1 the caller called this wrong
#
# WHY THIS IS ITS OWN SCRIPT. `tools/SHELL_CHECKLIST.md` item 12: two different
# things exit non-zero — "the answer is no" and "I could not take the answer" —
# and a gate that spells them the same way turns an environmental accident into a
# regression report. MEASURED, and this is the accident that produced the item:
# `/tmp` on this machine is RAM-backed at 24 GiB, a session filled it, `cargo`
# answered `Disk quota exceeded (os error 122)`, and
# `tools/solver_link_check.sh` correctly exited 2 with `cannot build the
# workspace's binaries` — which read, in the log and to the standing test, as a
# solver-link REGRESSION (docs/decisions.md D-281, D-285).
#
# Discovering a shortage through a tool's own error message is discovering it in
# THE TOOL'S vocabulary, and that vocabulary describes the tool. This asks the
# question in the gate's vocabulary, before the work, and answers it with the
# filesystem, the number available and the number wanted.
#
# THE FLOOR IS MEASURED, NOT GUESSED. The largest scratch consumer in `tools/` is
# `tools/ci.sh` gate 2, which unpacks the whole tracked file set into a temporary
# directory and builds it: measured at d6f6cbb, `du -sk` on that directory after
# `cargo build --workspace --locked` reports 340552 KiB — about 333 MiB, the
# clone and its whole debug target tree together. The floor is three times that
# rounded up to 1 GiB (1048576 KiB, i.e. 3.079x rather than 3x),
# which leaves room for the stub workspaces the test suites build under
# `$TMPDIR` alongside it and is still under 5% of this machine's `/tmp`.
#
# THE FLOOR CAN BE RAISED AND NOT LOWERED. `PISTOL_MIN_SCRATCH_KIB` exists so
# that a test can watch the refusal fire — item 10 wants a test driving the
# SHIPPED script, and nothing else can make a 24 GiB tmpfs look full. It is
# combined with the constant by MAXIMUM, so the binding can only ever tighten
# this check: a caller who sets it to zero gets the constant.

set -euo pipefail

# In KiB. See the header for where the number comes from.
MIN_SCRATCH_KIB=1048576

say() { printf 'scratch_preflight: %s\n' "$*"; }
# THE VOID, NAMED. Not `fail`: this script never says "the answer is no".
void() {
	printf 'scratch_preflight: RUN VOID: %s\n' "$*" >&2
	exit 2
}
bug() {
	printf 'scratch_preflight: usage: %s\n' "$*" >&2
	exit 1
}

[ "$#" -eq 1 ] || bug "scratch_preflight.sh <directory>; got $# argument(s): $*"
DIR="$1"

# WHAT REACHES A RECORD IS CALLER-CONTROLLED (item 9): the path is quoted back
# into every message below. The guard is an ALLOW-LIST and the locale is NOT
# pinned around it, so it is as wide as the ambient locale can make it (item 4).
case "$DIR" in
'') bug "the directory is empty" ;;
*[![:print:]]*) bug "the directory holds a non-printable character" ;;
esac
[ -d "$DIR" ] || void "no such directory to preflight: \`$DIR\`"

command -v stat >/dev/null || void "stat is not on PATH, so available space cannot be read"

# NOT `df`, AND THIS IS THE REASON. `df` answers in COLUMNS, and a mount source
# containing a space shifts every column left: field 4 stops being Available and
# becomes Used. MEASURED on a real tmpfs mounted as `my dev` — an empty 2 GiB
# filesystem reported `0 KiB available` and VOIDED a healthy run, and the same
# filesystem with about 1 MiB left reported `2096132 KiB available` and PASSED.
# Both directions at once, and the spelling guard below cannot see either,
# because Used is also a well-formed decimal. That is EXIT-0-WRONG-ANSWER, and
# the fix is not a better parse but NO PARSE: `stat -f` answers one field per
# `-c` directive and has no columns to shift.
#
# `%a` is the blocks free to an unprivileged caller — which is the number this
# question is about — and `%S` the block size; `%c`/`%d` are total and free
# inodes, and a filesystem out of INODES has space it cannot use.
STAT_OUT="$(LC_ALL=C stat -f -c '%a %S %c %d' -- "$DIR")" ||
	void "stat could not read the filesystem holding \`$DIR\`"
read -r BLOCKS BLOCKSIZE INODES_TOTAL INODES_FREE <<<"$STAT_OUT" ||
	void "stat's answer for \`$DIR\` could not be read: \`$STAT_OUT\`"

# ONE SPELLING PER NUMBER (item 8), on each field, before any arithmetic. A
# leading zero is read as octal by every arithmetic context in bash.
for FIELD in "$BLOCKS" "$BLOCKSIZE" "$INODES_TOTAL" "$INODES_FREE"; do
	case "$FIELD" in
	'' | *[!0-9]*) void "stat's answer for \`$DIR\` is not a number: \`$STAT_OUT\`" ;;
	0?*) void "stat's answer for \`$DIR\` is not written in decimal: \`$STAT_OUT\`" ;;
	esac
done
[ "$BLOCKSIZE" -gt 0 ] || void "stat reports a zero block size for \`$DIR\`: \`$STAT_OUT\`"

AVAIL="$(( BLOCKS * BLOCKSIZE / 1024 ))"
MOUNT="$DIR"
# WHICH filesystem, named without a column: the device number `stat` reports for
# the directory itself. Not a mount source — a mount source is exactly the field
# whose spacing broke the parse this replaced.
DEVICE="$(LC_ALL=C stat -c '%D' -- "$DIR")" ||
	void "stat could not name the device holding \`$DIR\`"

# INODES ARE SPACE TOO, and they run out separately. A filesystem reporting
# gigabytes free and zero free inodes refuses every create, and the tool that
# then fails describes ITSELF rather than the filesystem — the same seam this
# script exists to close. `%c` is 0 where a filesystem does not report inodes
# (btrfs among them), which is not an exhaustion and is not read as one.
if [ "$INODES_TOTAL" -gt 0 ] && [ "$INODES_FREE" -eq 0 ]; then
	void "$MOUNT has $AVAIL KiB available but NO FREE INODES, so nothing can be \
created there; nothing was measured and nothing failed"
fi

FLOOR="$MIN_SCRATCH_KIB"
if [ -n "${PISTOL_MIN_SCRATCH_KIB:-}" ]; then
	RAISED="$PISTOL_MIN_SCRATCH_KIB"
	case "$RAISED" in
	'' | *[!0-9]*) bug "PISTOL_MIN_SCRATCH_KIB is not a number: \`$RAISED\`" ;;
	0?*) bug "PISTOL_MIN_SCRATCH_KIB is not written in decimal: \`$RAISED\`" ;;
	esac
	# AND IT MUST FIT, or the guard fails OPEN. `[ x -le y ]` on a value above
	# 2^63-1 is an ERROR, not a comparison, and an erroring `[` in a CONDITION is
	# exempt from `set -e` (item 2) — so the floor silently stayed at the
	# constant and the check passed. MEASURED: the boundary is exact,
	# 9223372036854775807 refuses and ...808 exited 0.
	# The boundary is EXACT and both sides are tested: 9223372036854775807 is
	# representable and must still REFUSE (exit 2); one past it is a caller bug.
	# Equal-length digit strings compare correctly lexicographically, and the
	# locale is pinned so the collation cannot widen (item 4).
	if [ "${#RAISED}" -gt 19 ] ||
		{ [ "${#RAISED}" -eq 19 ] &&
			[ "$(LC_ALL=C printf '%s\n' "$RAISED" "9223372036854775807" | LC_ALL=C sort | tail -n1)" = "$RAISED" ] &&
			[ "$RAISED" != "9223372036854775807" ]; }; then
		bug "PISTOL_MIN_SCRATCH_KIB does not fit the arithmetic that compares it: \`$RAISED\`"
	fi
	# MAXIMUM, so the binding tightens and never loosens.
	[ "$RAISED" -le "$FLOOR" ] || FLOOR="$RAISED"
fi

if [ "$AVAIL" -lt "$FLOOR" ]; then
	void "$MOUNT (device $DEVICE) has $AVAIL KiB available and this run wants $FLOOR KiB; \
nothing was measured and nothing failed — free space and take the run again"
fi

say "$MOUNT (device $DEVICE) has $AVAIL KiB available, floor $FLOOR KiB"
