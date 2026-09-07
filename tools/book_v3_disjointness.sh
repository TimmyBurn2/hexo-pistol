#!/usr/bin/env bash
#
# `book_v3` is disjoint, under canonical form, from every book a governed run
# has drawn from and from the labelled corpus — and is internally distinct.
#
# WHY THIS EXISTS AND WHY IT IS NOT A CI GATE. D-644 requires the disjointness;
# D-647 selects the shape that delivers it. The GENERATOR enforces the first two
# terms by construction, so a check that shared its code would prove nothing —
# this reads the finished books through `book_keys`, which shares no draw, no
# filter and no rendering with the builder. It does share `canonical_form`, the
# fold itself, and must: that is the identity D-644 is stated over.
# It is not in `tools/ci.sh` because the corpus term needs a 43 MB file that
# lives OUTSIDE the repository (D-636) and that no clone has: a gate depending
# on it would be green on one workstation and red everywhere else.
#
# BOTH TERMS ARE NAMED ON EVERY LINE (docs/decisions.md D-479). `v3 vs v1: 0 of
# 8500` says what was compared and how many were compared, because a bare `0` is
# a number whose denominator the reader has to guess.
#
# Usage: tools/book_v3_disjointness.sh --v1 <path> --v2 <path> --v3 <path> \
#                                      --corpus <manifest> [--keys-bin <path>]
#
# `--keys-bin` names an already-built `book_keys` instead of building one. It
# exists because this script's own test drives it from inside `cargo test`, and
# a nested `cargo build` there blocks on the target directory's lock.
# Exit:  0 every count is zero — the answer is yes
#        1 a count is not zero — AN ANSWER, and it is no
#        2 THE RUN IS VOID: no answer was taken (item 12)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() { printf 'book_v3_disjointness: FAIL: %s\n' "$*" >&2; exit 1; }
void() { printf 'book_v3_disjointness: RUN VOID: %s\n' "$*" >&2; exit 2; }

V1=""; V2=""; V3=""; CORPUS=""; KEYS_BIN=""
while [ "$#" -gt 0 ]; do
	[ "$#" -ge 2 ] || void "$1 wants a value"
	case "$1" in
	--v1) V1="$2" ;;
	--v2) V2="$2" ;;
	--v3) V3="$2" ;;
	--corpus) CORPUS="$2" ;;
	--keys-bin) KEYS_BIN="$2" ;;
	*) void "unknown flag $1" ;;
	esac
	shift 2
done
for pair in "--v1 $V1" "--v2 $V2" "--v3 $V3" "--corpus $CORPUS"; do
	set -- $pair
	[ -n "${2:-}" ] || void "$1 is required; this script takes no default path"
done
for f in "$V1" "$V2" "$V3" "$CORPUS"; do
	[ -r "$f" ] || void "cannot read $f"
done

# SCRATCH SPACE, ASKED FOR BEFORE THE WORK (tools/SHELL_CHECKLIST.md item 12
# obligation 2, docs/decisions.md D-285). BOTH filesystems, because they are
# two: the scratch files go under `$TMPDIR` and the build goes to this
# repository's target tree, and on this machine those are a RAM-backed tmpfs and
# an nvme partition. A shortage on either otherwise reaches the log in `mktemp`'s
# or `cargo`'s vocabulary, which describes those tools rather than this gate.
PREFLIGHT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/scratch_preflight.sh"
[ -x "$PREFLIGHT" ] || void "the scratch preflight is missing beside this script: $PREFLIGHT"
for SCRATCH_FS in "${TMPDIR:-/tmp}" "$ROOT"; do
	"$PREFLIGHT" "$SCRATCH_FS" ||
		void "no scratch room; the lines above name the filesystem"
done

SCRATCH="$(mktemp -d)" || void "mktemp could not make a scratch directory"
# Preserves the body's status rather than replacing it with `rm`'s (item 7).
trap 'rc=$?; rm -rf "$SCRATCH"; exit "$rc"' EXIT

if [ -z "$KEYS_BIN" ]; then
	command -v cargo >/dev/null || void "cargo is not on PATH, so the key tool cannot be built"
	# Built ONCE, up front: a build failure is an environmental refusal and must
	# not be discovered halfway through as a wrong count (item 12 obligation 2).
	cargo build --quiet --locked --manifest-path "$ROOT/Cargo.toml" \
		--package pistol-cli --example book_keys 2>"$SCRATCH/build.err" ||
		void "the key tool did not build: $(tr '\n' ' ' <"$SCRATCH/build.err")"
	KEYS_BIN="$ROOT/target/debug/examples/book_keys"
fi
[ -x "$KEYS_BIN" ] || void "the key tool $KEYS_BIN is not executable"

# `sort -u` folds nothing a book should hold twice; the INTERNAL count below is
# what says whether it held one, so the fold cannot hide it.
keys_of() { # $1 = a book
	"$KEYS_BIN" "$1" >"$SCRATCH/raw" 2>"$SCRATCH/err" ||
		void "the key tool refused $1: $(tr '\n' ' ' <"$SCRATCH/err")"
	LC_ALL=C sort -u <"$SCRATCH/raw"
}

