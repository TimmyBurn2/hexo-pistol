#!/usr/bin/env python3
#
# The WP-2.1 sweep's assembly step: the passing tranches' corpora into the two
# manifests docs/experiments/wp21_prereg.md section 6 registers.
#
# WHAT IT WRITES. Two files, both MEASURED over the corpora it is given:
#   raw_manifest.txt      one row per corpus — its index, path, own body digest
#                         and record count. Raw is the record.
#   deduped_manifest.txt  one row per DISTINCT position under docs/decisions.md
#                         D-562(2)'s default — the record that represents it,
#                         named by corpus index and record index. Deduped is the
#                         training input, and it is an INDEX into the raw corpora
#                         rather than a copy of them: a merged corpus would need
#                         one experiment digest and sixteen tranches have sixteen.
#
# THE RULE, from D-562(2) and applied here in one place: two records are the
# SAME position exactly when all three keys agree — key_seq, key_pos and
# key_full. Of a set of records for one position the DEEPER label wins
# (`depth_turns`), and a tie goes to the FIRST in input order, corpora in the
# order given and records in file order. A record agreeing with an earlier one on
# some keys and not all is a DISAGREEMENT: it is kept as a distinct position and
# COUNTED, because which key rules such a pair is the question D-562(2) leaves
# open and nothing here may settle it by construction.
#
# OUTCOME COVERAGE is the fraction of deduped records whose game was DECIDED by
# the rules: `result` is a win and `end` is `normal`. A capped game is a horizon
# and a forfeit is the driver's, and D-562(1) lets outcome enter only on the
# decided subset.
#
# IT NEVER OVERWRITES. Both outputs are created exclusively; an existing file is
# a named refusal before anything is read (docs/decisions.md D-200's shape).
#
# Usage:
#   tools/wp21_assemble.py --out-dir <dir> --corpus <path> [--corpus <path>]...
#
# Exit:  0 both manifests written; each one's own body sha256 is printed
#        1 a named refusal — an output that already exists
#        2 THE RUN IS VOID: an input is not a corpus this instrument can read
#          (tools/SHELL_CHECKLIST.md item 12). A void is not an answer.

import argparse
import hashlib
import os
import sys
from pathlib import Path

VOID = 2
REFUSED = 1

FIELDS = 16
KEY_SEQ, KEY_POS, KEY_FULL = 3, 4, 5
DEPTH = 10
RESULT, END = 14, 15
WINS = ("p1_win", "p2_win")
BODY_MARKER = "# body_sha256 "
CAPTURE_MARKER = "# derived capture_sha256 "

RAW_NAME = "raw_manifest.txt"
DEDUPED_NAME = "deduped_manifest.txt"


class Void(Exception):
    """No answer could be taken."""


def say(what):
    print(f"wp21_assemble: {what}")


def readable(word, what):
    if any(ord(c) < 0x20 or ord(c) == 0x7F for c in word):
        raise Void(f"the {what} path carries a control character: {word!r}")
    path = Path(word)
    if not path.is_file():
        raise Void(f"the {what} `{word}` is not a regular file")
    return path


def body_of(text, source):
    """The body, checked against the digest the header claims."""
    claimed = [line[len(BODY_MARKER):].strip() for line in text.split("\n") if line.startswith(BODY_MARKER)]
    if len(claimed) != 1:
        raise Void(f"{source} carries {len(claimed)} `{BODY_MARKER.strip()}` line(s), and a corpus has one")
    at = text.index(BODY_MARKER)
    body = text[text.index("\n", at) + 1:]
    actual = hashlib.sha256(body.encode("utf-8")).hexdigest()
    if actual != claimed[0]:
        raise Void(f"{source} digests to {actual} and its header claims {claimed[0]}")
    return claimed[0], body


def capture_of(text, source):
    found = [line[len(CAPTURE_MARKER):].strip() for line in text.split("\n") if line.startswith(CAPTURE_MARKER)]
    if len(found) != 1:
        raise Void(f"{source} carries {len(found)} `{CAPTURE_MARKER.strip()}` line(s), and a corpus has one")
    return found[0]


def records_of(body, source):
    out = []
    for at, line in enumerate(body.split("\n")):
        if not line:
            continue
        if line.startswith("#"):
            raise Void(f"{source} record {at + 1} is a comment inside the body")
        fields = line.split("\t")
        if len(fields) != FIELDS:
            raise Void(
                f"{source} record {at + 1} carries {len(fields)} TAB-separated field(s) and a "
                f"corpus record has {FIELDS}"
            )
        try:
            depth = int(fields[DEPTH])
        except ValueError:
            raise Void(f"{source} record {at + 1}: `{fields[DEPTH]}` is not a depth")
        if str(depth) != fields[DEPTH] or depth < 0:
            raise Void(f"{source} record {at + 1}: `{fields[DEPTH]}` is not a depth")
        out.append((at + 1, fields, depth))
    if not out:
        raise Void(f"{source} holds no records")
    return out


