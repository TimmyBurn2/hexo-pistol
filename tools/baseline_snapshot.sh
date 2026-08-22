#!/usr/bin/env bash
#
# The registered baseline snapshot: a STANDING before/after instrument for
# Stage-1 work packages (docs/decisions.md D-230).
#
# WHAT THIS IS FOR. Every Stage-1 strength WP's report cites the snapshot delta
# beside its SPRT verdict, so a change that wins on strength while losing a
# third of its node rate cannot land without that being visible. IT LICENSES
# NOTHING BY ITSELF: it pre-registers no hypothesis, sets no threshold, and
# accepts or rejects no change. Rule 6's judge is SPRT over paired balanced
# openings; this is context beside that verdict and never a substitute for it.
#
# WHAT IT IS NOT, against docs/decisions.md D-14 and CLAUDE.md rule 5. D-14 and
# `pistol --help` both say `bench` is not implemented and that a benchmark ships
# with the first perf-sensitive change, its pre-registered hotspot and its abort
# threshold, not before. This is NOT that benchmark and does not implement
# `bench`. It is MEASUREMENT INFRASTRUCTURE in the same class as pistol-arena —
# a standing instrument with no hypothesis of its own, which per-change
# pre-registrations then cite. `tools/bench_delta.sh` remains the harness that
# carries a hotspot, a bracket and an abort threshold, and a perf-sensitive
# change still ships with its own pre-registered bracket against THAT. A
# snapshot delta is never a licence and never an abort.
#
# THE RECORD, and why it has two blocks. This is the arena report's idiom
# (crates/pistol-arena/src/report.rs, docs/decisions.md D-139, D-147, D-160,
# D-161): a kind token on the first line, then line-oriented whitespace-
# delimited records, split at the `# timing` marker.
#
#   - Everything ABOVE the marker is the INVARIANT BLOCK. Two runs at the same
#     revision, on the same machine, at the same budget produce it BYTE FOR
#     BYTE. Verified empirically: at a fixed node budget the engine's `nodes`,
#     `depth_turns`, `seldepth`, `hashfull`, `score` and `pv` are identical
#     across runs; only `nps` and `time` move, which is exactly what
#     tools/determinism.sh strips and what report.rs says.
#   - Everything BELOW it measures the machine.
#
#   The claim is stated exactly, as D-161 had to: A RUN THAT COMPLETES HAS AN
#   INVARIANT BLOCK. WHETHER A RUN COMPLETES IS NOT ITSELF INVARIANT, because a
#   ladder position is bounded by a wall-clock cap. A run in which any ladder
#   position failed to reach the requested depth carries the DIFFERENT KIND
#   TOKEN `baseline_snapshot_incomplete` — a token and not a flag, so that no
#   consumer can diff a partial record against a complete one (D-160).
#
#   The engine's own totals line mixes both classes on ONE line, so each
#   position emits TWO records: `position …` above the marker and
#   `timing position …` below it.
#
#   Key is the leading tokens, value is the rest of the line, and nothing is
#   quoted — the engine's own handshake carries multi-token values
#   (`id candidate_policy radius 2`, `id budgets depth_turns nodes`) and a rule
#   that refused a space would refuse the engine.
#
#   NOTHING ABOVE THE MARKER IS A FUNCTION OF THE RUN. No timestamp, no
#   hostname, no wall clock, and — the correction D-230 records — no WORKING
#   TREE STATE either: `dirty`/`clean` is a property of the checkout at the
#   moment the script ran, not of the revision, so it sits below the marker with
#   the other run-time facts. Stating it above would have made the invariance
#   claim false for the commonest case there is, a record written into the
#   repository it was taken in. FOUR path components appear above the marker,
#   enumerated here because the previous count of two was wrong and the omitted
#   pair is the caller-controlled one (docs/decisions.md D-232): `config <path>
#   <sha>` and the engine's own `engine_id config <path>` name the PINNED
#   instrument document and are constants of it, digested on the same line, which
#   is what D-147's no-path rule is about; `corpus <name> sha256 …` and
#   `openings <name> sha256 …` carry a BASENAME the caller chooses with
#   `--corpus`. A name is not a byte of the workload, so two byte-identical
#   corpora under two names give differing blocks — D-147's exact objection,
#   recorded as a NAMED RESIDUAL in D-232 rather than closed, because the record
#   format is pinned by a three-run byte-identical result two reviews verified.
#   What IS closed is the sharp end: a name containing a control character
#   injected attacker-chosen LINES into the invariant block, and a name
#   containing a SPACE shifted every field after it on its own line — the
#   digest a reader took from `corpus … sha256 …` was the string `sha256`.
#   Both are now refused at the guard below, and the second is why the rule
#   above says a VALUE may hold a space while a field BEFORE one may not.
#
# WHY `newgame` PRECEDES EVERY CORPUS POSITION. It is a determinism requirement,
# not a tidiness one: a table carried across positions lets one search's node
# count depend on another's (docs/decisions.md D-7), and the invariant block's
# per-position `nodes` is exactly what that would break. Do not remove it as
# redundant.
#
# THE LADDER CAP is a WALL CLOCK and is read as one. `timeout` passes the
# child's own exit status through unchanged, so 124 and 137 are equally what an
# engine exiting 124 itself and an OOM-killed engine produce; the status alone
# cannot tell those from the cap, and a snapshot that called an OOM kill "the
# 30 s cap fired" would be stating a terminal reason it never verified
# (docs/decisions.md D-232). Each rung is therefore timed, and 124/137 is the
# cap only if the run actually lasted the cap.
#
# THE BUDGET is 50 000 nodes — `tools/bench_delta.sh`'s own budget, so a
# snapshot number and a bench number are directly comparable. The rule that
# picked it (per-position setup under 5 % of per-position wall) is not assumed
# once and quoted forever: the script MEASURES the fraction on every run, from
# its own wall clock against the sum of the engine's reported times, and records
# it. Measured at 0f6c495 on a Ryzen 7 3700X: 1.66-1.71 % over three runs,
# whose invariant blocks were byte-identical (sha256 23bc24f1...).
#
# `tt_bytes` in the record is the config's REQUEST, which is what the engine
# advertises. The table actually allocated is
# previous_power_of_two(request / 96) * 96 — 192 MiB at a 256 MiB request — and
# no code change was made to expose it (that is a non-goal here).
#
# THE RECORD IS AN ARTIFACT (CLAUDE.md rule 8) and is never committed;
# tools/artifact_check.sh refuses it by CONTENT, under any filename.
#
# RULE9-JUSTIFICATION: one record over one instrument. The corpus pass, the
# ladder and the emit are three stages of producing a SINGLE document whose
# invariance claim is the whole point of the file; splitting them would put the
# claim in one file and the things that can falsify it in another, and every
# defect this script has had was a line that quietly stopped being invariant.
#
# Usage: tools/baseline_snapshot.sh [--out PATH] [--nodes N] [--corpus PATH]
#                                   [--ladder-depth D] [--ladder-cap-s S]
#                                   [--binary PATH]
#   --out           where to write the record (default: stdout)
#   --nodes         override the registered budget. A record made this way says
#                   OVERRIDE on its budget line, so it cannot be quoted as a
#                   baseline by accident.
#   --corpus        a different position fixture. WORKLOAD SCOPE: it shrinks the
#                   run without touching the registered budget, so the budget
#                   line still says `registered`. It must state at least one
#                   EARLY and one LATE position, because the ladder takes one
#                   rung from each band by name — see below.
#   --ladder-depth  the ladder's target depth (default 3). Workload scope, as
#                   above.
#   --ladder-cap-s  the ladder's wall-clock cap in seconds (default 30). It
#                   decides the KIND TOKEN, so it is not workload scope in the
#                   sense the two flags above are — but the record already states
#                   the cap it ran under on its own `ladder_cap_s` line, inside
#                   the invariant block, so a record taken under a different cap
#                   cannot be mistaken for a registered one and the line needs no
#                   second provenance token to say so (docs/decisions.md D-232).
#                   It exists because the CAP-FIRED path is otherwise untestable:
#                   the only honest evidence that the cap fired is that the run
#                   lasted it, and a test cannot afford to wait 30 s per rung.
#   --binary        the engine to measure (default target/release/pistol). The
#                   snapshot's PURPOSE is a release build; this exists so a test
#                   can drive the shipped script with the binary cargo just
#                   built, rather than testing a second implementation of the
#                   record (docs/decisions.md D-219's `unrescuable_beyond`).
#                   IT IS WORKLOAD-AFFECTING AND NOT BUDGET-AFFECTING, so the
#                   budget line still says `registered` and the only thing that
#                   separates a debug-build record from a release one is
#                   `binary_sha256` — which is why that line is there (D-230).
#                   The name is resolved the way the shell resolves it at exec
#                   time (`command -v`, then `realpath`) BEFORE it is digested:
#                   a bare name is PATH-resolved by the shell while `sha256sum`
#                   would read the cwd-relative file, and a digest that attests
#                   a file which never ran is worse than no digest at all.
# Exit:  0 the record was written, 1 a precondition or the run failed.
#        THERE IS NO VOID CLASS, stated rather than left to be inferred from
#        silence (tools/SHELL_CHECKLIST.md item 12 obligation 1). This is not a
#        gate and adjudicates nothing: it either wrote a record or it did not,
#        and every way of not writing one — a missing input, an engine that
#        would not answer, a refused name — is a 1 with a named reason on
#        stderr. A test that drives it therefore asserts the CODE and not merely
#        `!success`, because a 2 from here is bash's own or a signal's, and
#        would mean the script died somewhere it has no refusal for.

