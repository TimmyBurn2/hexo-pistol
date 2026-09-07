"""The Stage-E census: what the hex threat enum costs and what it separates.

Walks the R6 quiet population of the deduped corpus and reports, for ONE window
length and every ladder rung: distinct codes observed, observations per code,
the counts under Buro's three published floors, and CLASS PURITY as the two
terms of the law of total variance (docs/experiments/hex_threat_enum_v1.md
§6.5). THREE referents sit beside every purity number, because a purity number
alone passes vacuously: the un-quotiented raw folded code (a CEILING, and an
identity rather than a test); a random quotient of the enum's own shape at a
recorded seed (a FLOOR — it says only that the enum is not noise); and the
STONE-COUNT quotient, which is the one that can fail, because it is the
strongest partition that knows nothing the calculus names.

One length per run, because the per-cell work is linear in the rungs and the
lengths are independent — five processes finish in the time one does.

Nothing here selects anything and no number here gates anything (D-614).

Usage: census.py <out.txt> <L> [--limit N] [--seed N] [--legal-stride N]
                 [--manifest PATH] [--tranche TEMPLATE]

`--manifest` and `--tranche` exist so a test can drive this shipped script over
a synthetic corpus it wrote itself: the labelled corpus lives outside the
repository (D-636) and a gate that needed it would be green on one workstation
and red on every clone.
"""

import array
import collections
import pathlib
import random
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "texel"))
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import features as F
import hexenum as E
from report import NULL_REPLICATES, Moments, report

MANIFEST = "artifacts/arc3r_sweep_deduped_manifest.txt"
TRANCHE = "/home/tom/Projects/pistol-corpus/arc3r-sweep/tranche-{}/corpus.txt"
MOVES, TO_MOVE, SCORE_KIND, SCORE_VALUE = 2, 6, 7, 8
CURVE_STEP = 5000
RADIUS = 8


def quiet(stones):
    """R6's predicate (D-622): no 6-window holds 4+ of one side and none of the other."""
    a, b = F.per_side_counts(stones)
    return not any(a[k] or b[k] for k in (4, 5, 6))


def lines_of(stones, to_move):
    """`{(axis, line_id): {position: side}}`, mover-relative — 1 own, 2 opponent."""
    out = collections.defaultdict(dict)
    for (q, r), player in stones:
        side = 1 if (player == 0) == (to_move == "p1") else 2
        out[(0, q)][r] = side
        out[(1, r)][q] = side
        out[(2, q + r)][q] = side
    return out


def cell_of(axis, line_id, position):
    """Invert `lines_of`'s projection back to axial `(q, r)`."""
    if axis == 0:
        return (line_id, position)
    if axis == 1:
        return (position, line_id)
    return (position, line_id - position)


def legal_empty_cells(stones):
    """Rule 5's legal region minus the stones: the alternative population (§6.3)."""
    ball = set()
    for (q, r), _player in stones:
        for dq in range(-RADIUS, RADIUS + 1):
            for dr in range(max(-RADIUS, -dq - RADIUS), min(RADIUS, -dq + RADIUS) + 1):
                ball.add((q + dq, r + dr))
    return len(ball) - len(stones)


class Length:
    """Everything derived from one window length, built once per run."""

    def __init__(self, length, seed):
        self.length = length
        self.centres = E.centres(length)
        self.slots = [i for i in range(length) if i not in self.centres]
        self.powers = [3 ** j for j in range(len(self.slots))]
        self.size = 3 ** len(self.slots)
        self.classes = {}
        self.k = {}
        for rung in E.LADDER:
            codes, keys = E.table(length, rung)
            self.classes[rung] = codes
            self.k[rung] = len(keys)
        self.reverse = [E.reverse_code(length, code) for code in range(self.size)]
        self.folded = [min(code, self.reverse[code]) for code in range(self.size)]
        self.empty = {rung: self.classes[rung][0] for rung in E.LADDER}
        self.nulls = {rung: [self.permuted(rung, seed + length + 1000 * r)
                             for r in range(NULL_REPLICATES)]
                      for rung in E.LADDER}
        self.counts = array.array("i", (self.count_key(code) for code in range(self.size)))

    def count_key(self, code):
        """`(own stones, opp stones)` in the pattern, packed — the STRUCTURE-FREE baseline.

        The strongest partition that knows nothing the calculus names: no window,
        no openness, no completion cost, no rule-4 relevance, no position along
        the line. It is what the enum has to beat for "threat structure carries
        value" to mean anything, and a matched random referent cannot stand in
        for it — a random partition of the same shape is weaker than stone
        counting, so clearing the random one says only that the enum is not
        noise.
        """
        cell = [0] * self.length
        rest = code
        for index in self.slots:
            cell[index] = rest % 3
            rest //= 3
        return cell.count(1) * (self.length + 1) + cell.count(2)

    def permuted(self, rung, seed):
        """A random partition with the enum's own SHAPE — the matched referent.

        The classes are re-assigned by permuting which codes belong to which
        class, so the number of classes and the number of CODES in each class
        are exactly the enum's and the only thing that changes is which codes
        share a class. A referent drawn as `randrange(k)` instead would differ
        from the enum in class count and class shape as well, and a comparison
        against it could not tell those apart from the grouping
        (docs/process.md, "Criterion and defect class").
        """
        order = list(range(self.size))
        random.Random(seed).shuffle(order)
        table = self.classes[rung]
        return array.array("i", (table[order[code]] for code in range(self.size)))

    def codes_along(self, occupied):
        """`[(position, code)]` for every empty centre whose span holds a stone."""
        span = self.length - 1
        low = min(occupied) - span
        cells = [occupied.get(p, 0) for p in range(low, max(occupied) + span + 1)]
        out = []
        for start in range(len(cells) - self.length + 1):
            window = cells[start:start + self.length]
            if not any(window):
                continue
            if any(window[c] for c in self.centres):
                continue
            code = 0
            for power, index in zip(self.powers, self.slots):
                code += window[index] * power
            out.append((low + start + self.centres[0], code))
        return out

    def fold_is_class_invariant(self):
        """Every class equals its reversal's class — §5.4, checked not assumed."""
        for rung in E.LADDER:
            table = self.classes[rung]
            for code in range(self.size):
                if table[code] != table[self.reverse[code]]:
                    return False
        return True


