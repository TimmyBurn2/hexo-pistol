#!/usr/bin/env bash
#
# The judge itself, end to end, on every CI run. Hard Rule 6 makes SPRT over
# paired openings the judge of every search and eval change, so an arena that
# has quietly stopped working would not fail anything else in this suite — every
# other gate is about the engine.
#
# It is a SELF-MATCH, and that is what makes it assertable rather than merely
# plausible. Two identical deterministic engines play the same game whichever
# seat they sit in, so the expected answer is known in advance:
#
#   * every opening is played from both seats            -> n = 2 x openings
#   * both games of a pair are the same game move for move -> distinct_n = n/2
#   * every pair therefore scores 1-1                    -> pentanomial p2 = pairs
#   * no pair-to-pair variation exists                   -> no LLR is defined
#   * so the verdict is `inconclusive_degenerate`, which is the CORRECT answer
#     for two identical configurations and not a failure
#
# A gate whose expected output is "something plausible" is not a gate, which is
# why every one of those is asserted exactly.
#
# The second thing this gate does is the arena's OWN determinism, which nothing
# else covers: the run is repeated and the two verdict blocks are compared byte
# for byte. That is `tools/determinism.sh`'s shape applied to the instrument
# every strength claim comes from. The comparison is on the verdict block and
# not the whole file because the timing block is machine-dependent BY DESIGN and
# says so on its own marker line (docs/decisions.md D-161).
#
# Cost, pre-registered (CLAUDE.md rule 5): three arena runs at depth_turns 1 and
# candidate radius 1 over 4 openings, which is 24 games and about 480 engine
# searches. Expected well under a minute on the development machine; the cost
# grows with `turn_cap` because the candidate set grows with the stone count, so
# a change to that key in the config is a change to this gate's runtime.
#
# Usage: tools/arena_smoke.sh
# Exit:  0 the gate holds
#        1 it does not — AN ANSWER, and it is no
#        2 THE RUN IS VOID: no answer was taken, the environment having refused
#
# THE THIRD CODE IS tools/SHELL_CHECKLIST.md ITEM 12, and this gate needs it
# because it BUILDS: a full scratch filesystem makes cargo answer in its own
# vocabulary, which reads downstream as a smoke-gate regression.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CONFIG="configs/arena_smoke_v0.toml"

fail() { printf 'arena_smoke: FAIL: %s\n' "$*" >&2; exit 1; }
# THE VOID, NAMED. Not `fail`: no answer about the arena was taken.
void() { printf 'arena_smoke: RUN VOID: %s\n' "$*" >&2; exit 2; }

command -v cargo >/dev/null || fail "cargo is not on PATH"
command -v sha256sum >/dev/null || fail "sha256sum is not on PATH"
[ -f "$CONFIG" ] || fail "no arena config at $CONFIG"

# Never under the repository: match logs are artifacts and artifacts are not
# committed (CLAUDE.md rule 8).
WORK="$(mktemp -d)" || void "mktemp could not make a scratch directory for the match"
# The trap preserves the body's status rather than replacing it with `rm`'s
# (item 7): a cleanup that fails must not turn a clean run into a refusal.
trap 'rc=$?; rm -rf "$WORK"; exit "$rc"' EXIT

# SCRATCH SPACE, BEFORE THE BUILD AND IN THIS GATE'S VOCABULARY (item 12
# obligation 2, docs/decisions.md D-285). Both filesystems, because they are
# two: the match log and its clone go under $WORK, and the release build goes to
# the repository's target tree. A shortage on either otherwise reaches the log
# as `cargo` failing, which reads as this gate refusing.
PREFLIGHT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/scratch_preflight.sh"
[ -f "$PREFLIGHT" ] || fail "the scratch preflight is missing beside this gate: $PREFLIGHT"
for SCRATCH in "$WORK" "$ROOT"; do
	PF_RC=0
	bash "$PREFLIGHT" "$SCRATCH" || PF_RC=$?
	case "$PF_RC" in
	0) ;;
	2) void "not enough scratch space under $SCRATCH to build and play this match; the lines above name the filesystem, and NOTHING about the arena was measured" ;;
	*) fail "the scratch preflight refused its own arguments (exit $PF_RC)" ;;
	esac