set -euo pipefail

# The record's whole claim is invariance, so the tools that build it are pinned
# to one locale rather than inheriting the operator's: `$EPOCHREALTIME` writes
# the LOCALE'S decimal separator, which a comma locale turns into an integer for
# every `awk` that reads it, and the cap comparison below is a wall-clock
# comparison that may not depend on who is logged in.
export LC_ALL=C

# The CALLER'S directory, captured BEFORE the `cd` below, because the `cd` is what
# makes a relative `--out` mean something the caller did not ask for. See the
# resolution after the argument loop.
CALLER_PWD="$PWD"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

SCHEMA=1
KIND_COMPLETE="baseline_snapshot"
KIND_INCOMPLETE="baseline_snapshot_incomplete"
TIMING_MARKER="# timing"

CONFIG="configs/instrument_v0.toml"
CORPUS="crates/pistol-cli/tests/fixtures/bench_positions_v1.txt"
OPENINGS="crates/pistol-cli/tests/fixtures/openings_v1.txt"
BINARY="target/release/pistol"
REGISTERED_NODES=50000
NODES="$REGISTERED_NODES"
BUDGET_PROVENANCE="registered"
LADDER_DEPTH=3
EARLY_MAX=17
# The cap that decides the kind token. Registered with its margin stated:
# the slowest registered ladder position measured 9.38 s to depth 3 on a
# Ryzen 7 3700X, so 30 s is a 3.2x margin and background load cannot decide
# which kind of record this is.
LADDER_CAP_S=30
# The corpus session and the handshake get loud bounds of their own, for the
# same reason the ladder has one: `[ -x ]` admits a FIFO, and a read that never
# returns is indistinguishable from a slow machine without one.
CORPUS_TIMEOUT=900
HANDSHAKE_TIMEOUT=60
OUT=""

fail() { printf 'baseline_snapshot: FAIL: %s\n' "$*" >&2; exit 1; }

