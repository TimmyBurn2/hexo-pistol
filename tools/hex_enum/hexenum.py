"""The hex threat enum: classes of single-axis line patterns, computed.

The definition is `docs/experiments/hex_threat_enum_v1.md` §5 and nothing here
adds to it. A class is an equivalence class of length-L patterns under a
property tuple whose every component is a count or a minimum over the set of
6-windows through one empty cell, so the class count `k` is an OUTPUT of
enumeration and no threshold, name or number is imported (docs/decisions.md
D-706).

Encoding: a pattern is a tuple of L values, 0 empty, 1 own, 2 opponent, read
mover-relative. The centre cell is empty by construction and is not encoded, so
a pattern's CODE is the base-3 value of its L-1 non-centre positions.

Usage: hexenum.py <L> [<L> ...]   prints k and the ladder for each length.
"""

import sys

WIN_LEN = 6
INF = 99
LADDER = ("T4", "T3", "T2", "T1")


def centres(length):
    """The centre index of a length-L pattern, as a one-tuple.

    # Errors

    An EVEN length is refused by name. A class indexed by a CELL needs a cell in
    the middle and an even length has none; the two alignments a reader might
    take instead are two different sets of board cells, and two faithful
    readings of that convention were MEASURED to give different enums at every
    even length (10 against 33 at 6, 43 against 177 at 8, 182 against 920 at
    10). An under-specified object is refused rather than given a convention
    (CLAUDE.md rule 3).
    """
    if length % 2 == 0:
        raise ValueError(
            f"length {length} is even and a cell-centred class has no centre there; "
            "the enum is defined at odd lengths only")
    return (length // 2,)


def windows(length, centre):
    """Start indices of the 6-windows inside [0, length) that hold `centre`."""
    return [s for s in range(length - WIN_LEN + 1) if s <= centre < s + WIN_LEN]


def side_terms(pattern, starts, side):
    """`(min, open, r4)` for `side` — memo §5, computed over the window set.

    `min` counts the empty centre cell, which is one of the stones the side
    still owes: LAW-SUPPORT's "6 - own" read per stone. `INF` when no window
    through the centre is open for the side (LAW-HIT: a hit window is dead
    permanently).
    """
    other = 2 if side == 1 else 1
    best = None
    live = 0
    for start in starts:
        segment = pattern[start:start + WIN_LEN]
        if other in segment:
            continue
        live += 1
        cost = WIN_LEN - segment.count(side)
        if best is None or cost < best:
            best = cost
    least = INF if best is None else best
    return (least, live, least == 1)


def tuple_of(pattern, length):
    """The class key of one pattern: §5's 6-tuple, or a pair of them if even."""
    keys = []
    for centre in centres(length):
        starts = windows(length, centre)
        keys.append(side_terms(pattern, starts, 1) + side_terms(pattern, starts, 2))
    return tuple(sorted(keys))


def clip(least):
    """Cost at LAW-SUPPORT's own two boundaries, with DEAD kept apart from FAR.

    LAW-SUPPORT reads `own >= 6 - 2k`, so `k = 1` is `cost <= 2` and `k = 2` is
    `cost <= 4`; expressing both needs 1, 2, 3 and 4 apart and everything above
    them together. Clipping at `min(cost, 4)` instead merges cost 4 — a
    LAW-SUPPORT k=2 candidate — with 5 and 6, and cannot state the boundary it
    cites. `INF` stays apart because DEF-WINDOW's dead is not an expensive live
    window.
    """
    return INF if least == INF else (least if least <= 4 else 5)


def project(key, rung):
    """A coarser equivalence, as a function of the full tuple — memo §6.4."""
    if rung == "T4":
        return key
    out = []
    for half in key:
        own_min, own_open, _, opp_min, opp_open, _ = half
        if rung == "T3":
            out.append((own_min, min(own_open, 2), opp_min, min(opp_open, 2)))
        elif rung == "T2":
            # No alive-or-dead boolean: `open_X == 0` iff `min_X` is INF, so a
            # boolean here refines nothing (MEASURED, k = 47 either way) and
            # naming it would claim a distinction the rung does not carry.
            out.append((own_min, opp_min))
        elif rung == "T1":
            out.append((clip(own_min), clip(opp_min)))
        else:
            raise ValueError(f"no ladder rung named {rung!r}")
    return tuple(sorted(out))


def patterns(length):
    """Every length-L pattern with the centre cell empty, in code order.

    The code is the base-3 value of the non-centre positions, least significant
    first, so `code` and the pattern are one-to-one and the enumeration is
    deterministic without a sort.
    """
    slots = [i for i in range(length) if i not in centres(length)]
    total = 3 ** len(slots)
    for code in range(total):
        cell = [0] * length
        rest = code
        for index in slots:
            cell[index] = rest % 3
            rest //= 3
        yield code, tuple(cell)


def table(length, rung="T4"):
    """`(codes, keys)`: a code-indexed class id list and the key of each class."""
    ids = {}
    keys = []
    codes = []
    for _code, pattern in patterns(length):
        key = project(tuple_of(pattern, length), rung)
        found = ids.get(key)
        if found is None:
            found = len(keys)
            ids[key] = found
            keys.append(key)
        codes.append(found)
    return codes, keys


def multiset_codes(k):
    """`C(k+2, 3)`, the size-3 multisets over `k` classes — the row's parameters."""
    return (k + 2) * (k + 1) * k // 6


def reverse_code(length, code):
    """The code of the pattern reversed about its centre."""
    slots = [i for i in range(length) if i not in centres(length)]
    cell = [0] * length
    rest = code
    for index in slots:
        cell[index] = rest % 3
        rest //= 3
    cell.reverse()
    out = 0
    for index in reversed(slots):
        out = out * 3 + cell[index]
    return out


def refinement(length, step=2, rung="T4"):
    """How the length-L partition and the length-(L+step) one relate.

    Returns `(k_short, k_long, joint)`. `joint == k_short` says the longer
    partition is a FUNCTION of the shorter one — the extra cells changed no
    class, which is what THM-WINDOW's owed enumeration asks for. `joint ==
    k_long` says the shorter one is a function of the longer. Neither means the
    two partitions are not comparable at all, which is the interesting answer
    below the covering length: a short window asks a different question rather
    than a coarser one.
    """
    long_length = length + step
    offset = (long_length - length) // 2
    short = table(length, rung)[0]
    long_codes = table(long_length, rung)[0]
    short_slots = [i for i in range(length) if i not in centres(length)]
    short_powers = [3 ** j for j in range(len(short_slots))]
    pairs = set()
    shorts = set()
    longs = set()
    for code, pattern in patterns(long_length):
        core = pattern[offset:offset + length]
        core_code = 0
        for power, index in zip(short_powers, short_slots):
            core_code += core[index] * power
        pair = (short[core_code], long_codes[code])
        pairs.add(pair)
        shorts.add(pair[0])
        longs.add(pair[1])
    return (len(shorts), len(longs), len(pairs))


def main(argv):
    if argv and argv[0] == "--refine":
        for text in argv[1:]:
            length = int(text)
            for rung in LADDER:
                k_short, k_long, joint = refinement(length, 2, rung)
                verdict = ("the longer partition is a function of the shorter"
                           if joint == k_short else
                           "the shorter is a function of the longer"
                           if joint == k_long else "neither refines the other")
                print(f"L {length} -> {length + 2}  {rung}  k {k_short} -> {k_long}  "
                      f"joint {joint}  {verdict}", flush=True)
        return 0
    for text in argv:
        length = int(text)
        line = [f"L {length}"]
        for rung in LADDER:
            codes, keys = table(length, rung)
            line.append(f"{rung} k {len(keys)} codes {multiset_codes(len(keys))}")
        print("  ".join(line), flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
