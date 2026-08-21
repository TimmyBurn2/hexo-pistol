#!/usr/bin/env bash
#
# Every `D-<n>` key in docs/decisions.md appears exactly once.
#
# WHY THIS EXISTS. `D-276` and `D-277` were each appended TWICE, in one commit,
# with DIFFERENT TEXT, and nothing detected it (docs/decisions.md D-279). The log
# is append-only and its header says a decision that changes is superseded by a
# new line naming the one it replaces, so both copies stay and a later line says
# which is operative — but `D-276` denoting two different texts breaks the one
# property this log is cited for, and EVERY ADR reference in this repository is
# by number. `tools/SHELL_CHECKLIST.md` item 11 exists for the same reason at
# smaller scale.
#
# D-279 named this check — `grep -oE '^D-[0-9]+' docs/decisions.md | sort |
# uniq -d` must be empty — and DELIBERATELY DID NOT LAND IT, on the ground that a
# gate landing in the same commit as the defect it would have caught is a gate
# whose own first run is its author's word. It lands here, with a seeded
# violation in its own run and a test suite driving the shipped script.
#
# TWO THINGS THIS IS NOT. It is not a check that the numbering is dense or
# ascending: a gap is not a defect and the log has never promised one. And it is
# not a check of the ADR's CONTENT — that a line means what it says is judged,
# not mechanized (CLAUDE.md rule 10).
#
# AND IT CANNOT BE THE COMMAND D-279 WROTE DOWN, which is a finding about D-279
# and is recorded at docs/decisions.md D-284. D-279 registered
# «`grep -oE '^D-[0-9]+' docs/decisions.md | sort | uniq -d` must be empty» and
# in the same line REFUSED TO DELETE the two repeated copies, on the log's own
# «lines are never edited or deleted» rule, resolving the ambiguity by a RULING
# instead: the second copy of each is operative. Those two sentences cannot both
# hold. Run against the file D-279 left behind, that pipeline prints `D-276` and
# `D-277` — measured, on the first run of this script — so a gate spelled exactly
# as registered is red on arrival and always would have been. The prescription
# was never executed against its own subject, which is the defect class this
# repository keeps finding one level up from wherever it is looking.
#
# So the rule this gate ENFORCES is: no key repeats EXCEPT the two D-279
# dispositioned by name, and that exemption is checked in BOTH directions — a
# key in the list that is no longer repeated is refused too, so the list cannot
# outlive its subject and cannot quietly become the place repeats go. It is
# printed on every run rather than hidden in the source, because an exemption
# nobody sees is an exemption nobody re-reads (docs/decisions.md D-275's lesson
# about lists maintained by memory, which this is one of and says so).
#
# THE PASSING CASE IS `grep` FINDING NOTHING. `grep` exits 1 on no match and
# under `pipefail` that takes the whole pipeline down one line before the refusal
# written for it (tools/SHELL_CHECKLIST.md item 3), so the extraction carries
# `|| true` and its EMPTY result is then refused by name — while `sort` and
# `uniq -d`, which exit 0 on empty input, carry nothing. Naming which of the
# three needs it, and why the other two do not, is the item.
#
# Usage: tools/decision_key_check.sh
# Exit:  0 every key is unique, 1 a key repeats or the extraction is wrong.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

DOC="docs/decisions.md"

fail() { printf 'decision_key_check: FAIL: %s\n' "$*" >&2; exit 1; }

# ARGUMENTS ARE NOT SILENTLY IGNORED. Four sibling gates read `$@` not at all and
# exit 0 on a misspelled flag, having run the default (docs/decisions.md D-251,
# MINOR-3); this one does not join them.
[ "$#" -eq 0 ] || fail "this gate takes no arguments and was given: $*"

command -v git >/dev/null || fail "git is not on PATH"
git rev-parse --is-inside-work-tree >/dev/null 2>&1 ||
	fail "not a git repository: this gate reads the TRACKED bytes of $DOC"

# Every `D-<n>` at the start of a line, one per line. ANCHORED, because a
# substring is not a token (item 3): `D-27` inside prose is not a key and `^`
# is what says so.
keys_of() { # $1 = a file to read
	grep -oE '^D-[0-9]+' -- "$1" || true
}

# --- the seeded violation, before the tracked file ---------------------------
#
# A gate nobody has watched refuse is not a gate, and this one is asked to say
# "no" about a shape that must never appear in the file it guards — so the only
# place it can be watched refusing is a file seeded on purpose.

SEED="$(mktemp -d)"
trap 'rm -rf "$SEED"' EXIT

printf 'D-1: a choice — a reason — what flips it\nD-2: another\nprose mentioning D-1 mid-line\n' \
	>"$SEED/clean.md"
printf 'D-1: a choice — a reason — what flips it\nD-2: another\nD-1: the same key, different text\n' \
	>"$SEED/seeded.md"

SEED_CLEAN="$(keys_of "$SEED/clean.md" | LC_ALL=C sort | uniq -d)"
[ -z "$SEED_CLEAN" ] ||
	fail "self-test: the clean seed has no repeated key and the check found: $SEED_CLEAN"