done

echo "arena_smoke: building the engine and the arena (release, locked)"
# THE BINARY THIS GATE RUNS IS THE BINARY CARGO BUILT: the path comes from cargo's
# artifact stream, never from a literal here. `CARGO_TARGET_DIR`, `[build]
# target-dir` and `[build] target` each move the artifact, and a hardcoded
# `target/release/pistol` then runs whatever STALE binary sits at that path while
# the build goes elsewhere — a gate that passes for a binary nobody built
# (REPRODUCED on tools/tactical_check.sh; docs/decisions.md D-250).
#
# RESOLVING THE PATH IS HALF THE JOB HERE: the arena takes its engines from the
# config's `binary = ` lines, so for one revision both seats played the literal
# while `$ENGINE` sat validated and unread (docs/decisions.md D-252).
BUILD_LOG="$(cargo build --release --locked --quiet --bin pistol --bin arena \
	--message-format=json-render-diagnostics)" || fail "the engine and the arena do not build"
# Two bins, so each executable is taken by the name cargo gave the FILE rather
# than by the order the artifact stream happens to emit them in.
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
ENGINE=""
ARENA=""
# Two bins means selecting by NAME, and a `case` that selects by name is
# last-one-wins: two artifacts sharing a file name resolve silently to whichever
# the stream emitted second. The three sibling gates build one bin and spell that
# refusal `-eq 1`; selecting by name, this one counts for itself. REPRODUCED:
# two workspace members each declaring `[[bin]] name = "pistol"` make cargo emit
# two `executable` records for `--bin pistol` and exit 0.
ENGINE_N=0
ARENA_N=0
for path in ${BUILT[@]+"${BUILT[@]}"}; do
	case "${path##*/}" in
	pistol)
		ENGINE="$path"
		ENGINE_N=$((ENGINE_N + 1))
		;;
	arena)
		ARENA="$path"
		ARENA_N=$((ARENA_N + 1))
		;;
	esac
done
# ONE REFUSAL PER REASON (tools/SHELL_CHECKLIST.md item 8): a bin cargo named no
# executable for, SEVERAL executables sharing one file name, and then a named
# path that is absent, is not a regular file, or carries no `+x` — the last being
# the case `command -v` admits and exec answers with 126. A helper that must
# refuse is called as a statement, never inside a command substitution, where
# `fail` would exit only the subshell (item 1).
usable() { # $1 = the bin name, $2 = the path cargo named, $3 = how many it named
	[ -n "$2" ] || fail "cargo built no executable for --bin $1"
	[ "$3" -eq 1 ] ||
		fail "cargo named $3 executables whose file name is \`$1\`, so the file name does not choose one: ${BUILT[*]}"
	[ -e "$2" ] || fail "cargo named \`$2\` for --bin $1 and nothing is there"
	[ -f "$2" ] || fail "cargo named \`$2\` for --bin $1 and it is not a regular file"
	[ -x "$2" ] || fail "cargo named \`$2\` for --bin $1 and it is not executable"
}
usable pistol "$ENGINE" "$ENGINE_N"
usable arena "$ARENA" "$ARENA_N"

# What the config says, read from the config rather than restated here: a gate
# that hard-coded the numbers would pass after somebody changed the document.
TAKE="$(sed -n 's/^openings_take = \([0-9]*\).*/\1/p' "$CONFIG" | head -1)"
WORKERS="$(sed -n 's/^n_workers = \([0-9]*\).*/\1/p' "$CONFIG" | head -1)"
[ -n "$TAKE" ] || fail "$CONFIG states no openings_take"
GAMES=$((TAKE * 2))
echo "arena_smoke: $TAKE openings, $GAMES games, $WORKERS workers, config $CONFIG"

