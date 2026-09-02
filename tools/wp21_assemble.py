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
#                         named by corpus index and record number. Deduped is the
#                         training input, and it is an INDEX into the raw corpora
#                         rather than a copy of them: a merged corpus would need
#                         one experiment digest and sixteen tranches have sixteen.
#
# THE RULE, from D-562(2) and applied here in one place: two records are the
# SAME position exactly when all three keys agree — key_seq, key_pos and
# key_full. Of a set of records for one position the DEEPER label wins
# (`depth_turns`), and a tie goes to the FIRST in input order, corpora in the
# order given and records in file order. So the REPRESENTATIVE of a position
# depends on the order the corpora are given, by the rule's own tie clause, and
# every COUNT below is order-free.
#
# KEY DISAGREEMENTS. A distinct position that shares at least one key value with
# another distinct position is a record where the three keys DISAGREE about
# sameness — a transposition or a symmetry image the exact triple keeps apart.
# They are kept distinct and COUNTED, order-free, because which key rules such a
# pair is the question D-562(2) leaves open and nothing here may settle it.
#
# OUTCOME COVERAGE is the fraction of DEDUPED positions whose representing
# record's game was DECIDED by the rules: `result` is a win and `end` is
# `normal`. A capped game is a horizon and a forfeit is the driver's, and
# D-562(1) lets outcome enter only on the decided subset. The denominator is the
# deduped count, not the raw one: two records of one position may disagree on
# outcome across games, and the training input holds one of them.
#
# WHAT IT REFUSES, BY NAME, AS A VOID. Anything the corpus reader
# (crates/pistol-arena/src/labels_file.rs) refuses: a schema other than 1, a
# field count other than sixteen, an empty field, a result or end outside the
# closed sets, a body that does not digest to its header. And two things the
# reader cannot see because it reads one file: the same corpus twice (one
# tranche listed twice doubles `records`), and corpora labelled at different
# `label_go` lines (one training input, two teachers).
#
# IT NEVER OVERWRITES AND LEAVES NO HALF-WRITTEN PAIR. Both texts are rendered
# before either file is claimed; both are claimed exclusively; a refusal or a
# failed write removes what this run created and nothing else.
#
# Usage:
#   tools/wp21_assemble.py --out-dir <dir> --corpus <path> [--corpus <path>]...
#
# Exit:  0 both manifests written; each one's own body sha256 is printed
#        1 a named refusal — an output that already exists
#        2 THE RUN IS VOID: no manifest was written and no answer was taken
#          (tools/SHELL_CHECKLIST.md item 12). A void is not a refusal.

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
RESULTS = ("p1_win", "p2_win", "capped")
ENDS = ("normal", "forfeit")
WINS = ("p1_win", "p2_win")
SCHEMA_MARKER = "# param corpus_schema_version "
LABEL_GO_MARKER = "# param label_go "
BODY_MARKER = "# body_sha256 "
CAPTURE_MARKER = "# derived capture_sha256 "
SCHEMA = "1"
# Line separators a `splitlines()` reader honours and a `split("\n")` reader
# does not; a basename carrying one would inject a row into a receipt.
LINE_BREAKERS = ("\u0085", "\u2028", "\u2029")

RAW_NAME = "raw_manifest.txt"
DEDUPED_NAME = "deduped_manifest.txt"


class Void(Exception):
    """No answer could be taken."""


def say(what):
    print(f"wp21_assemble: {what}")


def readable(word, what):
    if not word.isprintable() or any(c in word for c in LINE_BREAKERS):
        raise Void(f"the {what} path carries a character a receipt cannot hold: {word!r}")
    path = Path(word)
    if not path.is_file():
        raise Void(f"the {what} `{word}` is not a regular file")
    return path


def text_of(path):
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError as why:
        raise Void(f"{path.name} is not UTF-8: {why}")
    except OSError as why:
        raise Void(f"{path.name} cannot be read: {why}")