def main(argv):
    args = list(argv)
    out_path = args.pop(0)
    length = int(args.pop(0))
    limit = None
    seed = 20260907
    legal_stride = 500
    manifest = MANIFEST
    tranche = TRANCHE
    while args:
        token = args.pop(0)
        if token == "--limit":
            limit = int(args.pop(0))
        elif token == "--seed":
            seed = int(args.pop(0))
        elif token == "--legal-stride":
            legal_stride = int(args.pop(0))
        elif token == "--manifest":
            manifest = args.pop(0)
        elif token == "--tranche":
            tranche = args.pop(0)
        else:
            raise SystemExit(f"census: unknown argument {token!r}")
    if "{}" not in tranche:
        raise SystemExit(f"census: --tranche {tranche!r} has no corpus-index placeholder")

    unit = Length(length, seed)
    if not unit.fold_is_class_invariant():
        raise SystemExit(f"census: L{length} classes are not reversal invariant")
    print(f"L{length} " + " ".join(f"{r} k={unit.k[r]}" for r in E.LADDER), flush=True)

    window_moments = {rung: Moments() for rung in E.LADDER}
    code_moments = {rung: Moments() for rung in E.LADDER}
    window_null = {(rung, r): Moments() for rung in E.LADDER
                   for r in range(NULL_REPLICATES)}
    code_null = {(rung, r): Moments() for rung in E.LADDER
                 for r in range(NULL_REPLICATES)}
    window_raw = Moments()
    code_raw = Moments()
    window_count = Moments()
    code_count = Moments()
    curve = collections.defaultdict(list)
    scored_cells = 0
    legal_sample = []

    wanted = {}
    with open(manifest) as handle:
        for line in handle:
            if line.startswith("#"):
                continue
            row = line.rstrip("\n").split("\t")
            wanted.setdefault(int(row[0]), set()).add((int(row[1]), row[4]))

    kept = 0
    for index in sorted(wanted):
        with open(tranche.format(index)) as handle:
            body = [l.rstrip("\n").split("\t") for l in handle if not l.startswith("#")]
        for record_number, _key in sorted(wanted[index]):
            record = body[record_number - 1]
            if record[SCORE_KIND] != "eval":
                continue
            stones = F.stones_of(record[MOVES])
            if not quiet(stones):
                continue
            kept += 1
            label = int(record[SCORE_VALUE])
            if kept % legal_stride == 0:
                legal_sample.append(legal_empty_cells(stones))
            per_cell = collections.defaultdict(dict)
            for (axis, line_id), occupied in lines_of(stones, record[TO_MOVE]).items():
                for position, code in unit.codes_along(occupied):
                    per_cell[cell_of(axis, line_id, position)][axis] = code
                    window_raw.add(unit.folded[code], label)
                    window_count.add(unit.counts[code], label)
                    for rung in E.LADDER:
                        window_moments[rung].add(unit.classes[rung][code], label)
                        for r, null in enumerate(unit.nulls[rung]):
                            window_null[(rung, r)].add(null[code], label)
            scored_cells += len(per_cell)
            for axes in per_cell.values():
                present = [axes.get(a) for a in (0, 1, 2)]
                for rung in E.LADDER:
                    table = unit.classes[rung]
                    empty = unit.empty[rung]
                    code_moments[rung].add(
                        tuple(sorted(empty if c is None else table[c] for c in present)), label)
                    for r, null in enumerate(unit.nulls[rung]):
                        code_null[(rung, r)].add(
                            tuple(sorted(null[0] if c is None else null[c]
                                         for c in present)), label)
                code_raw.add(tuple(sorted(0 if c is None else unit.folded[c]
                                          for c in present)), label)
                code_count.add(tuple(sorted(unit.counts[0] if c is None else unit.counts[c]
                                            for c in present)), label)
            if kept % CURVE_STEP == 0:
                for rung in E.LADDER:
                    curve[rung].append((kept, len(code_moments[rung].n)))
                print(f"  {kept} positions", flush=True)
            if limit and kept >= limit:
                break
        if limit and kept >= limit:
            break

    report(out_path, unit, kept, seed, scored_cells, legal_sample, legal_stride,
           window_moments, code_moments, window_null, code_null,
           window_raw, code_raw, window_count, code_count, curve)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