# THE USAGE TEXT IS THE HOME OF THE PATH-RESOLUTION CLAIM (docs/decisions.md
# D-331). Before this text existed the rule was stated in a comment beside ONE
# flag's implementation and was true of that flag alone, so a caller had no
# statement to read and the script had no single place where the rule lived.
usage() {
	cat <<'USAGE'
usage: tools/baseline_snapshot.sh [--out PATH] [--corpus PATH] [--binary PATH|NAME]
                                  [--nodes N] [--ladder-depth N] [--ladder-cap-s N]
                                  [--help]

Takes the registered baseline snapshot (docs/decisions.md D-230) and writes the
record to --out, or to stdout when --out is not given.

HOW A RELATIVE PATH IS RESOLVED. THERE IS ONE BASE AND IT IS YOURS.

  Every path YOU pass on the command line -- --out, --corpus, --binary -- is
  resolved against THE DIRECTORY YOU RAN THIS SCRIPT FROM. Not against the
  repository root. This script cd's to the repository root before it does
  anything, and a cd may not redefine what your own words meant.

  This script's OWN defaults are repository paths and are resolved against the
  repository root, because they are the script's words and not yours. They are
  configs/instrument_v0.toml, the fixture corpus, the opening corpus and
  target/release/pistol.

  A --binary with no `/` in it is a bare NAME, not a path: it is resolved by
  PATH at exec time, and PATH is already yours.

  WHERE THE TWO READINGS WOULD DISAGREE, THIS SCRIPT REFUSES AND DOES NOT
  CHOOSE. If a relative --corpus or --binary names an existing file under the
  repository root and a DIFFERENT existing file under your directory, or names
  one only under the repository root, you get a named refusal telling you both
  paths. Pass an absolute path to settle it. Silently preferring either reading
  is how a record comes to attest a file the caller never named.

EXIT STATUS. This script declares no VOID class (tools/SHELL_CHECKLIST.md item
12): 0 is a record written, and every refusal is a FAIL at exit 1, named on
stderr. Nothing here distinguishes "the answer is no" from "I could not take the
answer", because this instrument has no answer to give -- it either wrote a
record or it refused to.
USAGE
}

# ONE BASE, APPLIED AT THE FLAG. tools/SHELL_CHECKLIST.md item 11: a caller's
# path consumed by a write is resolved against the root the caller meant, and a
# cd is not that root. MEASURED before any of this existed, `--out
# relative_probe.txt` issued from /tmp wrote its record into the REPOSITORY ROOT
# -- a file the caller never asked for, in a tree whose cleanliness other gates
# then adjudicate on. That was fixed for `--out` alone, which left the script
# resolving `--out` against the caller and `--corpus` and `--binary` against the
# root: two bases, one flag apart, and the same defect standing in the flags
# whose paths reach the record's digest lines.
#
# The refusals below are the "fail loud" rule (CLAUDE.md rule 3) applied to a
# base CHANGE rather than to a bad value: a caller who relied on the old
# root-relative reading of `--corpus` must be told, not silently redirected.
#
# It sets a global rather than printing, for the reason `argument` gives above:
# a `fail` inside a command substitution exits only the subshell.
#
# THE BARE-NAME EXEMPTION IS A PROPERTY OF THE FLAG, NOT OF THE VALUE. Only
# `--binary` is PATH-resolved (`command -v` below), so only `exec` may treat a
# value with no `/` as a name rather than a path. Written the other way round —
# exempting every slashless value — it silently exempted `--out record.txt`,
# which is a relative path and the one this whole rule was first written for.
caller_path() { # flag value read|write|exec -> sets ARG to the resolved path
	local flag="$1" value="$2" kind="$3" here there
	case "$value" in
	/*) ARG="$value"; return 0 ;;
	esac
	if [ "$kind" = exec ]; then
		case "$value" in
		*/*) ;;
		*) ARG="$value"; return 0 ;;
		esac
	fi
	here="$CALLER_PWD/$value"
	there="$ROOT/$value"
	if [ "$CALLER_PWD" != "$ROOT" ] && [ "$kind" != write ]; then
		if [ -e "$there" ] && [ ! -e "$here" ]; then
			fail "$flag \`$value\` is a relative path and this script resolves yours against the directory you ran it from, which is $CALLER_PWD, where nothing of that name exists. It DOES exist at $there. This script will not silently read the repository's copy when you named your own — pass an absolute path, or run from the directory your path is relative to (--help states the rule)"
		fi
		if [ -e "$there" ] && [ -e "$here" ] && [ ! "$here" -ef "$there" ]; then
			fail "$flag \`$value\` is AMBIGUOUS: it names one file at $here and a DIFFERENT file at $there. This script resolves your relative paths against the directory you ran it from, so the two readings disagree about which bytes this record would attest — pass an absolute path (--help states the rule)"
		fi
	fi
	ARG="$here"
	return 0
}

# A DIGEST THIS SCRIPT COULD NOT TAKE IS A REFUSAL, NEVER AN EMPTY FIELD.
# `echo "binary_sha256 $(sha256sum "$BINARY" | cut -d' ' -f1)"` discards the
# substitution's status — it is an ARGUMENT — so an engine that is executable and
# not readable (mode 0111) wrote `binary_sha256 ` with nothing after it, exited 0,
# and carried the COMPLETE kind token; and since `binary_sha256` is the ONLY line
# separating a debug-build record from a release one (see `--binary` above), two
# such records were BYTE-IDENTICAL in their whole invariant block. REPRODUCED.
# The value is taken into a variable BEFORE the block is written, because a
# `fail` inside a command substitution exits only the subshell — the same reason
# `argument` and `score_checked` set globals — and the 64 hex digits are checked
# so a `sha256sum` that answered something else cannot become a digest either.
DIGEST=""
digest() { # path what -> sets DIGEST
	local line
	line="$(sha256sum -- "$1" 2>/dev/null)" ||
		fail "cannot read $2 at $1 to digest it — a record that states no digest for it states nothing about which bytes ran"
	DIGEST="${line%% *}"
	case "$DIGEST" in
	*[!0-9a-f]* | "") fail "sha256sum answered \`$line\` for $2 at $1, which is not a digest" ;;
	esac
	[ "${#DIGEST}" -eq 64 ] ||
		fail "sha256sum answered a ${#DIGEST}-character digest for $2 at $1"
}