def one_header(text, marker, source):
    found = [line[len(marker):].strip() for line in text.split("\n") if line.startswith(marker)]
    if len(found) != 1:
        raise Void(f"{source} carries {len(found)} `{marker.strip()}` line(s), and a corpus has one")
    return found[0]


def body_of(text, source):
    """The body, checked against the digest the header claims."""
    claimed = one_header(text, BODY_MARKER, source)
    at = text.index(BODY_MARKER)
    body = text[text.index("\n", at) + 1:]
    actual = hashlib.sha256(body.encode("utf-8")).hexdigest()
    if actual != claimed:
        raise Void(f"{source} digests to {actual} and its header claims {claimed}")
    return claimed, body


def records_of(body, source):
    """The records, numbered as the corpus reader numbers them."""
    out = []
    number = 0
    lines = body.split("\n")
    if lines and lines[-1] == "":
        lines.pop()
    for line in lines:
        if line == "":
            raise Void(f"{source} carries an empty line inside its body, after record {number}")
        if line.startswith("#"):
            raise Void(f"{source} record {number + 1} is a comment inside the body")
        fields = line.split("\t")
        number += 1
        if len(fields) != FIELDS:
            raise Void(
                f"{source} record {number} carries {len(fields)} TAB-separated field(s) and a "
                f"corpus record has {FIELDS}"
            )
        if any(field == "" for field in fields):
            raise Void(f"{source} record {number} carries an empty field")
        try:
            depth = int(fields[DEPTH])
        except ValueError:
            raise Void(f"{source} record {number}: `{fields[DEPTH]}` is not a depth")
        if str(depth) != fields[DEPTH] or depth < 0:
            raise Void(f"{source} record {number}: `{fields[DEPTH]}` is not a depth")
        if fields[RESULT] not in RESULTS:
            raise Void(f"{source} record {number}: `{fields[RESULT]}` is not a result this corpus writes")
        if fields[END] not in ENDS:
            raise Void(f"{source} record {number}: `{fields[END]}` is not an end this corpus writes")
        out.append((number, fields, depth))
    if not out:
        raise Void(f"{source} holds no records")
    return out


def corpus_of(index, word):
    path = readable(word, "corpus")
    text = text_of(path)
    source = path.name
    schema = one_header(text, SCHEMA_MARKER, source)
    if schema != SCHEMA:
        raise Void(f"{source} is corpus schema {schema}, and this instrument reads schema {SCHEMA}")
    label_go = one_header(text, LABEL_GO_MARKER, source)
    digest, body = body_of(text, source)
    capture = one_header(text, CAPTURE_MARKER, source)
    return index, path, digest, capture, label_go, records_of(body, source)


def manifest(header_lines, rows):
    """A manifest in the tree's own fixture shape: comments, a body digest, rows."""
    body = "".join(f"{row}\n" for row in rows)
    digest = hashlib.sha256(body.encode("utf-8")).hexdigest()
    head = "".join(f"# {line}\n" for line in header_lines)
    return f"{head}{BODY_MARKER}{digest}\n{body}", digest


def dedup(corpora):
    """D-562(2)'s default, applied once, plus the order-free counts."""
    chosen = {}
    order = []
    total = 0
    for index, _path, _digest, _capture, _go, records in corpora:
        for number, fields, depth in records:
            total += 1
            triple = (fields[KEY_SEQ], fields[KEY_POS], fields[KEY_FULL])
            held = chosen.get(triple)
            if held is None:
                chosen[triple] = (index, number, depth, fields)
                order.append(triple)
            elif depth > held[2]:
                chosen[triple] = (index, number, depth, fields)
    # A distinct triple sharing any one key value with another distinct triple.
    multiplicity = [{} for _ in range(3)]
    for triple in order:
        for column, key in enumerate(triple):
            multiplicity[column][key] = multiplicity[column].get(key, 0) + 1
    disagreements = sum(
        1 for triple in order
        if any(multiplicity[column][key] > 1 for column, key in enumerate(triple))
    )
    decided = sum(
        1 for triple in order
        if chosen[triple][3][RESULT] in WINS and chosen[triple][3][END] == "normal"
    )
    return chosen, order, total, decided, disagreements


