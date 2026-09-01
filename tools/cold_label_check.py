#!/usr/bin/env python3
#
# The cold-label agreement check (docs/decisions.md D-540).
#
# WHAT IT ANSWERS. A capture (`arena --capture`) asks every recorded position on
# ONE long-lived engine process, sending `newgame` before each ask. "Cold by
# construction" is the claim that a `newgame` makes that ask indistinguishable
# from the first ask of a fresh process. This script is the EXTERNAL REFERENT
# that check needs: it re-asks a registered sample of the capture's own records,
# ONE PROCESS PER POSITION, and compares the two answers byte for byte.
#
# WHY AN EXTERNAL REFERENT AND NOT AN INTERNAL AGREEMENT. D-527 records a census
# taken on a warm transposition table whose own check "passed vacuously on the
# two bands a warm table cannot move". A fresh process shares no transposition
# table, no heuristic table and no solver table with the capture pass, so the
# defect cannot be present in both the subject and the referent
# (docs/process.md, "Criterion and defect class").
#
# THE NORMALISATION IS RE-DERIVED HERE ON PURPOSE. `capture::normalise` strips
# ` nps <n> time <n>` from the totals line, which is gate 9's own rule
# (tools/determinism.sh). This script implements that strip independently rather
# than calling the arena's, so a defect in one implementation shows up as a
# DISAGREEMENT. The error direction is safe: an independent strip can only
# manufacture a mismatch, never hide one.
#
# WHAT IT DOES NOT WRITE. Nothing. It reads the capture, spawns the engine, and
# prints. There is no destructive site in this file (tools/SHELL_CHECKLIST.md
# item 11 answered by enumeration: none).
#
# Usage:
#   tools/cold_label_check.py --capture <path> --binary <path>
#                             --engine-config <path> --stride <n>
#                             [--timeout-s <n>]
#
# Exit:  0 every sampled record agrees byte for byte — THE ANSWER IS YES
#        1 a sampled record disagrees — THE ANSWER IS NO
#        2 THE RUN IS VOID: no answer was taken (tools/SHELL_CHECKLIST.md item
#          12). A void is not a disagreement and must not be read as one.

import argparse
import hashlib
import re
import subprocess
import sys
from pathlib import Path

# The two wall-clock fields, adjacent and in this order, exactly as
# `crates/pistol-cli/src/report.rs:82-84` writes them and as
# `crates/pistol-arena/src/capture.rs:65-95` removes them.
WALL_CLOCK = re.compile(r" nps \d+ time \d+")

BODY_MARKER = "# body_sha256 "
FIELDS = 5

VOID = 2
DISAGREES = 1


class Void(Exception):
    """No answer could be taken. Never a disagreement."""


def say(what):
    print(f"cold_label_check: {what}")


def spelled(word, what):
    """A count whose SPELLING is validated, not only its value.

    `+4`, ` 4` and `04` all parse to four and would land in a receipt
    unnormalised, describing a run nobody reproduces by copying the line back
    (tools/SHELL_CHECKLIST.md item 8).
    """
    try:
        value = int(word)
    except ValueError:
        raise Void(f"`{word}` is not a {what}")
    if str(value) != word:
        raise Void(
            f"`{word}` is a {what} spelled a way this program will not echo back; "
            f"write it as `{value}`"
        )
    if value < 1:
        raise Void(f"a {what} of {value} asks for nothing at all")
    return value


def readable(word, what):
    """A caller's path, guarded at the boundary before it reaches a record.

    A newline or a control character in a value this script prints would INJECT
    LINES into a receipt somebody parses (tools/SHELL_CHECKLIST.md item 9).
    """
    if any(ord(c) < 0x20 or ord(c) == 0x7F for c in word):
        raise Void(f"the {what} path carries a control character: {word!r}")
    path = Path(word)
    if not path.is_file():
        raise Void(f"the {what} `{word}` is not a regular file")
    return path


def body_of(text, source):
    """The capture's body, checked against the digest its own header claims.

    A body that does not digest to its header is not a capture this instrument
    can be a referent for, and saying so is a VOID rather than a disagreement.
    """
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


