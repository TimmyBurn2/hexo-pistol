"""A fake `minimax_cpp` for driving the REAL shim in CI.

`sealbot_shim.py` takes its two module directories from argv, so the shipped
shim can be exercised end to end without the sealbot checkout — which is what
makes the shim's stdout preamble and its `engine_time_ms` behavioural facts a
gate can hold, rather than source text a guard greps for.

The bot SLEEPS a known fraction of its limit, so `engine_time_ms` can be
checked against something other than the budget: a shim reporting its
configured `time_limit` instead of its elapsed time is the defect that would
launder a configured ratio as a measured one, and only a bot that deliberately
does not use its whole budget separates the two.
"""

import time

SLEEP_FRACTION = 0.25


class MinimaxBot:
    def __init__(self, time_limit):
        self.time_limit = time_limit

    def get_move(self, game):
        time.sleep(self.time_limit * SLEEP_FRACTION)
        return [(1, 1), (-1, 1)]