# THE INPUTS, NAMED BY CONTENT BEFORE ANY COUNT (docs/decisions.md D-479). Three
# of the four are committed and digest-pinned in the tree; the CORPUS is not — it
# is a 43 MB file outside the repository with no in-tree pin — so `v3 vs corpus`
# would otherwise name one term and leave the other unidentified.
command -v sha256sum >/dev/null || void "sha256sum is not on PATH, so no input can be named"
for f in "$V1" "$V2" "$V3" "$CORPUS"; do
	digest="$(sha256sum -- "$f")" || void "cannot digest $f"
	printf 'book_v3_disjointness: input %s\n' "${digest%% *}  $f"
done

keys_of "$V1" >"$SCRATCH/v1"
keys_of "$V2" >"$SCRATCH/v2"
"$KEYS_BIN" "$V3" >"$SCRATCH/v3.raw" 2>"$SCRATCH/err" ||
	void "the key tool refused $V3: $(tr '\n' ' ' <"$SCRATCH/err")"
LC_ALL=C sort -u <"$SCRATCH/v3.raw" >"$SCRATCH/v3" || void "cannot sort the v3 keys"

# The corpus's own `key_full` column, restricted to the five-stone rows — the
# only ones a five-stone opening can equal. Column 5, tab-separated, per the
# manifest's own `# columns:` header.
/usr/bin/grep -v '^#' -- "$CORPUS" |
	awk -F'\t' '{ k = $5; if (k != "-" && split(k, a, " ") == 5) print k }' |
	LC_ALL=C sort -u >"$SCRATCH/corpus5" || void "the corpus manifest could not be read"

# THE CONTROL, and without it the corpus line is worthless. This script renders
# a key with `book_keys` and compares it against a column the ARENA wrote, so a
# rendering that drifted apart from `labels.rs`'s would make every corpus
# comparison find nothing and report `0 of N` — a pass produced by comparing two
# vocabularies rather than by disjointness (tools/SHELL_CHECKLIST.md item 10's
# "a control run so a pass cannot come from a gate that refuses everything").
# book_v2 is the control because the corpus was labelled FROM it: every
# five-stone corpus key must be one of its openings, so this intersection is the
# whole corpus slice or the two sides are not speaking the same language.
CORPUS_N="$(wc -l <"$SCRATCH/corpus5" | tr -d ' ')" || void "cannot count the corpus keys"
CONTROL="$(LC_ALL=C comm -12 "$SCRATCH/corpus5" "$SCRATCH/v2" | wc -l | tr -d ' ')" ||
	void "cannot intersect the corpus with book_v2"
[ "$CORPUS_N" -gt 0 ] || void "the corpus manifest yielded no five-stone key_full rows"
if [ "$CONTROL" -ne "$CORPUS_N" ]; then
	printf 'book_v3_disjointness: control: corpus5 ^ v2: %s of %s\n' "$CONTROL" "$CORPUS_N" >&2
	void "the control failed: every five-stone corpus key must be a book_v2 opening, and \
$((CORPUS_N - CONTROL)) are not. Either the corpus was not labelled from book_v2, or this \
script's key rendering has drifted from the arena's key_full — and in the second case a \
corpus comparison would report 0 for the wrong reason"
fi
printf 'book_v3_disjointness: control: corpus5 ^ v2: %s of %s (the renderings agree)\n' \
	"$CONTROL" "$CORPUS_N"

N="$(wc -l <"$SCRATCH/v3.raw" | tr -d ' ')" || void "cannot count the v3 openings"
case "$N" in
# A count the instrument could not produce is a VOID and not a finding about
# book_v3 (tools/SHELL_CHECKLIST.md item 12).
'' | *[!0-9]*) void "the opening count is not a number: \`$N\`" ;;
esac
[ "$N" -gt 0 ] || void "$V3 states no openings, so no disjointness question was asked"

overlap() { LC_ALL=C comm -12 "$SCRATCH/v3" "$1" | wc -l | tr -d ' '; }
A="$(overlap "$SCRATCH/v1")" || void "cannot intersect v3 with v1"
B="$(overlap "$SCRATCH/v2")" || void "cannot intersect v3 with v2"
C="$(overlap "$SCRATCH/corpus5")" || void "cannot intersect v3 with the corpus"
# Internal: how many openings are NOT their own distinct key.
DISTINCT="$(wc -l <"$SCRATCH/v3" | tr -d ' ')" || void "cannot count the distinct v3 keys"
D="$((N - DISTINCT))"

printf 'book_v3_disjointness: v3 vs v1: %s of %s\n' "$A" "$N"
printf 'book_v3_disjointness: v3 vs v2: %s of %s\n' "$B" "$N"
printf 'book_v3_disjointness: v3 vs corpus: %s of %s\n' "$C" "$N"
printf 'book_v3_disjointness: v3 internal: %s of %s\n' "$D" "$N"
printf 'book_v3_disjointness: corpus five-stone keys read: %s\n' "$CORPUS_N"

if [ "$A" -ne 0 ] || [ "$B" -ne 0 ] || [ "$C" -ne 0 ] || [ "$D" -ne 0 ]; then
	fail "book_v3 is not disjoint; every count above must be 0 of $N (D-644)"
fi
printf 'book_v3_disjointness: all four counts are 0 of %s\n' "$N"
