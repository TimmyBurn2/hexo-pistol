"""Count the census quantities WP-2.2 §B adjudicates on.

THE PARTITION IS THE ALTERNATIVE'S OWN. D-537's floor is powered against
`p1 = 12/14`, which `tools/stage3_allocator_bound.py` produces by
`knapsack_bound` over its `COLUMNS` tuple, applied atomically — its own words:
"A score is a function of the columns, so it cannot tell two firings apart when
every column agrees. The finest partition it can act on is the column-vector
CLASS." So the count of DISTINGUISHABLE trials among win-proving firings is the
count of distinct column-vector classes, and it is neither the distinct-key
count (which over-counts) nor a count over any one candidate list (which is the
NULL's family and under-counts).

Usage: census_classes.py <census-output> [<positions>]
"""

import sys

# tools/stage3_allocator_bound.py COLUMNS, in its order.
COLUMNS = (
    "turns", "mover_hot", "opp_hot", "mover_w1", "opp_w1",
    "mover_l3", "opp_l3", "cover", "covers",
)


def rows_of(path):
    """Every census row, as a dict of its printed columns."""
    for line in open(path):
        if " row entry " not in line:
            continue
        words = line.split()
        yield {words[i]: words[i + 1] for i in range(2, len(words) - 1)}


def tally(path):
    """Keys, roots and column-classes over the WIN-proving rows, plus the
    loss-direction key count, which is reported separately and never summed
    (D-522, D-535)."""
    keys, roots, classes, loss = set(), set(), set(), set()
    firings = 0
    for row in rows_of(path):
        firings += 1
        if row["att_proved"] == "true":
            keys.add(row["key"])
            roots.add(row["entry"])
            classes.add(tuple(row[c] for c in COLUMNS))
        if row["def_proved"] == "true":
            loss.add(row["key"])
    return firings, keys, roots, classes, loss


def curve(path, step=100):
    """Distinct classes against positions processed, so saturation is visible.

    A class count is a coupon-collector quantity: it cannot be extrapolated
    linearly, and a registration that does so registers a criterion it may not
    be able to meet.
    """
    seen, points, ordered = set(), [], []
    for row in rows_of(path):
        if row["att_proved"] == "true":
            ordered.append((int(row["entry"]), tuple(row[c] for c in COLUMNS)))
    ordered.sort(key=lambda pair: pair[0])
    highest = max((entry for entry, _ in ordered), default=0) + 1
    for bound in range(step, highest + step, step):
        for entry, value in ordered:
            if entry < bound:
                seen.add(value)
        points.append((bound, len(seen)))
    return points


def main(path, positions=None):
    firings, keys, roots, classes, loss = tally(path)
    n = int(positions) if positions else None
    print(f"census_classes: {firings} firing(s)")
    print(f"  win-proving distinct KEYS    {len(keys)}   (D-537's literal unit, D-570)")
    print(f"  win-proving distinct ROOTS   {len(roots)}")
    print(f"  win-proving column CLASSES   {len(classes)}   (the alternative's own partition)")
    print(f"  loss-direction distinct keys {len(loss)}   [separate, never summed]")
    if n:
        print(f"  per position over n={n}: keys {len(keys)/n:.3f} "
              f"roots {len(roots)/n:.3f} classes {len(classes)/n:.3f}")
    print("  class curve (positions:classes) — read it for SATURATION, never extrapolate it:")
    print("    " + "  ".join(f"{p}:{v}" for p, v in curve(path)))


if __name__ == "__main__":
    main(*sys.argv[1:])
