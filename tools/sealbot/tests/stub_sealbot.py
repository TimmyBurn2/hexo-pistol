#!/usr/bin/env python3
"""A scripted sealbot-shim engine, for the matchserver tests.

Speaks the shim's JSON-lines contract exactly: a readiness preamble, then one
request line in and one reply line out, the stones taken verbatim from a script
file. Like the real shim it owns no rules; unlike the real shim it does not
think.

Usage: stub_sealbot.py <script.json>
       script.json = [[[q, r], [q, r]], ...]  one stones list per request
"""

from __future__ import annotations

import json
import sys


def main() -> int:
    if len(sys.argv) != 2:
        sys.stderr.write("usage: stub_sealbot.py <script.json>\n")
        return 2
    with open(sys.argv[1], encoding="utf-8") as handle:
        turns = json.load(handle)

    # The same preamble the real shim writes, on both streams: the client reads
    # the stdout one in `new_game` before it starts the clock, so a stub that
    # stayed silent would take a pregame forfeit in every matchserver test.
    for stream in (sys.stdout, sys.stderr):
        stream.write("sealbot_shim: ready\n")
        stream.flush()

    request_count = 0
    for line in sys.stdin:
        line = line.strip()
        if not line or line == "quit":
            break
        request = json.loads(line)
        if request_count >= len(turns):
            sys.stderr.write(
                f"stub_sealbot: script exhausted at request {request_count}\n"
            )
            return 1
        stones = turns[request_count]
        request_count += 1
        # A scripted engine spends no time; the field is REQUIRED by the client
        # (hard rule 3), so it is sent and is honest about being zero.
        sys.stdout.write(json.dumps({"moves": stones, "engine_time_ms": 0}) + "\n")
        sys.stdout.flush()
    return 0


if __name__ == "__main__":
    sys.exit(main())
