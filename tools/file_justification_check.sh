#!/usr/bin/env bash
#
# CLAUDE.md rule 9's soft cap, mechanized: a tracked `.rs`, `.sh` or `.py` file
# over the cap has to say WHY, and that why lives in
# `docs/rule9_justifications.md` (docs/decisions.md D-131 amending D-118; D-234
# for the `.sh` widening; D-467 for the registry; D-679 for the `.py` one).
#
# WHY `.py` COULD NOT BE ADDED BEFORE, AND WHY IT CAN NOW. D-415 ruled the
# widening right in principle and wrong to couple to its cost: a why was then a
# MARKER COMMENT IN THE FILE, so widening meant EDITING two frozen instruments
# named with their revisions by pre-registrations. D-467 moved every why into
# the registry, which dissolved the coupling — the gate reads a document now, so
# a Python instrument comes under the cap without a byte of it changing.
#
# THE WHY LIVES IN ONE PLACE, AND IT IS NOT THE FILE. It used to be a marker
# comment in the file's own header, inside the blocks D-443's sweep was
# deleting: the sweep took 30 of them out and this gate went red on 30 files
# whose argument nobody had withdrawn. A why kept beside the code it argues
# about is one a comment-only sweep can delete by accident.
#
# The four things this gate is, precisely:
#
#   1. A cap. 300 lines, which is what rule 9's "~300-line soft cap" is worth as
#      a number a script can compare against. Over it is not a failure — rule 9
#      calls the cap soft and means it — it is a demand for a sentence. The cap
#      is one number for all three suffixes: rule 9 says "Files", and a
#      per-language cap would be this gate deciding a question rule 9 did not
#      ask it.
#   2. A registry. One `- ``path``: why` entry per over-cap file, and those
#      entries are the only top-level `- ` lines the document has. A checker has
#      to RECOGNIZE a justification, and prose cannot be recognized, so the
#      entry form is the interface.
#   3. A count ban. Rule 9 says counts are derived and never asserted, so a why
#      that says how long the file is fails the gate. What this catches is the
#      literal form — a numeral beside "line" or "lines". A count spelled out in
#      words gets past it, and that is named here rather than implied: the check
#      is a guard against the obvious mistake, not a proof.
#   4. A staleness check. An entry naming a path nothing tracks, an entry naming
#      a file under the cap, and two entries for one path are all refused. Each
#      is a claim that does no work, and the third is the drift a single home is
#      supposed to make impossible.
#
# THE GATE READS THE TRACKED BYTES AND NOT THE WORKING TREE (docs/decisions.md
# D-233's fix, applied to the sibling gate it was left out of). `git ls-files`
# named the path and `wc -l`/`grep` then opened THE WORKTREE FILE OF THAT NAME,
# which is a different file: staging an over-cap unjustified source file and
# overwriting its worktree copy with two lines made this gate print `0 over the
# cap, all justified` and exit 0 while the real file went to HEAD — the same
# exit-0-wrong-answer the artifact gate had, reproduced here at ccba146 with one
# `.rs` and one `.sh`. What commits is the INDEX, so the index's blob is what is
# read — the registry's blob included, since an unstaged registry edit is not
# what a fresh clone will be judged on either.
#
# The gate self-tests before it runs, over seeded files and a seeded registry in
# a temporary directory: a checker nobody has watched fail is not a checker, and
# this one is asked to say "no" about a file that will not exist until somebody
# writes it.
#
# Usage: tools/file_justification_check.sh
# Exit:  0 every tracked .rs/.sh/.py file is under the cap or registered,
#        1 otherwise,
#        2 THE RUN IS VOID — no verdict was taken (item 12).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"


fail() { printf 'file_justification_check: FAIL: %s\n' "$*" >&2; exit 1; }
# THE VOID, NAMED (item 12 obligation 1): a filesystem with no room for the
# seed tree is not a rule-9 finding.
void() { printf 'file_justification_check: RUN VOID: %s\n' "$*" >&2; exit 2; }

# Programs are resolved through the resolver, never `command -v` — item 8's
# table and docs/decisions.md D-683 say why. This script keeps its own class.
REQUIRE_TOOL="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/require_tool.sh"
[ -x "$REQUIRE_TOOL" ] || fail "the tool resolver is missing beside this script: $REQUIRE_TOOL"

"$REQUIRE_TOOL" git >/dev/null || fail "git is not usable"

# The cap, and where every why for a file over it is written down.
SOFT_CAP=300
REGISTRY='docs/rule9_justifications.md'
ENTRY_RE='^- `([^`]+)`: (.*)$'

declare -A WHY=()
REGISTERED=0

