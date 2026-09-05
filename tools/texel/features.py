"""Feature extraction for the handcrafted_v0 evaluation.

The v0 score is LINEAR in the five table entries, so a position reduces to five
integers. This module replicates `crates/pistol-eval/src/handcrafted.rs`'s
window bookkeeping exactly; `verify_against_engine` is what says so, and
nothing here is trusted until that has run.
"""

WINDOW_LEN = 6
EVAL_MAX = 16000

# crates/pistol-core/src/axis.rs:26-32 — the three line directions, in the
# order `Axis::ALL` fixes.
AXES = ((0, 1), (1, 0), (1, -1))


def windows_through(q, r):
    """Every window holding (q, r), as (axis_index, start_q, start_r).

    Mirrors `windows_through_indexed` (crates/pistol-core/src/window.rs:112).
    The addressable-lattice check is omitted: it can only fail thousands of
    turns of legal-region growth from the origin, which no corpus position
    reaches.
    """
    for axis, (dq, dr) in enumerate(AXES):
        for back in range(WINDOW_LEN):
            yield (axis, q - back * dq, r - back * dr)


def window_counts(stones):
    """Map every window holding a stone to its (p1_count, p2_count)."""
    counts = {}
    for (q, r), player in stones:
        for window in windows_through(q, r):
            cell = counts.get(window)
            if cell is None:
                cell = [0, 0]
                counts[window] = cell
            cell[player] += 1
    return counts


def features(stones):
    """The six-vector `f`, indexed by own-stone count 1..=6.

    `f[k]` is (windows holding exactly k P1 stones and no P2 stone) minus the
    same for P2. A window holding both players is dead and contributes to
    neither, which is `contribution`'s `_ => 0` arm.
    """
    f = [0] * (WINDOW_LEN + 1)
    for p1, p2 in window_counts(stones).values():
        if p2 == 0:
            f[p1] += 1
        elif p1 == 0:
            f[p2] -= 1
    return f


def score_p1(f, table):
    """P1-relative score: the linear form, then the eval band's clamp.

    `table` is the five stated entries; index 6 is `DECIDED_WINDOW_VALUE`,
    spliced in exactly as `weights.rs:179-181` splices it.
    """
    raw = sum(f[k] * table[k - 1] for k in range(1, WINDOW_LEN)) + f[WINDOW_LEN] * EVAL_MAX
    return max(-EVAL_MAX, min(EVAL_MAX, raw))


def value(f, table, side_to_move):
    """What the position is worth to `side_to_move`, in eval units."""
    clamped = score_p1(f, table)
    return clamped if side_to_move == 0 else -clamped


def stones_of(moves):
    """Replay a corpus `moves` field into (coord, player) pairs.

    Turn 1 is one stone; every later turn is two (rule 3). The mover alternates
    per TURN, not per stone.
    """
    stones = []
    if moves == "-":
        return stones
    for turn_index, turn in enumerate(moves.split()):
        player = turn_index % 2
        for cell in turn.split("/"):
            q, r = cell.split(",")
            stones.append(((int(q), int(r)), player))
    return stones
