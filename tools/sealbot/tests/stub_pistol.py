#!/usr/bin/env python3
"""A scripted pistol-line-protocol engine, for the matchserver tests.

Speaks the shipped binary's protocol shape exactly — the handshake (with the
`id mode` and `id budgets` lines the pistol client pins), the
silent verbs, and a `go` answered by one info line, one totals line and a
bestmove taken verbatim from a script file. It validates nothing about the
position it is handed; the REAL protocol validation is the dry run against
the real binary, which this is not.

The MODE it advertises is an argument, because the pistol client pins the
handshake's mode to the seat's budget in both directions and a suite that could
only stand up one mode could not exercise the refusal.

Usage: stub_pistol.py <script.json> [instrument|play]
       script.json = ["q,r/q,r", "q,r", ...]  one bestmove token per go
"""

from __future__ import annotations

import json
import sys


def main() -> int:
    if len(sys.argv) not in (2, 3):
        sys.stderr.write("usage: stub_pistol.py <script.json> [instrument|play]\n")
        return 2
    mode = sys.argv[2] if len(sys.argv) == 3 else "instrument"
    if mode not in ("instrument", "play"):
        sys.stderr.write(f"stub_pistol: unknown mode {mode}\n")
        return 2
    with open(sys.argv[1], encoding="utf-8") as handle:
        moves = json.load(handle)

    go_count = 0
    while True:
        line = sys.stdin.readline()
        if not line:
            break
        line = line.strip()
        if not line or line == "quit":
            break
        if line == "pistol":
            print("id name stub-pistol")
            print("id version 1")
            print(f"id mode {mode}")
            print("id budgets depth_turns nodes movetime")
            print("pistolok", flush=True)
        elif line == "newgame":
            pass
        elif line.startswith("position"):
            pass
        elif line.startswith("go"):
            # The go line reaches stderr because that is where the suite reads
            # it: which BUDGET VERB the client sent is the thing under test,
            # and nothing else in the record carries it.
            sys.stderr.write(f"stub_pistol: saw {line}\n")
            sys.stderr.flush()
            if go_count >= len(moves):
                sys.stderr.write(f"stub_pistol: script exhausted at go {go_count}\n")
                return 1
            token = moves[go_count]
            go_count += 1
            print(
                f"info depth_turns 1 seldepth 1 nodes 7 nps 7 time 1 "
                f"hashfull 0 score cp 0 pv {token}"
            )
            print(
                f"info totals depth_turns 1 seldepth 1 nodes 7 nps 7 time 1 "
                f"hashfull 0 score cp 0 pv {token}"
            )
            print(f"bestmove {token}", flush=True)
        else:
            sys.stderr.write(f"stub_pistol: unexpected line: {line}\n")
            return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
