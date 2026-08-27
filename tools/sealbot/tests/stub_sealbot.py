#!/usr/bin/env python3
"""A scripted sealbot-shim engine, for the matchserver tests.

Speaks the shim's JSON-lines contract exactly: one request line in, one
reply line out, the stones taken verbatim from a script file. Like the real
shim it owns no rules; unlike the real shim it does not think.

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
        sys.stdout.write(json.dumps({"moves": stones}) + "\n")
        sys.stdout.flush()
    return 0


if __name__ == "__main__":
    sys.exit(main())