# An EMPTY value is not a value (CLAUDE.md rule 3). `--out ''` used to fall back
# to stdout in silence, which is the skip-with-default rule 3 forbids, in the one
# flag whose whole job is to say where the record goes.
#
# It sets a global rather than printing, for the reason `score_checked` below
# gives: a `fail` inside a command substitution exits only the subshell.
ARG=""
argument() { # flag remaining-count value -> sets ARG
	[ "$2" -ge 2 ] || fail "$1 needs a value"
	[ -n "$3" ] || fail "$1 was given an empty value, and an empty value is not one"
	ARG="$3"
}
while [ "$#" -gt 0 ]; do
	case "$1" in
	--out) argument --out "$#" "${2:-}"; caller_path --out "$ARG" write; OUT="$ARG"; shift 2 ;;
	--nodes)
		argument --nodes "$#" "${2:-}"; NODES="$ARG"; BUDGET_PROVENANCE="OVERRIDE"; shift 2 ;;
	--corpus) argument --corpus "$#" "${2:-}"; caller_path --corpus "$ARG" read; CORPUS="$ARG"; shift 2 ;;
	--ladder-depth) argument --ladder-depth "$#" "${2:-}"; LADDER_DEPTH="$ARG"; shift 2 ;;
	--ladder-cap-s) argument --ladder-cap-s "$#" "${2:-}"; LADDER_CAP_S="$ARG"; shift 2 ;;
	--binary) argument --binary "$#" "${2:-}"; caller_path --binary "$ARG" exec; BINARY="$ARG"; shift 2 ;;
	--help) usage; exit 0 ;;
	*) usage >&2; fail "unknown argument \`$1\`" ;;
	esac
done

# The COUNT'S SPELLING, not just its value. `[ 010 -ge 1 ]` is true because bash
# reads a leading zero as OCTAL, and the engine then parses the same token as
# decimal 10 while the record's invariant block quotes `010`; `+50000` and
# ` 50000` pass the same test and land above the marker un-normalized, the second
# putting a DOUBLE SPACE inside an invariant line. One spelling per number
# (docs/decisions.md D-232).
count() { # flag value -> a named refusal for anything but a bare positive count
	case "$2" in
	[1-9]) ;;
	[1-9]*) case "${2#?}" in *[!0-9]*) fail "$1 takes a positive integer with no sign, space or leading zero, got \`$2\`" ;; esac ;;
	*) fail "$1 takes a positive integer with no sign, space or leading zero, got \`$2\`" ;;
	esac
}
count --nodes "$NODES"
count --ladder-depth "$LADDER_DEPTH"
count --ladder-cap-s "$LADDER_CAP_S"
[ -f "$CONFIG" ] || fail "no config at $CONFIG"
[ -f "$CORPUS" ] || fail "no corpus at $CORPUS"
[ -f "$OPENINGS" ] || fail "no opening corpus at $OPENINGS"
# The two BASENAMES that reach the invariant block are caller-controlled, and a
# control character in one of them is not a cosmetic problem: a NEWLINE injects
# attacker-chosen LINES into the block, which forged a `ladder_terminal` and a
# `position` line with the record still exiting 0 and carrying the COMPLETE kind
# token (REPRODUCED, docs/decisions.md D-232). `${x##*/}` and not `basename`,
# because a command substitution strips the trailing newline the refusal is for.
# THE CLASS IS AN ALLOW-LIST AND NOT `[[:cntrl:]]`, because that class is as wide
# as the LOCALE says and this script pins `LC_ALL=C` above: under C it is ASCII,
# so LF was refused while U+2028 and U+0085 — both control characters by every
# Unicode reading, both bytes a file name may carry — walked straight through the
# guard into the block. Inverting it fixes the direction of the locale's effect:
# only printable ASCII is admitted, so the C pin now makes the refusal as WIDE as
# it can be, and a locale calling fewer characters printable can only refuse more.
# The named cost: a corpus whose file name is not printable ASCII is refused,
# which is what an ASCII record's provenance line can honestly carry. THAT COST
# STANDS UNCHANGED — the arm below is an ADDITION to that allow-list and narrows
# nothing it admitted for a reason.
#
# AND A SPACE IS THE ONE PRINTABLE CHARACTER THE ALLOW-LIST ADMITS BY
# CONSTRUCTION AND THE RECORD CANNOT CARRY. `[:print:]` INCLUDES the space, in C
# and in every other locale, so the guard above let it through by definition
# rather than by oversight — and the record is whitespace-token-delimited with
# nothing quoted, so a spaced basename does not corrupt one field, it SHIFTS
# every field after it: `corpus mini corpus.txt sha256 <hex> positions 2` gives a
# reader taking the digest from the line's fourth token the literal string
# `sha256`, with exit 0 and the COMPLETE kind token (REPRODUCED at 369d43a
# against the shipped script). The defect class is the header's own — a line that
# quietly stopped being what the block says it is — and it is exit-0-wrong-answer
# (tools/SHELL_CHECKLIST.md items 4 and 9).
#
# REFUSED AND NOT SUPPORTED, deliberately. Making a spaced name WORK means
# quoting or delimiting this line, which is a change to the record SCHEMA and to
# every reader of it, against a format the header pins to a three-run
# byte-identical result — and the record's leading-tokens rule is the same rule
# that lets the engine's own handshake carry multi-token VALUES, so a value may
# hold a space and a field before one may not. Refusing is one guard, costs the
# caller a `mv`, and keeps `corpus`/`openings` parsable by the rule the header
# states. The named cost, stated the way the line above states its own: a corpus
# whose file name contains a space cannot be snapshotted under that name. Only
# the BASENAME is guarded, because only the basename reaches the record — a
# corpus inside a spaced DIRECTORY still runs, and a test pins that so this
# refusal cannot quietly widen into one that refuses everything.
for named in "$CORPUS" "$OPENINGS"; do
	case "${named##*/}" in
	*[![:print:]]*) fail "the corpus path \`$named\` has a character outside printable ASCII in its file name, and its name is written into the record's invariant block" ;;
	*' '*) fail "the corpus path \`$named\` has a SPACE in its file name, and its name is written into a whitespace-delimited field of the record's invariant block, where it would shift every field after it" ;;
	esac