def header(text, kind, key, source):
    """One header value, or a void naming the key."""
    prefix = f"# {kind} {key} "
    found = [line[len(prefix):].strip() for line in text.split("\n") if line.startswith(prefix)]
    if not found:
        raise Void(f"{source} carries no `{kind} {key}` line")
    if len(found) > 1:
        raise Void(f"{source} carries more than one `{kind} {key}` line")
    return found[0]


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
        raise Void(f"{source} holds no records, so there is nothing to re-ask")
    return out


def ask_cold(binary, config, position, go, timeout_s, where):
    """One fresh process, one position, the engine's own two lines.

    The process is the referent: it shares no table with the capture pass, and
    it is spawned, asked and quit before the next sample is taken.
    """
    script = f"newgame\n{position}\n{go}\nquit\n"
    try:
        done = subprocess.run(
            [str(binary), "--config", str(config)],
            input=script,
            capture_output=True,
            text=True,
            timeout=timeout_s,
        )
    except subprocess.TimeoutExpired:
        raise Void(f"{where}: the engine did not answer inside {timeout_s} s")
    except OSError as why:
        raise Void(f"{where}: the engine could not be spawned: {why}")
    if done.returncode != 0:
        raise Void(
            f"{where}: the engine exited {done.returncode}; stderr: "
            f"{done.stderr.strip()[:400]}"
        )
    totals = None
    best = None
    for line in done.stdout.split("\n"):
        line = line.rstrip("\r")
        if line.startswith("info totals "):
            totals = line
        elif line.startswith("bestmove "):
            best = line
        elif line.startswith("error "):
            raise Void(f"{where}: the engine refused: `{line}`")
    if totals is None:
        raise Void(f"{where}: the engine wrote no `info totals` line")
    if best is None:
        raise Void(f"{where}: the engine wrote no `bestmove` line")
    return totals, best


def normalise(line, where):
    """The totals line without ` nps <n> time <n>`, gate 9's own two fields."""
    stripped, count = WALL_CLOCK.subn("", line, count=1)
    if count != 1:
        raise Void(
            f"{where}: the engine's totals line carries no adjacent `nps <n> time <n>` "
            f"pair, so this instrument cannot normalise it: `{line}`"
        )
    return stripped


def main():
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--capture", required=True)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--engine-config", required=True)
    parser.add_argument("--stride", required=True)
    parser.add_argument("--timeout-s", default="600")
    args = parser.parse_args()

    capture_path = readable(args.capture, "capture")
    binary = readable(args.binary, "engine binary")
    config = readable(args.engine_config, "engine config")
    stride = spelled(args.stride, "stride")
    timeout_s = spelled(args.timeout_s, "timeout in seconds")

    text = capture_path.read_text(encoding="utf-8")
    source = capture_path.name
    go = header(text, "param", "label_go", source)
    body = body_of(text, source)
    rows = records_of(body, source)

    sampled = [(at, row) for at, row in enumerate(rows) if at % stride == 0]
    say(
        f"{len(rows)} record(s) in {source}; the sample is every record whose "
        f"zero-based index is a multiple of {stride}, which is {len(sampled)} of them"
    )
    say(f"the label ask is `{go}`, one fresh process per sampled position")

    disagreements = []
    for at, row in sampled:
        game, turns, position, totals, best = row
        where = f"record {at} (game {game}, turn {turns})"
        cold_totals, cold_best = ask_cold(binary, config, position, go, timeout_s, where)
        cold_totals = normalise(cold_totals, where)
        if cold_totals != totals:
            disagreements.append(f"{where}: totals\n  capture: {totals}\n  cold:    {cold_totals}")
        if cold_best != best:
            disagreements.append(f"{where}: bestmove\n  capture: {best}\n  cold:    {cold_best}")

    if disagreements:
        say(f"{len(disagreements)} DISAGREEMENT(S) over {len(sampled)} sampled record(s):")
        for one in disagreements:
            print(one)
        say("a cold re-ask did not reproduce the capture's own bytes")
        return DISAGREES
    say(f"{len(sampled)} of {len(sampled)} sampled record(s) agree byte for byte")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Void as why:
        print(f"cold_label_check: RUN VOID: {why}", file=sys.stderr)
        print(
            "cold_label_check: no answer was taken; this is NOT a disagreement",
            file=sys.stderr,
        )
        sys.exit(VOID)
