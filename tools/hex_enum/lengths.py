"""Everything derived from one window length, built once per census run.

Split from `census.py` because the walk's job is to visit the corpus and this
file's job is to say, for one length, what a code means: its class at every
ladder rung, its folded twin, its stone-count referent, and the two nulls those
are measured against.
"""

import array
import collections
import random

import hexenum as E
import report as R


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
                             for r in range(R.NULL_REPLICATES)]
                      for rung in E.LADDER}
        self.counts = array.array("i", (self.count_key(code) for code in range(self.size)))
        self.join_nulls = {rung: [self.permuted_within_counts(rung, seed + length + 7919 * r)
                                  for r in range(R.NULL_REPLICATES)]
                           for rung in E.LADDER}

    def permuted_within_counts(self, rung, seed):
        """The referent for the NESTED test, matched on the join's own cell count.

        `permuted` shuffles the class table over the WHOLE code space, so
        `(count, permuted class)` has about four times the cells of
        `(count, class)` — measured, 194 against 49 at L=7 T4 and 5 267 against
        1 323 at L=11 T4. A finer partition of this space scores higher whatever
        it means, so that null carries the very advantage the nested test exists
        to remove, on the null's side, where it manufactures false negatives.

        Permuting WITHIN each count stratum instead preserves the multiset of
        classes inside every stratum, so the join's cell count and its per-cell
        code multiplicities are exactly the real join's and the only thing that
        changes is which codes share a class within a count.
        """
        strata = collections.defaultdict(list)
        for code in range(self.size):
            strata[self.counts[code]].append(code)
        table = self.classes[rung]
        out = array.array("i", [0]) * self.size
        rng = random.Random(seed)
        for key in sorted(strata):
            codes = strata[key]
            order = list(codes)
            rng.shuffle(order)
            for code, donor in zip(codes, order):
                out[code] = table[donor]
        return out

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
