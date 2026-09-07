#!/usr/bin/env bash
#
# Resolve an external program by name, or refuse with the reason.
#
# Usage: tools/require_tool.sh <name>
# Exit:  0 resolved — the absolute path is on stdout
#        1 the caller called this wrong
#        2 the name does not resolve to a runnable external program; the reason
#          is named on stderr, and the CALLER decides whether that is its own
#          `fail` or its own `void`
#
# WHY THIS IS NOT `command -v`. `tools/SHELL_CHECKLIST.md` item 8 says a single
# combined test gives a wrong diagnosis, and names three reasons. MEASURED on
# this machine (bash 5.3.15), `command -v` admits FIVE outcomes and only one of
# them is "you may run this":
#
#   a real executable          ACCEPTED, and correct
#   an unfindable name         declined
#   a directory on PATH        declined — the SAME answer as unfindable, so a
#                              gate keying on the status alone reports the wrong
#                              cause
#   a FIFO on PATH             ACCEPTED, and every read of it then BLOCKS
#   a NON-EXECUTABLE file      ACCEPTED, path returned, and exec answers 126
#   a function/alias/builtin   ACCEPTED, and what comes back is the NAME, not a
#                              path — so the gate runs something that is not the
#                              program it checked for
#
# The last two are the fourth and fifth cases the ROADMAP's WP-1.10 left open.
# Both are EXIT-0-WRONG-ANSWER: the check says yes and the run does something
# else, which is the class this checklist exists for.

set -euo pipefail

say() { printf 'require_tool: %s\n' "$*" >&2; }
bug() { say "usage: require_tool.sh <name>; $*"; exit 1; }
no() { say "$*"; exit 2; }

[ "$#" -eq 1 ] || bug "got $# argument(s): $*"
NAME="$1"

# WHAT REACHES A RECORD IS CALLER-CONTROLLED (item 9), and the guard is an
# ALLOW-LIST so the locale makes it as WIDE as it can be rather than as narrow
# (item 4). The name is quoted back in every refusal below.
case "$NAME" in
'') bug "the name is empty" ;;
*[![:print:]]*) bug "the name holds a non-printable character" ;;
*/*) bug "\`$NAME\` is a path and not a name; this resolves names on PATH" ;;
esac

# ONE REFUSAL PER REASON (item 8). `type -t` names the KIND, and it is asked
# first because a function or alias shadowing the name is the case where
# `command -v` returns something that is not a path at all.
KIND="$(type -t -- "$NAME" 2>/dev/null || true)"
case "$KIND" in
file) ;;
'')
	# NOT YET "not found": a DIRECTORY of that name on PATH is invisible to
	# both `type` and `command -v`, so concluding here would give an absent
	# name and a directory the SAME diagnosis — the conflation item 8 names,
	# reproduced by the very script written to end it (caught by this
	# script's own test, which is why it has one). PATH is walked instead.
	# `IFS=:` in a subshell-free read so an entry containing a space survives.
	SHADOW=""
	while IFS= read -r -d ':' ENTRY || [ -n "$ENTRY" ]; do
		[ -n "$ENTRY" ] || ENTRY="."
		[ -e "$ENTRY/$NAME" ] || continue
		SHADOW="$ENTRY/$NAME"
		break
	done <<<"${PATH}:"
	[ -z "$SHADOW" ] || {
		[ ! -d "$SHADOW" ] ||
			no "\`$NAME\` names a DIRECTORY at \`$SHADOW\`, which is on PATH and is not a program — an absent name would say so differently"
		no "\`$NAME\` exists at \`$SHADOW\` and is not a program PATH lookup will hand back"
	}
	no "no \`$NAME\` on PATH"
	;;
*) no "\`$NAME\` resolves to a shell $KIND, not a program on PATH — a gate that
runs it runs something other than the tool it checked for" ;;
esac

# The status is DISCARDED deliberately here and the VALUE is checked below
# (item 1): `type -t` has already said this is a `file`, so an empty answer
# would be a disagreement between two builtins and is refused as one.
FOUND="$(command -v -- "$NAME" 2>/dev/null || true)"
[ -n "$FOUND" ] ||
	no "\`$NAME\` is a \`file\` to \`type\` and nothing to \`command -v\`; the two disagree and this is not a tool to run"
case "$FOUND" in
/*) ;;
*) no "\`$NAME\` resolved to \`$FOUND\`, which is not an absolute path" ;;
esac

# THE THREE THINGS `command -v` GETS WRONG, each answered on its own.
[ ! -d "$FOUND" ] || no "\`$NAME\` resolves to \`$FOUND\`, which is a directory"
[ -f "$FOUND" ] ||
	no "\`$NAME\` resolves to \`$FOUND\`, which is not a regular file — a FIFO there blocks every read rather than failing"
[ -x "$FOUND" ] ||
	no "\`$NAME\` resolves to \`$FOUND\`, which carries no execute bit — exec answers 126, in the kernel's vocabulary and not this gate's"

printf '%s\n' "$FOUND"