done
# The engine, resolved to the file that will ACTUALLY be exec'd before anything
# digests it. `-f` refuses the directory and the FIFO that `[ -x ]` admits.
BINARY_NAMED="$BINARY"
BINARY="$(command -v -- "$BINARY" 2>/dev/null || true)"
# ONE REFUSAL PER REASON. The single combined test told an operator who had
# pointed `--binary` at a directory to go and build the engine, which is a WRONG
# DIAGNOSIS where a named one belongs (CLAUDE.md rule 3). `command -v` declines a
# directory and an unfindable name identically, and ACCEPTS a FIFO, which then
# blocks every read this script makes — so each case says which it was.
if [ -z "$BINARY" ]; then
	[ ! -d "$BINARY_NAMED" ] ||
		fail "the engine named $BINARY_NAMED is a directory"
	[ ! -e "$BINARY_NAMED" ] ||
		fail "the engine named $BINARY_NAMED exists but is not an executable file"
	fail "no engine at $BINARY_NAMED (cargo build --release --locked -p pistol-cli --bin pistol)"
fi
[ -f "$BINARY" ] ||
	fail "the engine named $BINARY_NAMED is not a regular file — a FIFO passes \`[ -x ]\` and blocks every read this script makes"
[ -x "$BINARY" ] ||
	fail "the engine named $BINARY_NAMED is not executable"
BINARY="$(realpath -- "$BINARY")"

# Every digest the invariant block states, taken HERE — before the engine is
# launched and before a line of the record is written. An input this script
# cannot read is a refusal that costs nothing, and never a hole discovered by a
# reader of the record.
digest "$BINARY" "the engine"; BINARY_SHA256="$DIGEST"
digest "$CONFIG" "the config"; CONFIG_SHA256="$DIGEST"
digest "$CORPUS" "the corpus"; CORPUS_SHA256="$DIGEST"
digest "$OPENINGS" "the opening corpus"; OPENINGS_SHA256="$DIGEST"

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# `|| true` because a corpus with no entries makes both greps exit 1, which
# under `set -o pipefail` would abort the script with NO diagnostic at all where
# the named refusal below belongs (CLAUDE.md rule 3). Emptiness is a verdict
# this script states, not a status it dies of.
entries() { grep -v '^#' "$1" | grep . || true; }
tail_of() { printf '%s' "${1%% #*}"; }

# One `info totals` field. The protocol keys on names, not positions.
field() { # name < line
	awk -v want="$2" '{ for (i=1;i<NF;i++) if ($i==want) { print $(i+1); exit } }' <<<"$1"
}

# The `score` value, VALIDATED. `s/ pv.*$//` and not `s/ pv .*//`, because an
# empty pv leaves a bare ` pv` at the end of the line which the spaced form
# leaves sitting in the score field. The shape check is what F4's terminal
# reasons rest on, so it is a refusal and not a best effort.
#
# It sets a global instead of printing, because a `fail` inside a command
# substitution exits only the subshell — the defect D-226 records in the sibling
# script, not repeated here.
SCORE=""
score_checked() { # line -> sets SCORE
	local value
	SCORE="$(sed 's/.* score //; s/ pv.*$//' <<<"$1")"
	case "$SCORE" in
	'cp '*)
		value="${SCORE#cp }"; value="${value#-}"
		case "$value" in '' | *[!0-9]*) fail "unparseable score \`$SCORE\` in \`$1\` (the protocol emits cp/mate/-mate, crates/pistol-cli/src/report.rs)" ;; esac ;;
	'mate '* | '-mate '*)
		value="${SCORE##*mate }"
		# `mate 0` is "the side to move completes a line in zero turns", which is
		# not a position this engine is ever asked to search — and the ladder
		# reads a mate score as a TERMINAL REASON, so a distance of zero would
		# stop the ladder on a claim contradicting itself (D-232).
		case "$value" in
		'' | *[!0-9]*) fail "unparseable score \`$SCORE\` in \`$1\` (the protocol emits cp/mate/-mate, crates/pistol-cli/src/report.rs)" ;;
		esac
		[ "$value" -ge 1 ] || fail "the engine reported \`$SCORE\`, a mate distance of zero, which is not a score for a position it was asked to search: $1" ;;
	*) fail "unparseable score \`$SCORE\` in \`$1\` (the protocol emits cp/mate/-mate, crates/pistol-cli/src/report.rs)" ;;
	esac
}

# Every `info` field this script puts in a record or an expression is a COUNT,
# and none of them was checked to be one. `time` is the sharp case: it reaches
# bash ARITHMETIC at `SUM_S=$((SUM_S + tms))`, and `$(( ))` performs command
# substitution on its operand, so an engine answering `time PIPESTATUS[$(>/x)]`
# RAN that command with the record still written and exit 0 (REPRODUCED, D-232).
# The sibling `stones` field is guarded with this idiom three sections up.
counted() { # field value line
	case "$2" in
	'' | *[!0-9]*) fail "the engine reported \`$1\` as \`$2\`, which is not a count: $3" ;;
	esac
}