# --- the engines that PLAY are bound to the binary cargo built -----------------
#
# The arena has no `--binary` flag — `--config` and `--out` are its only
# arguments — so the DOCUMENT is the seam, exactly as the one-worker run below
# already rewrites `n_workers`. Without this the config's literal
# `binary = "target/release/pistol"` is what every seat plays, whatever cargo
# built and wherever it built it (REPRODUCED at 0d7682d: with `CARGO_TARGET_DIR`
# redirected, 54 of 54 engine invocations went to a decoy sitting at that
# literal path and this gate exited 0; docs/decisions.md D-252).

# WHAT REACHES A RECORD IS CALLER-CONTROLLED (tools/SHELL_CHECKLIST.md item 9):
# `$ENGINE` is interpolated into a TOML document the arena parses, so a newline
# in it would INJECT LINES. The guard is an ALLOW-LIST (item 4) and the locale is
# deliberately NOT pinned: at the ambient locale `[[:print:]]` refuses LF, TAB,
# U+0085 and U+2028 — every character that could inject — while `LC_ALL=C` would
# also refuse a legal `/home/tomás/…`, a FALSE refusal and the wrong direction
# for a pin to move a guard (all six measured). A quote or a backslash cannot
# reach here: the record cross-check above refuses those by name.
case "$ENGINE" in
*[![:print:]]*)
	fail "cargo named an engine path carrying a non-printable character, which this gate will not write into a config: \`$ENGINE\`"
	;;
esac

# How many seats the document has, counted FROM THE DOCUMENT. `grep -c` prints 0
# and STILL exits 1 on no match (item 3), so the count is a value to refuse and
# never a status to test, and its SPELLING is checked too (item 8).
STANZAS="$(grep -c '^binary = ' "$CONFIG" || true)"
case "$STANZAS" in
*[!0-9]* | "") fail "the engine-stanza count is not a number: \`$STANZAS\`" ;;
esac
[ "$STANZAS" -eq 2 ] ||
	fail "$CONFIG names $STANZAS engine binaries and a match is played by exactly two"

# THE SECOND HALF OF THE SEAT. The arena now binds each engine BY CONTENT and
# refuses a `binary` whose digest is not the one the document names
# (docs/decisions.md D-283), so rewriting the path alone leaves this gate
# refusing every run: the committed digest belongs to the build the document was
# written at, and this gate builds its own. Both keys are rewritten together and
# both rewrites are counted, because a rewrite that landed on one of them is a
# gate that either refuses everything or attests a digest nobody checked.
DIGEST_STANZAS="$(grep -c '^binary_sha256 = ' "$CONFIG" || true)"
case "$DIGEST_STANZAS" in
*[!0-9]* | "") fail "the digest-stanza count is not a number: \`$DIGEST_STANZAS\`" ;;
esac
[ "$DIGEST_STANZAS" -eq "$STANZAS" ] ||
	fail "$CONFIG names $STANZAS engine binaries and $DIGEST_STANZAS digests; every seat is bound by content or none is"

# A COMMAND SUBSTITUTION WHOSE STATUS IS DISCARDED CANNOT FAIL
# (tools/SHELL_CHECKLIST.md item 1): the value is taken into a variable, and
# then its SHAPE is checked (item 8) rather than its emptiness alone — a
# truncated digest is 63 characters and would be written into the document as
# happily as a whole one.
ENGINE_SHA="$(sha256sum -- "$ENGINE" | cut -d' ' -f1)" ||
	fail "cannot digest the engine cargo built at \`$ENGINE\`"
case "$ENGINE_SHA" in
*[!0-9a-f]* | "") fail "the engine digest is not lowercase hex: \`$ENGINE_SHA\`" ;;
esac
[ "${#ENGINE_SHA}" -eq 64 ] ||
	fail "the engine digest is ${#ENGINE_SHA} characters, not 64: \`$ENGINE_SHA\`"

# `awk` rather than `sed`: the replacement is built by CONCATENATION, so an `&`
# in the path stays an `&` rather than becoming the whole match, and no delimiter
# can collide with a `/` or a `|` in it. The value arrives through `ENVIRON`, not
# `-v`, which processes escape sequences in what it is given.
BOUND="$WORK/bound.toml"
ENGINE="$ENGINE" ENGINE_SHA="$ENGINE_SHA" awk '
	/^binary = / { print "binary = \"" ENVIRON["ENGINE"] "\""; next }
	/^binary_sha256 = / { print "binary_sha256 = \"" ENVIRON["ENGINE_SHA"] "\""; next }
	{ print }
