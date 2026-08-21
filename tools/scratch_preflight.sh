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
# clone and its whole debug target tree together. The floor is three times that,
# which leaves room for the stub workspaces the test suites build under
# `$TMPDIR` alongside it and is still under 5% of this machine's `/tmp`.
#
# THE FLOOR CAN BE RAISED AND NOT LOWERED. `PISTOL_MIN_SCRATCH_KIB` exists so
# that a test can watch the refusal fire — item 10 wants a test driving the
# SHIPPED script, and nothing else can make a 24 GiB tmpfs look full. It is
# combined with the constant by MAXIMUM, so the binding can only ever tighten
# this check: a caller who sets it to zero gets the constant.

set -euo pipefail

# In KiB, matching `df -Pk`. See the header for where the number comes from.
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

command -v df >/dev/null || void "df is not on PATH, so available space cannot be read"
command -v awk >/dev/null || void "awk is not on PATH, so df's output cannot be read"

# `-P` is the POSIX one-line-per-filesystem format, which is what makes field
# 4 a field rather than a guess; `-k` fixes the unit so no locale or `BLOCKSIZE`
# can change what the number means. THE LOCALE PIN IS FOR DETERMINISM AND MOVES
# NO GUARD (item 4): it fixes df's own column wording and its decimal separator,
# and nothing here is a character class.
DF_LINE="$(LC_ALL=C df -Pk -- "$DIR" | sed -n '2p')" ||
	void "df could not read the filesystem holding \`$DIR\`"
[ -n "$DF_LINE" ] || void "df printed no filesystem line for \`$DIR\`"

FS="$(printf '%s\n' "$DF_LINE" | awk '{ print $1 }')"
AVAIL="$(printf '%s\n' "$DF_LINE" | awk '{ print $4 }')"
MOUNT="$(printf '%s\n' "$DF_LINE" | awk '{ print $NF }')"

# ONE SPELLING PER NUMBER (item 8). A leading zero is read as octal by every
# arithmetic context in bash, so `08` would be a syntax error and `010` would
# silently be eight.
case "$AVAIL" in
'' | *[!0-9]*) void "df's available column is not a number for \`$DIR\`: \`$AVAIL\`" ;;
0?*) void "df's available column is not written in decimal for \`$DIR\`: \`$AVAIL\`" ;;
esac

FLOOR="$MIN_SCRATCH_KIB"
if [ -n "${PISTOL_MIN_SCRATCH_KIB:-}" ]; then
	RAISED="$PISTOL_MIN_SCRATCH_KIB"
	case "$RAISED" in
	'' | *[!0-9]*) bug "PISTOL_MIN_SCRATCH_KIB is not a number: \`$RAISED\`" ;;
	0?*) bug "PISTOL_MIN_SCRATCH_KIB is not written in decimal: \`$RAISED\`" ;;
	esac
	# MAXIMUM, so the binding tightens and never loosens.
	[ "$RAISED" -le "$FLOOR" ] || FLOOR="$RAISED"
fi

if [ "$AVAIL" -lt "$FLOOR" ]; then
	void "$MOUNT ($FS) has $AVAIL KiB available and this run wants $FLOOR KiB; \
nothing was measured and nothing failed — free space and take the run again"
fi

say "$MOUNT ($FS) has $AVAIL KiB available, floor $FLOOR KiB"
