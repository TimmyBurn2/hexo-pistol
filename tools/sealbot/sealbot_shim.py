#!/usr/bin/env python3
"""The sealbot engine shim: JSON lines in, JSON lines out.

The matchserver's sealbot seat drives this shim; the shim drives sealbot.
Everything sealbot-specific lives here (its python, its module paths, its
HexGame replay), so the matchserver's client stays a thin protocol driver and
no game rule is implemented outside pistol-core (tools/sealbot/README.md).

This is the same replay pattern hexo-bridge's seal_perf_engine.py ships,
reached over a simpler contract:

    argv:  sealbot_shim.py <bot_dir> <root_dir>
    stdin:  one JSON object per request:
            {"setup": [[q, r], ...], "moves": [[q, r], ...],
             "time_limit": <seconds>}
    stdout: one JSON object per reply:
            {"moves": [[q, r], ...]}

`setup` is the server-played opening (the origin cross); `moves` is every
stone after it, in true play order. `time_limit` comes with every request —
the matchserver's config is the single source of the budget — and a request
without one falls back to a conservative default rather than guessing big.

On any failure the shim exits nonzero WITHOUT a reply: the matchserver reads
that as an engine failure (a forfeit), never as a move.
"""

from __future__ import annotations

import json
import sys


def main() -> int:
    if len(sys.argv) != 3:
        sys.stderr.write("usage: sealbot_shim.py <bot_dir> <root_dir>\n")
        return 2
    bot_dir, root_dir = sys.argv[1], sys.argv[2]
    default_limit = 0.1

    sys.path.insert(0, bot_dir)
    from minimax_cpp import MinimaxBot

    sys.path.insert(0, root_dir)
    from game import HexGame

    bot = MinimaxBot(default_limit)
    sys.stderr.write("sealbot_shim: ready\n")
    sys.stderr.flush()

    for line in sys.stdin:
        line = line.strip()
        if not line or line == "quit":
            break
        request = json.loads(line)
        game = HexGame(win_length=6)
        game.reset()
        # The server-played opening first, then every stone in play order —
        # sealbot's own HexGame applies its own turn model to both.
        for q, r in request.get("setup", []):
            game.make_move(int(q), int(r))
        for q, r in request.get("moves", []):
            game.make_move(int(q), int(r))
        bot.time_limit = float(request.get("time_limit", default_limit))
        result = bot.get_move(game)
        reply = {"moves": [[int(q), int(r)] for q, r in result]}
        sys.stdout.write(json.dumps(reply) + "\n")
        sys.stdout.flush()
    return 0


if __name__ == "__main__":
    sys.exit(main())