# The registry, into `WHY`. A line that opens like an entry and is not one is a
# refusal rather than a skip: an entry with a typo in it would otherwise read as
# no entry at all, and the file it belongs to as unjustified for a reason its
# author cannot see (rule 3).
parse_registry() {
	local file="$1" line path
	WHY=()
	REGISTERED=0
	[ -f "$file" ] || fail "$file: the registry every why is read from is not there"
	# `|| [ -n "$line" ]` because a registry whose last line carries no newline
	# still HAS that line, and dropping it silently turned the duplicate, stale
	# and malformed refusals into exit 0 on a registry built to fail them.
	while IFS= read -r line || [ -n "$line" ]; do
		[[ "$line" == "- "* ]] || continue
		[[ "$line" =~ $ENTRY_RE ]] ||
			fail "$REGISTRY: an entry that is not of the form '- \`path\`: why': $line"
		path="${BASH_REMATCH[1]}"
		[ -z "${WHY[$path]+set}" ] ||
			fail "$REGISTRY: $path has two entries, and two whys for one file is the drift one home exists to prevent"
		WHY["$path"]="${BASH_REMATCH[2]}"
		REGISTERED=$((REGISTERED + 1))
	done <"$file"
}

# One file's verdict, on stdout: `under`, `justified`, `unjustified`, `empty`,
# `counted` or `missing`. Everything this gate knows about a file is in here, so
# that the seeded self-test below exercises the same function the tracked file
# set goes through. `bytes` is what to measure and `path` is what to look up —
# two arguments because the tracked loop measures an extracted blob and looks up
# the name it was extracted for.
verdict() {
	local bytes="$1" path="$2" lines why
	# Nothing to read. From the tracked loop this is unreachable — the bytes it
	# passes are an extracted blob, and an extraction that failed is refused
	# there by name — so what this guards is the seeded self-test below and any
	# future caller that hands over a path. Without it `wc -l` fails, `lines` is
	# empty, the `[` test errors, and the file is misdiagnosed as an unregistered
	# over-cap file: a wrong answer where a named refusal belongs (rule 3).
	if [ ! -f "$bytes" ]; then
		echo "missing"
		return
	fi
	# `awk END{NR}` and not `wc -l`: newlines are not lines, so an unterminated
	# last line would read one short (the registry parser below already knows).
	lines="$(awk 'END { print NR }' <"$bytes")"
	if [ "$lines" -le "$SOFT_CAP" ]; then
		echo "under"
		return
	fi
	if [ -z "${WHY[$path]+set}" ]; then
		echo "unjustified"
		return
	fi
	why="${WHY[$path]}"
	if [ -z "$why" ]; then
		echo "empty"
		return
	fi
	# "counts are derived, never asserted" (rule 9).
	if grep -qiE '[0-9]+[[:space:]]*-?[[:space:]]*lines?([^a-z]|$)|lines?[[:space:]]+[0-9]+' <<<"$why"; then
		echo "counted"
		return
	fi
	echo "justified"
}

# --- the self-test, on files nobody tracks --------------------------------

# SCRATCH SPACE, ASKED FOR BEFORE THE WORK (tools/SHELL_CHECKLIST.md item 12
# obligation 2, docs/decisions.md D-285). Discovering the shortage through
# `mktemp`'s own error message is discovering it in `mktemp`'s vocabulary, and
# that vocabulary describes `mktemp` rather than this gate.
PREFLIGHT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/scratch_preflight.sh"
[ -x "$PREFLIGHT" ] || void "the scratch preflight is missing beside this script: $PREFLIGHT"
"$PREFLIGHT" "${TMPDIR:-/tmp}" ||
	void "no scratch room; the lines above name the filesystem"

SEED="$(mktemp -d)"
# Item 7: the trap's LAST command decides the status, so a scratch directory
# that will not remove would turn a chosen void (2) into a fail (1) — the exact
# reading item 12 exists to prevent.
trap 'rc=$?; rm -rf "$SEED"; exit "$rc"' EXIT

seed() {
	local name="$1" filler="${2:-// filler}"
	for _ in $(seq 1 "$((SOFT_CAP + 1))"); do echo "$filler"; done >"$SEED/$name"
}

seed over_unjustified.rs
seed over_justified.rs
seed over_empty.rs
seed over_counted.rs
printf 'fn main() {}\n' >"$SEED/under.rs"
# The shell spelling, seeded for the same reason the Rust ones are: the widening
# is only worth having if the checker has been watched accept and refuse a file
# named with it (docs/decisions.md D-234).
seed over_justified.sh "# filler"
# And the Python spelling, seeded for exactly the reason the shell one is
# (docs/decisions.md D-234, D-679): a suffix this gate has never been watched
# accept and refuse is a suffix it is trusted about rather than tested about.
seed over_justified.py "# filler"

cat >"$SEED/registry.md" <<'SEEDED'
Prose the parser walks past, including a line that opens like an entry
- but is not one, because it is prose and the parser sees that.
SEEDED
# Written after the heredoc so the malformed line above can be dropped for the
# accepting cases and restored for the refusing one.
grep -v '^- but is not one' "$SEED/registry.md" >"$SEED/registry-clean.md"
{
	echo '- `over_justified.rs`: the recursion'"'"'s parts are not independent'
	echo '- `over_empty.rs`: '
	echo '- `over_counted.rs`: it is 348 lines and every one earns its place'
	echo '- `over_justified.sh`: one measurement over one pre-registration'
	echo '- `over_justified.py`: one instrument, and its refusals are its argument'
} >>"$SEED/registry-clean.md"