# ---- the corpus's shape, validated ONCE and by name (rule 3) ---------------
# The sibling tools/bench_delta.sh guards the identical `${entry##* stones }`
# expression, and this script did not: an entry with no stones count made the
# WHOLE LINE the band key and the record's `stones` value, with a raw `[:
# integer expected` on stderr and exit 0; and `[ 15$'\r' -le 17 ]` is ACCEPTED
# by bash, so a CRLF corpus put a bare CR mid-line in the invariant block, where
# two records differing only by a corpus's line endings look identical in any
# diff viewer (D-230).
COUNT="$(entries "$CORPUS" | wc -l)"
[ "$COUNT" -gt 0 ] || fail "the corpus states no positions: $CORPUS"
EARLY_COUNT=0
LATE_COUNT=0
while IFS= read -r entry; do
	case "$entry" in
	*$'\r'*) fail "the corpus has CRLF line endings, whose carriage return would land raw in the invariant block: $CORPUS" ;;
	esac
	stones="${entry##* stones }"
	case "$stones" in
	'' | *[!0-9]*) fail "entry without a stones count: $entry" ;;
	esac
	if [ "$stones" -le "$EARLY_MAX" ]; then EARLY_COUNT=$((EARLY_COUNT + 1)); else LATE_COUNT=$((LATE_COUNT + 1)); fi
done < <(entries "$CORPUS")
# The ladder takes one rung from each band BY BAND and not by a magic index, so
# a corpus with an empty band has no `late_mid` to name. Refused rather than
# silently mislabelled: the shipped record used to call a 15-stone position
# `late_mid` whenever the corpus was short.
[ "$EARLY_COUNT" -gt 0 ] || fail "the corpus states no EARLY position (stones <= $EARLY_MAX): $CORPUS"
[ "$LATE_COUNT" -gt 0 ] || fail "the corpus states no LATE position (stones > $EARLY_MAX): $CORPUS"
# `[ -f ]` says the openings fixture EXISTS, which is not the same as its having
# a position in it: an empty one made the ladder's opening rung `position` with
# no argument, so the engine's refusal was the first sign anything was wrong.
[ "$(entries "$OPENINGS" | wc -l)" -gt 0 ] || fail "the opening corpus states no positions: $OPENINGS"

INVARIANT="$WORK/invariant"
TIMING="$WORK/timing"
: >"$INVARIANT"; : >"$TIMING"
INCOMPLETE=0

# ---- engine identity, from the engine's own handshake (D-198) --------------
# The status is KEPT so the refusal can name the bound, the way the corpus and
# ladder refusals name theirs: a handshake that never returned and a handshake
# that answered nothing are different failures.
HANDSHAKE_RC=0
printf 'pistol\nquit\n' | timeout "$HANDSHAKE_TIMEOUT" "$BINARY" --config "$CONFIG" >"$WORK/hs" 2>/dev/null || HANDSHAKE_RC=$?
grep '^id ' "$WORK/hs" >"$WORK/id" ||
	fail "the engine at $BINARY answered the \`pistol\` handshake with no id lines (it exited $HANDSHAKE_RC; 124 means it did not answer within the ${HANDSHAKE_TIMEOUT}s bound)"

REVISION="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
# A property of THE RUN, not of the revision, so it belongs below the marker:
# see the header. A record written into the repository dirties the tree for the
# next run, so above the marker this token made the invariance claim false by
# the record's own existence (D-230).
if [ -n "$(git status --porcelain 2>/dev/null || true)" ]; then TREE=dirty; else TREE=clean; fi

{
	echo "schema $SCHEMA"
	echo "revision $REVISION"
	echo "binary_sha256 $BINARY_SHA256"
	# `$CONFIG` is a constant of this script and no flag sets it, so it is not
	# caller-named and the guard above does not cover it. If a `--config` flag is
	# ever added, this line joins that guard in the same commit: it has the same
	# shape as the two below and would shift its digest the same way.
	echo "config $CONFIG $CONFIG_SHA256"
	sed 's/^id /engine_id /' "$WORK/id"
	# `${x##*/}` and NOT `$(basename "$x")`, so the string written here is the
	# same expression the guard above checked — character for character. Two
	# spellings of "the basename" is how a guard comes to guard a value the record
	# does not write: the substitution strips the trailing newline the control-
	# character refusal exists for, and its status is an argument to `echo` and so
	# is discarded outright (tools/SHELL_CHECKLIST.md items 1 and 9).
	echo "corpus ${CORPUS##*/} sha256 $CORPUS_SHA256 positions $COUNT"
	echo "openings ${OPENINGS##*/} sha256 $OPENINGS_SHA256"
	echo "budget nodes $NODES $BUDGET_PROVENANCE"
	echo "ladder_depth $LADDER_DEPTH"
	echo "ladder_cap_s $LADDER_CAP_S"
} >>"$INVARIANT"

# ---- the corpus: one process, `newgame` before every position (D-7) --------
: >"$WORK/corpus.session"
while IFS= read -r entry; do
	printf 'newgame\nposition %s\ngo nodes %s\n' "$(tail_of "$entry")" "$NODES" >>"$WORK/corpus.session"
done < <(entries "$CORPUS")
echo quit >>"$WORK/corpus.session"
# The header says "Do not remove it as redundant". This is what makes that
# sentence binding rather than decorative: dropping the `newgame` carries the
# table across positions and the invariant block's per-position `nodes` and
# `hashfull` start depending on the position before (D-7, D-230).
[ "$(grep -c '^newgame$' "$WORK/corpus.session")" -eq "$COUNT" ] ||
	fail "the corpus session must carry one \`newgame\` per position ($COUNT), or one search's node count depends on another's (D-7)"

CORPUS_T0="$EPOCHREALTIME"
CORPUS_RC=0
timeout "$CORPUS_TIMEOUT" "$BINARY" --config "$CONFIG" <"$WORK/corpus.session" >"$WORK/corpus.out" || CORPUS_RC=$?
CORPUS_T1="$EPOCHREALTIME"
[ "$CORPUS_RC" -eq 0 ] ||
	fail "the engine exited $CORPUS_RC on the corpus session (124 means it exceeded ${CORPUS_TIMEOUT}s — a session that runs longer than that is a finding, not a wait)"
! grep -q '^error ' "$WORK/corpus.out" ||
	fail "the engine refused something on the corpus: $(grep -m1 '^error ' "$WORK/corpus.out")"