' "$CONFIG" >"$BOUND" || fail "$CONFIG could not be rewritten to bind the engines"

# THE REWRITE MATCHED. An `awk` that matched nothing exits 0 and writes a copy
# with the literal still in it, which is this gate's own defect in a new
# spelling. Counted with the SAME enumeration as the source (item 5) and matched
# as a WHOLE LINE with `-F`, so a `.` in the path cannot let the pattern accept a
# near miss (item 3).
REBOUND="$(grep -c -F -x -- "binary = \"$ENGINE\"" "$BOUND" || true)"
case "$REBOUND" in
*[!0-9]* | "") fail "the bound-line count is not a number: \`$REBOUND\`" ;;
esac
[ "$REBOUND" -eq "$STANZAS" ] ||
	fail "the rewrite bound $REBOUND of $STANZAS engine binaries to \`$ENGINE\`; $CONFIG does not spell them \`binary = \`"
REDIGESTED="$(grep -c -F -x -- "binary_sha256 = \"$ENGINE_SHA\"" "$BOUND" || true)"
case "$REDIGESTED" in
*[!0-9]* | "") fail "the bound-digest count is not a number: \`$REDIGESTED\`" ;;
esac
[ "$REDIGESTED" -eq "$DIGEST_STANZAS" ] ||
	fail "the rewrite bound $REDIGESTED of $DIGEST_STANZAS engine digests to \`$ENGINE_SHA\`; $CONFIG does not spell them \`binary_sha256 = \`"
echo "arena_smoke: both seats bound to $ENGINE ($ENGINE_SHA)"

run_arena() {
	local out="$1" config="$2"
	"$ARENA" --config "$config" --out "$out" >"$out.stdout" 2>"$out.stderr" ||
		fail "the arena exited nonzero:
$(tail -20 "$out.stderr")"
	[ -s "$out" ] || fail "no report was written to $out"
}

field() { sed -n "s/^$2 //p" "$1" | head -1; }

echo "arena_smoke: run 1"
run_arena "$WORK/a.txt" "$BOUND"

# --- the self-match's knowable answer -----------------------------------------

KIND="$(head -1 "$WORK/a.txt" | awk '{print $1}')"
[ "$KIND" = "arena_report" ] ||
	fail "the run was abandoned: the report is an \`$KIND\`, not a verdict-carrying one"

# WHICH BINARY ACTUALLY PLAYED, read off the RUN'S OWN RECORD and not off the
# document this gate wrote — the arena records a seat's binary and digests the
# file it ran (that digest was checked here against `sha256sum` of the file
# actually executed). ` binary … binary_sha256 ` is delimited on both sides, so
# this matches the FIELD and not a substring of another path (item 3). The lines
# sit in the verdict block, so the comparisons below carry this to runs 2 and 3.
PLAYED_BY_BUILT="$(grep -c -F -- " binary $ENGINE binary_sha256 $ENGINE_SHA " "$WORK/a.txt" || true)"
case "$PLAYED_BY_BUILT" in
*[!0-9]* | "") fail "the bound-seat count is not a number: \`$PLAYED_BY_BUILT\`" ;;
esac
[ "$PLAYED_BY_BUILT" -eq "$STANZAS" ] ||
	fail "$PLAYED_BY_BUILT of $STANZAS seats played \`$ENGINE\`, the binary cargo built; the rest played something else:
$(grep '^engine ' "$WORK/a.txt")"

PLAYED="$(grep -c '^game ' "$WORK/a.txt" || true)"
[ "$PLAYED" -eq "$GAMES" ] ||
	fail "played $PLAYED games, expected $GAMES (every opening from both seats)"