def claim(path, give_back=None):
    """Create exclusively, or refuse by name.

    `give_back` is a claim already taken this run: a refusal here removes it,
    so a refusal leaves no half-written pair behind.
    """
    try:
        fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    except FileExistsError:
        if give_back is not None:
            held, held_path = give_back
            held.close()
            os.unlink(held_path)
        print(f"wp21_assemble: REFUSED: {path} already exists and is not overwritten", file=sys.stderr)
        raise SystemExit(REFUSED)
    return os.fdopen(fd, "w", encoding="utf-8")


def manifest(header_lines, rows):
    """A manifest in the tree's own fixture shape: comments, a body digest, rows."""
    body = "".join(f"{row}\n" for row in rows)
    digest = hashlib.sha256(body.encode("utf-8")).hexdigest()
    head = "".join(f"# {line}\n" for line in header_lines)
    return f"{head}{BODY_MARKER}{digest}\n{body}", digest


def main():
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--out-dir", required=True)
    parser.add_argument("--corpus", required=True, action="append")
    args = parser.parse_args()

    out_dir = Path(args.out_dir)
    if not out_dir.is_dir():
        raise Void(f"the output directory `{args.out_dir}` is not a directory")
    raw_path = out_dir / RAW_NAME
    deduped_path = out_dir / DEDUPED_NAME

    corpora = []
    for index, word in enumerate(args.corpus, start=1):
        path = readable(word, "corpus")
        text = path.read_text(encoding="utf-8")
        digest, body = body_of(text, path.name)
        capture = capture_of(text, path.name)
        corpora.append((index, path, digest, capture, records_of(body, path.name)))

    # Claimed after every input has been read and refused if unreadable, so a
    # void leaves nothing behind; claimed BEFORE the arithmetic, so a collision
    # is refused before a number is printed that nothing then holds.
    raw_file = claim(raw_path)
    deduped_file = claim(deduped_path, give_back=(raw_file, raw_path))

    # THE DEDUP. `chosen` maps a three-key identity to the record representing it.
    chosen = {}
    order = []
    seen_seq, seen_pos, seen_full = set(), set(), set()
    disagreements = 0
    total = 0
    for index, path, digest, capture, records in corpora:
        for line, fields, depth in records:
            total += 1
            triple = (fields[KEY_SEQ], fields[KEY_POS], fields[KEY_FULL])
            partial = (
                triple[0] in seen_seq or triple[1] in seen_pos or triple[2] in seen_full
            ) and triple not in chosen
            if partial:
                disagreements += 1
            seen_seq.add(triple[0])
            seen_pos.add(triple[1])
            seen_full.add(triple[2])
            held = chosen.get(triple)
            if held is None:
                chosen[triple] = (index, line, depth, fields)
                order.append(triple)
            elif depth > held[2]:
                chosen[triple] = (index, line, depth, fields)

    decided = sum(
        1 for triple in order
        if chosen[triple][3][RESULT] in WINS and chosen[triple][3][END] == "normal"
    )
    distinct = len(order)

    raw_rows = [
        f"{index}\t{path.name}\t{digest}\t{len(records)}\t{capture}"
        for index, path, digest, capture, records in corpora
    ]
    raw_text, raw_digest = manifest(
        [
            "wp21_assemble — the RAW manifest: every corpus assembled, with its own body digest",
            "columns: corpus_index, corpus_file, corpus_body_sha256, records, capture_sha256",
            f"param corpora {len(corpora)}",
            f"derived records {total}",
        ],
        raw_rows,
    )
    deduped_rows = [
        "\t".join(
            [
                str(chosen[t][0]),
                str(chosen[t][1]),
                t[0],
                t[1],
                t[2],
                str(chosen[t][2]),
                chosen[t][3][RESULT],
                chosen[t][3][END],
            ]
        )
        for t in order
    ]
    deduped_text, deduped_digest = manifest(
        [
            "wp21_assemble — the DEDUPED manifest: one record per distinct position",
            "rule: three-key agreement (D-562(2)); the deeper label wins; a tie goes to the first",
            "columns: corpus_index, record_line, key_seq, key_pos, key_full, depth_turns, result, end",
            "decided: result is a win and end is normal — the subset D-562(1) lets outcome enter on",
            f"param corpora {len(corpora)}",
            f"derived records {total}",
            f"derived distinct_positions {distinct}",
            f"derived decided {decided}",
            f"derived outcome_coverage {decided / distinct:.4f}",
            f"derived key_disagreements {disagreements}",
        ],
        deduped_rows,
    )
    raw_file.write(raw_text)
    raw_file.close()
    deduped_file.write(deduped_text)
    deduped_file.close()

    say(f"corpora {len(corpora)}")
    say(f"records {total}")
    say(f"distinct positions {distinct}")
    say(f"decided {decided}")
    say(f"outcome coverage {decided / distinct:.4f}")
    say(f"key disagreements {disagreements}")
    say(f"raw manifest {raw_path} body sha256 {raw_digest}")
    say(f"deduped manifest {deduped_path} body sha256 {deduped_digest}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Void as why:
        print(f"wp21_assemble: RUN VOID: {why}", file=sys.stderr)
        print("wp21_assemble: no manifest was written; this is NOT an answer", file=sys.stderr)
        sys.exit(VOID)