grep ' totals ' "$WORK/corpus.out" >"$WORK/corpus.totals" ||
	fail "no totals lines in the corpus transcript for $COUNT positions"
grep '^bestmove ' "$WORK/corpus.out" >"$WORK/corpus.best" || fail "no bestmove lines in the corpus transcript"
[ "$(wc -l <"$WORK/corpus.totals")" -eq "$COUNT" ] ||
	fail "$(wc -l <"$WORK/corpus.totals") totals lines for $COUNT positions"
# The same count check on the OTHER list, which had none: a short bestmove list
# left `... bestmove ` with an empty value at the end of an invariant line.
[ "$(wc -l <"$WORK/corpus.best")" -eq "$COUNT" ] ||
	fail "$(wc -l <"$WORK/corpus.best") bestmove lines for $COUNT positions"

SUM_S=0
i=0
while IFS= read -r entry; do
	i=$((i + 1))
	stones="${entry##* stones }"
	if [ "$stones" -le "$EARLY_MAX" ]; then band=early; else band=late; fi
	line="$(sed -n "${i}p" "$WORK/corpus.totals")"
	best="$(sed -n "${i}p" "$WORK/corpus.best" | cut -d' ' -f2-)"
	nodes="$(field "$line" nodes)"; tms="$(field "$line" time)"; nps="$(field "$line" nps)"
	dep="$(field "$line" depth_turns)"; sel="$(field "$line" seldepth)"; hf="$(field "$line" hashfull)"
	counted nodes "$nodes" "$line"; counted time "$tms" "$line"; counted nps "$nps" "$line"
	counted depth_turns "$dep" "$line"; counted seldepth "$sel" "$line"; counted hashfull "$hf" "$line"
	score_checked "$line"
	printf 'position %d %s %s nodes %s depth_turns %s seldepth %s hashfull %s score %s bestmove %s\n' \
		"$i" "$band" "$stones" "$nodes" "$dep" "$sel" "$hf" "$SCORE" "$best" >>"$INVARIANT"
	printf 'timing position %d time_ms %s nps %s\n' "$i" "$tms" "$nps" >>"$TIMING"
	SUM_S=$((SUM_S + tms))
done < <(entries "$CORPUS")

CORPUS_WALL="$(awk -v a="$CORPUS_T0" -v b="$CORPUS_T1" 'BEGIN{printf "%.0f", (b-a)*1000}')"
# The measured setup fraction has FOUR outcomes, not two. A wall clock that did
# not measure is `unmeasured` and never `0.00 ok`, which read as a PERFECT setup
# score for a run whose wall clock was unusable; and a NEGATIVE fraction — the
# engine reporting more time than the wall clock, so one of the two numbers is
# wrong — gets its own flag instead of passing as `ok` (D-230).
if [ "$CORPUS_WALL" -le 0 ]; then
	SETUP_PCT="unmeasured"; SETUP_FLAG="SETUP-UNMEASURED"
else
	SETUP_PCT="$(awk -v w="$CORPUS_WALL" -v s="$SUM_S" 'BEGIN{ printf "%.2f", 100*(w-s)/w }')"
	SETUP_FLAG="$(awk -v p="$SETUP_PCT" 'BEGIN{
		if (p + 0 < 0)        print "SETUP-NEGATIVE";
		else if (p + 0 > 5.0) print "SETUP-HEAVY";
		else                  print "ok" }')"
fi

