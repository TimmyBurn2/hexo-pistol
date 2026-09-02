#!/usr/bin/env python3
#
# The label cache's yield, counted on a real capture
# (docs/experiments/wp21_throughput_prereg.md §4.1).
#
# WHAT IT ANSWERS. `arena --capture` asks the engine once per asked prefix. A
# label cache memoises the answer under the `position` line's exact bytes, so
# its yield is the ratio of asked prefixes to DISTINCT `position` lines. This
# script counts that ratio on a capture file, and counts three coarser keys
# beside it so the choice of key is decided by a measurement rather than by an
# argument.
#
# WHY THE COARSER KEYS ARE COUNTED HERE AND NOT ARGUED. The coarser folds
# (transposition, symmetry) each admit a class of wrong answer, and are worth
# nothing unless they merge a pair the exact key does not. Printing all four
# side by side is what makes "the fold merges nothing" a reading rather than a
# claim.
#
# WHAT IT DOES NOT WRITE. Nothing. It reads one capture file and prints. There
# is no destructive site in this file (tools/SHELL_CHECKLIST.md item 11
# answered by enumeration: none).
#
# Usage:
#   tools/label_cache_count.py --capture <path>
#
# Exit:  0 the count was taken
#        2 THE RUN IS VOID — no count was taken (tools/SHELL_CHECKLIST.md
#          item 12). A void is not an answer of zero.

import argparse
import hashlib
import sys
from collections import Counter
from pathlib import Path

BODY_MARKER = "# body_sha256 "
FIELDS = 5
POSITION = 2
VOID = 2


class Void(Exception):
    """No count could be taken. Never an answer."""


def say(what):
    print(f"label_cache_count: {what}")


def readable(word, what):
    """A caller's path, guarded at the boundary before it reaches a record.

    A control character in a value this script prints would inject lines into a
    receipt somebody parses (tools/SHELL_CHECKLIST.md item 9).
    """
    if any(ord(c) < 0x20 or ord(c) == 0x7F for c in word):
        raise Void(f"the {what} path carries a control character: {word!r}")
    path = Path(word)
    if not path.is_file():
        raise Void(f"the {what} `{word}` is not a regular file")
    return path


def body_of(text, source):
    """The capture's body, checked against the digest its own header claims."""
    claimed = None
    for line in text.split("\n"):
        if line.startswith(BODY_MARKER):
            if claimed is not None:
                raise Void(f"{source} carries more than one `{BODY_MARKER.strip()}` line")
            claimed = line[len(BODY_MARKER):].strip()
    if claimed is None:
        raise Void(f"{source} carries no body digest, so nothing binds its records")
    at = text.index(BODY_MARKER)
    after = text.index("\n", at) + 1
    body = text[after:]
    actual = hashlib.sha256(body.encode("utf-8")).hexdigest()
    if actual != claimed:
        raise Void(f"{source} digests to {actual} and its header claims {claimed}")
    return body


def records_of(body, source):
    """The capture's records, in the order they were asked."""
    out = []
    for at, line in enumerate(body.split("\n")):
        if not line or line.startswith("#"):
            continue
        fields = line.split("\t")
        if len(fields) != FIELDS:
            raise Void(
                f"{source} record {at + 1} carries {len(fields)} TAB-separated field(s) "
                f"and a capture record has {FIELDS}"
            )
        out.append(fields)
    if not out:
        raise Void(f"{source} holds no records, so there is nothing to count")
    return out


def stones_of(position, where):
    """The (cell, player) list a `position` line replays to, in play order.

    Derived from the line's own turn tokens rather than from an engine: turn 1
    is one stone, every later turn is two by the mover, and the movers
    alternate (game rules 3 and 4). A truncated last turn is a rule-4 win and
    carries one stone.
    """
    if position == "position start":
        return []
    prefix = "position start moves "
    if not position.startswith(prefix):
        raise Void(f"{where}: `{position}` is not a position line this script reads")
    stones = []
    for at, turn in enumerate(position[len(prefix):].split(" ")):
        if not turn:
            raise Void(f"{where}: `{position}` carries an empty turn token")
        player = "p1" if at % 2 == 0 else "p2"
        for cell in turn.split("/"):
            stones.append((cell, player))
    return stones


def axial(cell, where):
    """One `q,r` token as a pair of integers."""
    parts = cell.split(",")
    if len(parts) != 2:
        raise Void(f"{where}: `{cell}` is not a `q,r` cell")
    try:
        return int(parts[0]), int(parts[1])
    except ValueError:
        raise Void(f"{where}: `{cell}` is not a `q,r` cell")


# The six rotations of the hex lattice in axial coordinates, each composed with
# the identity and with one reflection: the twelve images `canonical_form`
# folds (docs/decisions.md D-8, D-137).
def images(stones, where):
    """The twelve symmetry images of a stone list, each sorted."""
    pts = [(axial(cell, where), player) for cell, player in stones]
    out = []
    for reflect in (False, True):
        cur = [((q, r) if not reflect else (r, q), p) for (q, r), p in pts]
        for _ in range(6):
            cur = [((-r, q + r), p) for (q, r), p in cur]
            out.append(tuple(sorted(cur)))
    return out


def main():
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--capture", required=True)
    args = parser.parse_args()

    path = readable(args.capture, "capture")
    text = path.read_text(encoding="utf-8")
    source = path.name
    body = body_of(text, source)
    rows = records_of(body, source)

    exact = set()
    stoneset = set()
    folded = set()
    for at, row in enumerate(rows):
        where = f"record {at}"
        position = row[POSITION]
        exact.add(position)
        stones = stones_of(position, where)
        stoneset.add(tuple(sorted(stones)))
        folded.add(min(images(stones, where)))

    asked = len(rows)
    say(f"capture {source}, body sha256 {hashlib.sha256(body.encode('utf-8')).hexdigest()}")
    say(f"asked prefixes                          {asked}")
    say(f"distinct `position` lines (cache key)   {len(exact)}")
    say(f"distinct sorted (cell, player) lists    {len(stoneset)}")
    say(f"distinct symmetry-folded stone lists    {len(folded)}")
    say(f"duplication factor (asked/cache key)    {asked / len(exact):.4f}")
    say(f"cache hits                              {asked - len(exact)}")
    say(f"cache misses                            {len(exact)}")
    say(f"hit rate                                {(asked - len(exact)) / asked:.4f}")
    # THE MULTIPLICITY STRUCTURE, because a ratio alone cannot say WHERE the
    # duplication comes from: two seats replaying one game and one prefix
    # shared by every game are different mechanisms with different scaling
    # laws, and a reader who takes the second for the first looks for a fold
    # that is not there.
    counts = Counter(Counter(row[POSITION] for row in rows).values())
    for multiplicity in sorted(counts):
        say(
            f"  {counts[multiplicity]:>6} `position` line(s) asked "
            f"{multiplicity} time(s)"
        )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Void as why:
        print(f"label_cache_count: RUN VOID: {why}", file=sys.stderr)
        print(
            "label_cache_count: no count was taken; this is NOT an answer of zero",
            file=sys.stderr,
        )
        sys.exit(VOID)