SEED_DIRTY="$(keys_of "$SEED/seeded.md" | LC_ALL=C sort | uniq -d)"
[ "$SEED_DIRTY" = "D-1" ] ||
	fail "self-test: the seeded violation is D-1 and the check found: \`$SEED_DIRTY\`"
# And the anchor holds: the clean seed mentions D-1 a second time MID-LINE, so a
# check that dropped `^` would report it as a duplicate and this line would fail.
SEED_KEYS="$(keys_of "$SEED/clean.md" | wc -l)"
[ "$SEED_KEYS" -eq 2 ] ||
	fail "self-test: the clean seed holds 2 keys and the extraction counted $SEED_KEYS"
echo "decision_key_check: self-test passed — a clean seed, a seeded repeat, and the anchor"

# --- the tracked bytes -------------------------------------------------------
#
# THE INDEX IS WHAT COMMITS (tools/SHELL_CHECKLIST.md item 5). `git ls-files`
# names a PATH, and opening that path reads the WORKTREE file of that name: stage
# a repeated key and overwrite the worktree copy with something clean, and a gate
# reading paths passes it while the real bytes go to HEAD. That is not a
# hypothetical here — it is the defect the sibling gate had, reproduced
# (docs/decisions.md D-233).

BLOB="$SEED/tracked-blob"
TRACKED=0
# `-s -z` prints `<mode> SP <object> SP <stage> TAB <path>` per NUL-terminated
# record, the only spelling that survives a path containing a newline.
while IFS= read -r -d '' entry; do
	meta="${entry%%$'\t'*}"
	meta="${meta#* }"
	object="${meta%% *}"
	git cat-file blob "$object" >"$BLOB" 2>/dev/null ||
		fail "git could not read the tracked blob $object for $DOC"
	TRACKED=$((TRACKED + 1))
done < <(git ls-files -s -z -- "$DOC")

[ "$TRACKED" -eq 1 ] ||
	fail "$DOC resolves to $TRACKED tracked blobs; this gate reads exactly one"

KEYS="$(keys_of "$BLOB")"
[ -n "$KEYS" ] ||
	fail "no \`D-<n>\` key at the start of any line of $DOC; the EXTRACTION is wrong, not the file"
COUNT="$(printf '%s\n' "$KEYS" | wc -l)"
# Validate the SPELLING, not only the value (item 8): a `wc` that printed
# nothing would make the comparison below an error, not an answer.
case "$COUNT" in
'' | *[!0-9]*) fail "the key count is not a number: \`$COUNT\`" ;;
esac

# `sort` and `uniq` exit 0 on empty input, so neither needs the `|| true` the
# extraction above does — which is the whole of item 3 stated precisely.
#
# THE MULTIPLICITY, NOT THE KEY, and this is the correction D-296 makes to the
# first spelling. `uniq -d` prints a repeated key ONCE however many times it
# occurs, so a set comparison against an exempted KEY cannot tell the two copies
# D-279 ruled on from a THIRD one. Reproduced: a third `D-276` with a third
# different text, staged, and this gate printed «no repeat outside the
# exemption» and exited 0 — the log's «one key, one text» property broken and
# reported as held. `uniq -c` carries the count, so a third copy is a different
# record and falls outside the exemption on its own.
DUPES="$(printf '%s\n' "$KEYS" | LC_ALL=C sort | uniq -c |
	awk '$1 > 1 { print $2, $1 }' | LC_ALL=C sort)"

# THE CLOSED EXEMPTION. These are the two keys docs/decisions.md D-279 found
# appended twice and ruled on rather than deleted, WITH THE COUNT EACH WAS
# GRANTED; nothing may join them, and neither may grow, without an ADR line
# saying why the log's «one key, one text» property is being spent again.
# Spelled as a sorted list of `<key> <count>` so the comparisons below are on the
# same shape the extraction produces.
GRANDFATHERED='D-276 2
D-277 2'
echo "decision_key_check: grandfathered by D-279 (ruled, not deleted): $(printf '%s' "$GRANDFATHERED" | tr '\n' ' ')"

# BOTH DIRECTIONS. An unexpected repeat is the defect this gate exists for; an
# exemption with nothing to exempt is a list outliving its subject, and refusing
# it is what stops the list becoming the place repeats go to be forgotten.
UNEXPECTED="$(LC_ALL=C comm -23 <(printf '%s\n' "$DUPES") <(printf '%s\n' "$GRANDFATHERED"))"
STALE="$(LC_ALL=C comm -13 <(printf '%s\n' "$DUPES") <(printf '%s\n' "$GRANDFATHERED"))"

if [ -n "$UNEXPECTED" ]; then
	printf 'decision_key_check: repeated key (key, times): %s\n' "$UNEXPECTED" >&2
	fail "$DOC states the key(s) above more than once; every ADR reference in this repository is by number, so one key must denote one text (docs/decisions.md D-279, D-284)"
fi
if [ -n "$STALE" ]; then
	printf 'decision_key_check: exempted at a count it does not have: %s\n' "$STALE" >&2
	fail "the exemption above names a key/count pair $DOC does not have; an exemption that outlives its subject, or one granted for two copies where the file now holds another number, is a list that has stopped describing the file"
fi

echo "decision_key_check: $COUNT decision keys in $DOC, no repeat outside the exemption"