# ---- the depth ladder ------------------------------------------------------
# One search per position: iterative deepening reports every completed depth, so
# `go depth_turns D` yields the whole d1..dD ladder with cumulative
# time-to-depth. Three separate searches would re-pay the shallow depths and
# measure the same thing worse.
ladder_one() { # name position
	# Declared in two statements on purpose: bash expands every argument of a
	# single `local` before it performs any of its assignments, so an `out`
	# that interpolated `$name` in the same statement would read it unset and
	# die under `set -u`.
	local name="$1" pos="$2"
	local out="$WORK/ladder.$name" rc=0 d last=0 last_score="" reason line
	local t0 t1 elapsed
	t0="$EPOCHREALTIME"
	printf 'newgame\nposition %s\ngo depth_turns %s\nquit\n' "$pos" "$LADDER_DEPTH" |
		timeout "$LADDER_CAP_S" "$BINARY" --config "$CONFIG" >"$out" 2>/dev/null || rc=$?
	t1="$EPOCHREALTIME"
	elapsed="$(awk -v a="$t0" -v b="$t1" 'BEGIN{ printf "%.2f", b - a }')"
	! grep -q '^error ' "$out" || fail "the engine refused the ladder position $name"
	# Every `info` line is flushed as it is written (Rust's Stdout is a
	# LineWriter), so a killed process still yields every depth that completed.
	while IFS= read -r line; do
		d="$(field "$line" depth_turns)"
		[ -n "$d" ] || continue
		# The guard the corpus loop uses, here for the reason F5 was found in the
		# first place: a non-numeric depth makes `[ "$last" -lt "$LADDER_DEPTH" ]`
		# below write `[: x: integer expected` to stderr, evaluate FALSE, and fall
		# through to `reason=complete` — exit 0, a COMPLETE kind token and a
		# terminal reason nothing verified (REPRODUCED, docs/decisions.md D-232).
		counted depth_turns "$d" "$line"
		# A ladder DEEPENS. An engine re-reporting a shallower depth would leave
		# `last` below depths the block already states, so the record would
		# contradict itself one line after stating them.
		[ "$d" -gt "$last" ] ||
			fail "the ladder position $name reported depth $d after depth $last — iterative deepening only goes deeper, and a terminal depth below one the record already states contradicts it"
		counted nodes "$(field "$line" nodes)" "$line"
		counted seldepth "$(field "$line" seldepth)" "$line"
		counted time "$(field "$line" time)" "$line"
		counted nps "$(field "$line" nps)" "$line"
		last="$d"
		score_checked "$line"; last_score="$SCORE"
		printf 'ladder %s depth %s nodes %s seldepth %s score %s\n' \
			"$name" "$d" "$(field "$line" nodes)" "$(field "$line" seldepth)" \
			"$SCORE" >>"$INVARIANT"
		printf 'timing ladder %s depth %s time_ms %s nps %s\n' \
			"$name" "$d" "$(field "$line" time)" "$(field "$line" nps)" >>"$TIMING"
	done < <(grep '^info depth_turns ' "$out" || true)
	# THE TERMINAL REASON IS READ, NEVER ASSERTED (D-230). Both branches were
	# guesses, and the record kept the COMPLETE kind token either way: a crashed
	# or refusing engine yielded a clean exit 0 and a record stating that the
	# wall-clock cap had fired, and a run that emitted nothing at all was
	# recorded as a mate at depth 0.
	if [ "$rc" -ne 0 ]; then
		# 124 and 137 are the statuses `timeout` reports when it fired and when
		# it had to escalate — AND ARE NOT ITS ALONE. `timeout` passes the
		# child's own status through unchanged, so an engine exiting 124 itself
		# and an OOM-killed engine (128+9) produce exactly the same two numbers
		# in MILLISECONDS. The status is therefore necessary and not sufficient:
		# the cap can only have fired if the run actually lasted the cap, and
		# that is a WALL-CLOCK fact this function now measures (REPRODUCED with
		# stubs exiting 124 and 137 at once against a 30 s cap, both recorded as
		# `reached 0 cap`; docs/decisions.md D-232).
		case "$rc" in
		124 | 137)
			awk -v e="$elapsed" -v c="$LADDER_CAP_S" 'BEGIN{ exit (e + 0 >= c + 0) ? 0 : 1 }' ||
				fail "the engine exited $rc on the ladder position $name after ${elapsed}s of a ${LADDER_CAP_S}s cap — \`timeout\` passes the child's own status through, so 124 and 137 from a run that did not reach the cap are the ENGINE failing (137 is what an OOM kill yields) and not the cap firing"
			reason=cap; INCOMPLETE=1 ;;
		*) fail "the engine exited $rc on the ladder position $name — only 124/137 after the full ${LADDER_CAP_S}s is the cap firing, and no other reason is one this record may state" ;;
		esac
	elif [ "$last" -lt "$LADDER_DEPTH" ]; then
		# A proven mate stops the deepening loop. That is a real answer, not a
		# truncated one, so it does NOT make the record incomplete — but it is
		# the ENGINE'S OWN SCORE that says so, read from the line the loop above
		# already parsed.
		case "$last_score" in
		'mate '* | '-mate '*) reason=mate ;;
		'') fail "the ladder position $name completed no depth at all, so this record has no terminal reason it can state" ;;
		*) fail "the ladder position $name stopped at depth $last of $LADDER_DEPTH scoring \`$last_score\` — neither the cap nor a mate, so this record has no terminal reason it can state" ;;
		esac
	else
		reason=complete
	fi
	printf 'ladder_terminal %s reached %s %s\n' "$name" "$last" "$reason" >>"$INVARIANT"
	# depth-at-500ms is DERIVED from this same run: instrument mode refuses a
	# `movetime` budget by name, so a wall-clock budget is not available and
	# this is the honest substitute. `ceiled` means the ladder itself ran out.
	awk -v name="$name" -v target="$LADDER_DEPTH" '
		/^info depth_turns /{ for(i=1;i<NF;i++){ if($i=="depth_turns") d=$(i+1); if($i=="time") t=$(i+1) }
			if (t <= 500) best=d }
		END { printf "timing depth_at_500ms %s %s%s\n", name, (best?best:0), (best==target ? " ceiled" : "") }' \
		"$out" >>"$TIMING"
}

# One rung per BAND, chosen by the band the entry states rather than by a magic
# index. `sed -n 13p` was never checked against the band, so on a short corpus
# the record called an EARLY position `late_mid` three lines below labelling the
# same entry `early` (D-230). On the registered 24-position corpus the first
# late entry IS entry 13, so the registered record is unchanged by this.
band_first() { # early|late < entries -> the first entry in that band
	awk -v max="$EARLY_MAX" -v want="$1" '{
		s = $0; sub(/.* stones /, "", s);
		if (((s + 0 <= max) ? "early" : "late") == want) { print; exit }
	}'
}
LADDER_OPENING="$(tail_of "$(entries "$OPENINGS" | sed -n 1p)")"
LADDER_EARLY="$(tail_of "$(entries "$CORPUS" | band_first early)")"
LADDER_LATE="$(tail_of "$(entries "$CORPUS" | band_first late)")"
ladder_one opening "$LADDER_OPENING"
ladder_one early_mid "$LADDER_EARLY"
ladder_one late_mid "$LADDER_LATE"

# ---- emit ------------------------------------------------------------------
KIND="$KIND_COMPLETE"
[ "$INCOMPLETE" -eq 0 ] || KIND="$KIND_INCOMPLETE"
{
	echo "$KIND $SCHEMA"
	cat "$INVARIANT"
	echo "$TIMING_MARKER — machine-, schedule- and worktree-dependent; excluded from every comparison"
	echo "timing host $(uname -m) cores $(nproc)"
	echo "timing tree $TREE"
	echo "timing corpus_wall_ms $CORPUS_WALL engine_sum_ms $SUM_S"
	echo "timing setup_fraction_pct $SETUP_PCT $SETUP_FLAG"
	cat "$TIMING"
} >"$WORK/record"

if [ -n "$OUT" ]; then
	cp "$WORK/record" "$OUT" || fail "cannot write the record to $OUT"
	echo "baseline_snapshot: $KIND schema $SCHEMA, $COUNT positions at nodes $NODES ($BUDGET_PROVENANCE), setup $SETUP_PCT% $SETUP_FLAG -> $OUT" >&2
else
	cat "$WORK/record"
fi
