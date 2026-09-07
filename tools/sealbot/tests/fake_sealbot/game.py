"""A fake `game` module for driving the REAL shim in CI. See minimax_cpp.py."""


class HexGame:
    def __init__(self, win_length=6):
        self.win_length = win_length
        self.stones = []

    def reset(self):
        self.stones = []

    def make_move(self, q, r):
        self.stones.append((q, r))
