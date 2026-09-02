#!/usr/bin/env python3
#
# How much a coarser key would fold, over an opening book's own prefixes.
#
# WHAT IT ANSWERS. The label cache keys on the `position` line's exact bytes
# (docs/decisions.md D-576). A coarser key — the position's stone set, or that
# set folded over the lattice's twelve symmetry images — would merge more asks
# into one, at the cost of answering with another position's `bestmove`. This
# counts the merges the coarser keys would make, per prefix depth, over a
# registered window of a book.
#
# WHY IT EXISTS. `docs/experiments/matrix_label_cache_key.md` revision 2 argued
# a-priori that a book deduped by `canonical_form` cannot hold two openings that
# transpose or mirror at `k <= opening_turns`. THE ARGUMENT IS FALSE AT k < 3
# and true at k = 3: deduping the WHOLE opening says nothing about its prefixes.
# This instrument is what the argument should have been.
#
# WHAT IT DOES NOT DO. It reads a book, not a capture: it says what the openings
# themselves would fold, and nothing about transpositions that arise later in a
# game. Beyond `opening_turns` the question belongs to the sweep's own per-tranche
# counters (D-581).
#
# Usage:
#   tools/opening_prefix_fold.py --book <path> --skip <n> --take <n>
#
# Exit:  0 the count was taken
#        2 THE RUN IS VOID — no count was taken (tools/SHELL_CHECKLIST.md item
#          12). A void is not an answer of zero.

import argparse
import collections
import sys
from pathlib import Path

VOID = 2


class Void(Exception):
    """No count could be taken. Never an answer."""


def say(what):
    print(f"opening_prefix_fold: {what}")


def readable(word, what):
    """A caller's path, guarded at the boundary before it reaches a record."""
    if any(ord(c) < 0x20 or ord(c) == 0x7F for c in word):
        raise Void(f"the {what} path carries a control character: {word!r}")
    path = Path(word)
    if not path.is_file():
        raise Void(f"the {what} `{word}` is not a regular file")
    return path


def spelled(word, what):
    """A count whose SPELLING is validated, not only its value."""
    try:
        value = int(word)
    except ValueError:
        raise Void(f"`{word}` is not a {what}")
    if str(value) != word:
        raise Void(f"`{word}` is a {what} spelled a way this program will not echo back")
    if value < 0:
        raise Void(f"a {what} of {value} asks for nothing at all")
    return value


def axial(cell, where):
    parts = cell.split(",")
    if len(parts) != 2:
        raise Void(f"{where}: `{cell}` is not a `q,r` cell")
    try:
        return int(parts[0]), int(parts[1])
    except ValueError:
        raise Void(f"{where}: `{cell}` is not a `q,r` cell")


def images(stones):
    """The twelve symmetry images of a stone list, each sorted.

    Six rotations composed with the identity and with one reflection: the fold
    `pistol_core::canonical_form` defines (docs/decisions.md D-8, D-137).
    """
    out = []
    for reflect in (False, True):
        cur = [((q, r) if not reflect else (r, q), p) for (q, r), p in stones]
        for _ in range(6):
            cur = [((-r, q + r), p) for (q, r), p in cur]
            out.append(tuple(sorted(cur)))
    return out


def prefix_stones(line, turns, where):
    """The stones of a line's first `turns` turns, with their movers.

    Turn 1 is one stone and every later turn is two by the mover, the movers
    alternating (game rules 3 and 4).
    """
    tokens = line.split(" ")
    if tokens[:2] != ["start", "moves"]:
        raise Void(f"{where}: `{line}` is not a `start moves …` opening line")
    stones = []
    for at, turn in enumerate(tokens[2:][:turns]):
        player = "p1" if at % 2 == 0 else "p2"
        for cell in turn.split("/"):
            stones.append((axial(cell, where), player))
    return stones


def main():
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--book", required=True)
    parser.add_argument("--skip", required=True)
    parser.add_argument("--take", required=True)
    args = parser.parse_args()

    path = readable(args.book, "book")
    skip = spelled(args.skip, "skip")
    take = spelled(args.take, "take")
    body = [
        line.rstrip("\n")
        for line in path.read_text(encoding="utf-8").split("\n")
        if line.strip() and not line.startswith("#")
    ]
    if skip + take > len(body):
        raise Void(
            f"skip {skip} + take {take} exceeds the book's {len(body)} opening(s)"
        )
    window = body[skip : skip + take]
    if not window:
        raise Void("the window holds no openings, so there is nothing to count")

    turns = max(len(line.split(" ")) - 2 for line in window)
    say(f"book {path.name}, openings {skip}..{skip + take - 1} ({take}), {turns} turn(s) deep")
    say("k   exact-key classes   stone-set classes  sharing   symmetry classes  sharing")
    for k in range(1, turns + 1):
        exact = collections.Counter()
        stoneset = collections.Counter()
        folded = collections.Counter()
        for at, line in enumerate(window):
            where = f"opening {skip + at}"
            stones = prefix_stones(line, k, where)
            exact[" ".join(line.split(" ")[2:][:k])] += 1
            stoneset[tuple(sorted(stones))] += 1
            folded[min(images(stones))] += 1
        share_set = sum(n for n in stoneset.values() if n > 1)
        share_fold = sum(n for n in folded.values() if n > 1)
        say(
            f"{k}   {len(exact):17d}   {len(stoneset):17d}  {share_set:7d}   "
            f"{len(folded):16d}  {share_fold:7d}"
        )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Void as why:
        print(f"opening_prefix_fold: RUN VOID: {why}", file=sys.stderr)
        print(
            "opening_prefix_fold: no count was taken; this is NOT an answer of zero",
            file=sys.stderr,
        )
        sys.exit(VOID)