COUNTS="$(field "$WORK/a.txt" counts)"
case "$COUNTS" in
*"n $GAMES distinct_n $TAKE"*) ;;
*) fail "expected n $GAMES and distinct_n $TAKE, got: $COUNTS
Two identical deterministic engines must produce identical games; anything else
means the dedupe, the seating or the determinism law has moved." ;;
esac
case "$COUNTS" in
*"forfeits 0"*) ;;
*) fail "a self-match forfeited a game, which means the engine broke its own protocol:
$COUNTS
$(grep '^refusal ' "$WORK/a.txt" | head -5)" ;;
esac

PENTANOMIAL="$(field "$WORK/a.txt" pentanomial)"
case "$PENTANOMIAL" in
*"p2 $TAKE"*) ;;
*) fail "expected every one of the $TAKE pairs to score 1-1, got: $PENTANOMIAL" ;;
esac

VERDICT="$(field "$WORK/a.txt" verdict)"
[ "$VERDICT" = "inconclusive_degenerate" ] ||
	fail "expected \`inconclusive_degenerate\` — two identical configurations give a
sample with no variance and therefore no likelihood ratio — got \`$VERDICT\`"
[ "$(field "$WORK/a.txt" verdict_unit)" = "pair" ] ||
	fail "the verdict is read off the PAIR unit (docs/decisions.md D-154)"

# Per-side compute was recorded, not merely printed. A driver that billed
# nothing would still emit the fields.
while read -r line; do
	for key in nodes_a nodes_b depth_a depth_b; do
		value="$(echo "$line" | sed -n "s/.* $key \([0-9]*\).*/\1/p")"
		[ -n "$value" ] && [ "$value" -gt 0 ] ||
			fail "$key is zero or missing, so per-side compute was not recorded: $line"
	done
done < <(grep '^game ' "$WORK/a.txt")

echo "arena_smoke: $GAMES games, distinct-n $TAKE, verdict $VERDICT, compute recorded"

# --- the arena's own determinism ----------------------------------------------

echo "arena_smoke: run 2 (same config, compared to run 1)"
run_arena "$WORK/b.txt" "$BOUND"

# Everything before the timing marker is the verdict block, which is
# worker-invariant and machine-invariant by design.
verdict_block() { sed '/^# timing/,$d' "$1"; }
verdict_block "$WORK/a.txt" >"$WORK/a.block"
verdict_block "$WORK/b.txt" >"$WORK/b.block"
[ -s "$WORK/a.block" ] || fail "the verdict block is empty, so the diff below proves nothing"
diff -u "$WORK/a.block" "$WORK/b.block" >"$WORK/diff.repeat" ||
	fail "two runs of the same config disagreed:
$(head -40 "$WORK/diff.repeat")"

# --- and its independence from the worker count -------------------------------

echo "arena_smoke: run 3 (one worker, compared to $WORKERS)"
# From the BOUND document and not from $CONFIG, or the one-worker run would go
# back to playing the literal and the comparison would be between two different
# pairs of engines.
sed "s/^n_workers = .*/n_workers = 1/" "$BOUND" >"$WORK/single.toml"
run_arena "$WORK/c.txt" "$WORK/single.toml"
verdict_block "$WORK/c.txt" >"$WORK/c.block"
diff -u "$WORK/a.block" "$WORK/c.block" >"$WORK/diff.workers" ||
	fail "one worker and $WORKERS workers disagreed on the verdict block, which is the one
thing scheduling may never touch (CLAUDE.md rule 4, docs/decisions.md D-161):
$(head -40 "$WORK/diff.workers")"

grep -q '^timing n_workers 1 ' "$WORK/c.txt" ||
	fail "the one-worker run does not record its worker count, so the comparison above
could have passed against a report that simply omits it"
grep -q "^timing n_workers $WORKERS " "$WORK/a.txt" ||
	fail "the $WORKERS-worker run does not record its worker count either"

printf 'arena_smoke: ok — %d games over %d openings, distinct-n %d, verdict %s,\n' \
	"$GAMES" "$TAKE" "$TAKE" "$VERDICT"
printf 'arena_smoke:      three runs agree on the verdict block at 1 and %d workers\n' "$WORKERS"