def claim(path):
    try:
        fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    except FileExistsError:
        return None
    except OSError as why:
        raise Void(f"cannot create {path}: {why}")
    return os.fdopen(fd, "w", encoding="utf-8")


def write_pair(raw_path, raw_text, deduped_path, deduped_text):
    """Claim both exclusively and write both, or leave nothing of this run's."""
    created = []
    try:
        for path, text in ((raw_path, raw_text), (deduped_path, deduped_text)):
            handle = claim(path)
            if handle is None:
                for made in created:
                    os.unlink(made)
                print(
                    f"wp21_assemble: REFUSED: {path} already exists and is not overwritten",
                    file=sys.stderr,
                )
                raise SystemExit(REFUSED)
            created.append(path)
            with handle:
                handle.write(text)
    except OSError as why:
        for made in created:
            try:
                os.unlink(made)
            except OSError:
                pass
        raise Void(f"writing the manifests failed and both were removed: {why}")


class Arguments(argparse.ArgumentParser):
    """An argument error is a VOID with the instrument's own vocabulary."""

    def error(self, message):
        raise Void(f"arguments: {message}")


def main():
    parser = Arguments(add_help=True)
    parser.add_argument("--out-dir", required=True)
    parser.add_argument("--corpus", required=True, action="append")
    args = parser.parse_args()

    out_dir = Path(args.out_dir)
    if not out_dir.is_dir():
        raise Void(f"the output directory `{args.out_dir}` is not a directory")
    raw_path = out_dir / RAW_NAME
    deduped_path = out_dir / DEDUPED_NAME

    corpora = [corpus_of(index, word) for index, word in enumerate(args.corpus, start=1)]
    seen_digest = {}
    seen_capture = {}
    label_go = corpora[0][4]
    for index, path, digest, capture, go, _records in corpora:
        if go != label_go:
            raise Void(
                f"corpus {index} ({path.name}) was labelled at `{go}` and corpus 1 at "
                f"`{label_go}`: one training input cannot carry two teachers"
            )
        if digest in seen_digest:
            raise Void(
                f"corpus {index} ({path.name}) has the same body digest as corpus "
                f"{seen_digest[digest]}: one tranche given twice would count twice"
            )
        if capture in seen_capture:
            raise Void(
                f"corpus {index} ({path.name}) was derived from the same capture as corpus "
                f"{seen_capture[capture]}: one tranche given twice would count twice"
            )
        seen_digest[digest] = index
        seen_capture[capture] = index

    chosen, order, total, decided, disagreements = dedup(corpora)
    distinct = len(order)

    raw_text, raw_digest = manifest(
        [
            "wp21_assemble — the RAW manifest: every corpus assembled, with its own body digest",
            "columns: corpus_index, corpus_file, corpus_body_sha256, records, capture_sha256",
            f"param label_go {label_go}",
            f"param corpora {len(corpora)}",
            f"derived records {total}",
        ],
        [
            f"{index}\t{path.name}\t{digest}\t{len(records)}\t{capture}"
            for index, path, digest, capture, _go, records in corpora
        ],
    )
    deduped_text, deduped_digest = manifest(
        [
            "wp21_assemble — the DEDUPED manifest: one record per distinct position",
            "rule: three-key agreement (D-562(2)); the deeper label wins; a tie goes to the first",
            "columns: corpus_index, record_number, key_seq, key_pos, key_full, depth_turns, result, end",
            "decided: result is a win and end is normal — the subset D-562(1) lets outcome enter on",
            "outcome_coverage: decided over DEDUPED positions, not over raw records",
            "key_disagreements: distinct positions sharing any one key value with another; order-free",
            f"param label_go {label_go}",
            f"param corpora {len(corpora)}",
            f"derived records {total}",
            f"derived distinct_positions {distinct}",
            f"derived decided {decided}",
            f"derived outcome_coverage {decided / distinct:.4f}",
            f"derived key_disagreements {disagreements}",
        ],
        [
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
        ],
    )
    write_pair(raw_path, raw_text, deduped_path, deduped_text)

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