parse_registry "$SEED/registry-clean.md"

expect() {
	local got
	got="$(verdict "$SEED/$1" "$1")"
	[ "$got" = "$2" ] ||
		fail "self-test: seeded $1 should be '$2' and the check said '$got'"
}

expect over_unjustified.rs unjustified
expect over_justified.rs justified
expect over_empty.rs empty
expect over_counted.rs counted
expect under.rs under
expect no_such_file.rs missing
expect over_justified.sh justified
expect over_justified.py justified
[ "$REGISTERED" -eq 5 ] ||
	fail "self-test: the seeded registry has five entries and the parser found $REGISTERED"

# The parser's own two refusals, in a subshell because each one exits.
( parse_registry "$SEED/registry.md" ) 2>/dev/null &&
	fail "self-test: a line opening like an entry and malformed was not refused"
{
	cat "$SEED/registry-clean.md"
	echo '- `over_justified.rs`: a second why for a file that already has one'
} >"$SEED/registry-doubled.md"
( parse_registry "$SEED/registry-doubled.md" ) 2>/dev/null &&
	fail "self-test: two entries for one path were not refused"

echo "file_justification_check: self-test passed on 8 seeded verdicts and 2 seeded registry refusals (cap $SOFT_CAP)"

# --- the tracked file set -------------------------------------------------

git cat-file -e ":$REGISTRY" 2>/dev/null ||
	fail "$REGISTRY is not in the index, and the index is what this gate reads — stage it"
git cat-file blob ":$REGISTRY" >"$SEED/registry-tracked.md"
parse_registry "$SEED/registry-tracked.md"

OVER=0
TRACKED=0
BAD=()
declare -A USED=()
# Where each tracked blob is unpacked, one at a time. Inside `$SEED` so the trap
# already set above removes it: a second `mktemp -d` would need a second trap,
# and a second trap on EXIT REPLACES the first (docs/decisions.md D-231's leak).
BLOB="$SEED/tracked-blob"
# `-s -z` prints `<mode> SP <object> SP <stage> TAB <path>` per NUL-terminated
# record — the only spelling that survives a path containing a newline.
while IFS= read -r -d '' entry; do
	meta="${entry%%$'\t'*}"
	file="${entry#*$'\t'}"
	meta="${meta#* }"
	blob="${meta%% *}"
	git cat-file blob "$blob" >"$BLOB" 2>/dev/null ||
		fail "git could not read the tracked blob $blob for $file, and a file this gate cannot read is not one it may pass"
	TRACKED=$((TRACKED + 1))
	case "$(verdict "$BLOB" "$file")" in
	under) ;;
	justified)
		OVER=$((OVER + 1))
		USED["$file"]=1
		echo "file_justification_check: over the cap and registered: $file"
		;;
	unjustified) BAD+=("$file: over the cap with no entry in $REGISTRY") ;;
	empty)
		USED["$file"]=1
		BAD+=("$file: its $REGISTRY entry carries no why")
		;;
	counted)
		USED["$file"]=1
		BAD+=("$file: its $REGISTRY entry states a line count, and counts are derived")
		;;
	missing) BAD+=("$file: its tracked blob unpacked to nothing this gate could read") ;;
	*) fail "$file: verdict said something this gate has no arm for, and an unread verdict is not a pass" ;;
	esac
done < <(git ls-files -s -z '*.rs' '*.sh' '*.py')

# An entry nothing above claimed argues about a file this gate never weighs.
for path in "${!WHY[@]}"; do
	[ -z "${USED[$path]+set}" ] || continue
	# `ls-files` and not `cat-file -e ":$path"`: the latter resolves `0:big.rs` as
	# a stage spec and answers about a file the entry does not name, so a refusal
	# would carry the wrong reason (rule 3). Literal pathspecs so an entry
	# containing a glob character cannot match some other file either.
	if [ -n "$(GIT_LITERAL_PATHSPECS=1 git ls-files -z -- "$path" 2>/dev/null)" ]; then
		BAD+=("$REGISTRY: an entry for $path, which is under the cap or outside the .rs/.sh/.py set this gate weighs")
	else
		BAD+=("$REGISTRY: an entry for $path, which nothing tracks")
	fi
done

if [ "${#BAD[@]}" -gt 0 ]; then
	printf 'file_justification_check: %s\n' "${BAD[@]}" >&2
	fail "${#BAD[@]} finding(s) against CLAUDE.md rule 9's soft cap"
fi

echo "file_justification_check: $TRACKED tracked .rs/.sh/.py files, $OVER over the cap, all registered in $REGISTRY ($REGISTERED entries)"
