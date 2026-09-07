"""Label moments per class, and the Stage-E census report they feed.

Separated from the walk because the walk's job is to visit the corpus once and
this file's job is to say what was seen: the two variance terms of the law of
total variance, the observation counts at both population units, and the growth
curve. Neither half needs to read the other's internals.
"""

import collections
import pathlib

import hexenum as E

# Replicates of the matched random-quotient referent: a single draw of a random
# partition is one sample of a noisy statistic, and a referent reported without
# a spread cannot say whether the enum beat it or beat one draw of it.
NULL_REPLICATES = 3  # overridable by census.py's --null-replicates


class Moments:
    """Per-key `n`, sum and sum of squares — enough for both variance terms."""

    def __init__(self):
        self.n = collections.Counter()
        self.total = collections.Counter()
        self.square = collections.Counter()

    def add(self, key, label):
        self.n[key] += 1
        self.total[key] += label
        self.square[key] += label * label

    def terms(self):
        """`(within, between, total, classes, n, eta2, omega2)`.

        The first three are the law of total variance's terms, per observation.
        `eta2` rises with the class count whatever the classes mean, so `omega2`
        is reported beside it: it subtracts the between-group sum of squares a
        partition of that many groups would earn from noise alone, which is the
        correction the random-quotient referent estimates empirically.
        """
        count = sum(self.n.values())
        if not count:
            return (0.0, 0.0, 0.0, 0, 0, 0.0, 0.0)
        grand = sum(self.total.values()) / count
        within = 0.0
        between = 0.0
        for key, n in self.n.items():
            mean = self.total[key] / n
            within += self.square[key] - n * mean * mean
            between += n * (mean - grand) ** 2
        classes = len(self.n)
        # THE TOTAL IS COMPUTED WITHOUT THE PARTITION, and that is the whole
        # point of computing it twice. Defining `total = within + between` makes
        # the law-of-total-variance check compare a number to itself, which is
        # the same empty check the enum memo deleted from its solver arm. This
        # form shares no term with the loop above, so a defect in either half of
        # the decomposition shows up as a disagreement.
        partition_free = sum(self.square.values()) - count * grand * grand
        if abs(within + between - partition_free) > 1e-6 * max(1.0, abs(partition_free)):
            raise ValueError(
                f"the two variance terms sum to {within + between} where the "
                f"partition-free total is {partition_free}")
        total = partition_free
        eta2 = between / total if total else 0.0
        omega2 = 0.0
        if count > classes and total:
            mean_within = within / (count - classes)
            omega2 = (between - (classes - 1) * mean_within) / (total + mean_within)
        return (within / count, between / count, total / count, classes, count, eta2, omega2)



def report(path, unit, kept, seed, scored_cells, legal_sample, legal_stride,
           window_moments, code_moments, window_null, code_null,
           window_raw, code_raw, window_count, code_count,
           window_join, window_join_null, curve):
    lines = []

    def say(text=""):
        lines.append(text)

    say("# hex threat enum — Stage E census")
    say(f"length: {unit.length}")
    say(f"quiet eval positions walked: {kept}")
    say(f"random-quotient referent seed: {seed}")
    say(f"scored cells, total: {scored_cells}")
    say(f"scored cells per position: {scored_cells / kept:.1f}" if kept else "")
    if legal_sample:
        ordered = sorted(legal_sample)
        say(f"rule-5 legal empty cells, sampled every {legal_stride}th position "
            f"(n={len(ordered)}): mean {sum(ordered) / len(ordered):.1f} "
            f"median {ordered[len(ordered) // 2]}")
    say("")
    say("## k and the code count")
    say("rung\tk\tcodes=C(k+2,3)")
    for rung in E.LADDER:
        say(f"{rung}\t{unit.k[rung]}\t{E.multiset_codes(unit.k[rung])}")
    say("")
    rows = []
    for rung in E.LADDER:
        rows.append(("window", f"enum {rung}", window_moments[rung]))
    rows.append(("window", "raw folded", window_raw))
    rows.append(("window", "COUNT-ONLY", window_count))
    for rung in E.LADDER:
        rows.append(("window", f"JOIN count x {rung}", window_join[rung]))
    for rung in E.LADDER:
        for r in range(NULL_REPLICATES):
            rows.append(("window", f"JOINNULL count x {rung} r{r}",
                         window_join_null[(rung, r)]))
    for rung in E.LADDER:
        for r in range(NULL_REPLICATES):
            rows.append(("window", f"null {rung} r{r}", window_null[(rung, r)]))
    for rung in E.LADDER:
        rows.append(("code", f"enum {rung}", code_moments[rung]))
    rows.append(("code", "raw folded", code_raw))
    rows.append(("code", "COUNT-ONLY", code_count))
    for rung in E.LADDER:
        for r in range(NULL_REPLICATES):
            rows.append(("code", f"null {rung} r{r}", code_null[(rung, r)]))
    say("## observations per parameter, at both population units")
    say("unit\tpartition\tdistinct\tobservations\tmean\tmedian\t<=4\t<20\t<75\tcover90")
    for name, partition, store in rows:
        if partition.startswith("null") or partition.startswith("JOINNULL"):
            continue
        counts = sorted(store.n.values(), reverse=True)
        if not counts:
            continue
        total = sum(counts)
        running = 0
        cover = 0
        for count in counts:
            running += count
            cover += 1
            if running >= 0.9 * total:
                break
        say(f"{name}\t{partition}\t{len(counts)}\t{total}\t{total / len(counts):.1f}\t"
            f"{counts[len(counts) // 2]}\t{sum(1 for c in counts if c <= 4)}\t"
            f"{sum(1 for c in counts if c < 20)}\t{sum(1 for c in counts if c < 75)}\t{cover}")
    say("")
    say("## purity: within / between / total label variance, eta^2 and omega^2")
    say("unit\tpartition\tclasses\tn\twithin\tbetween\ttotal\teta2\tomega2")
    for name, partition, store in rows:
        within, between, total, classes, count, eta2, omega2 = store.terms()
        say(f"{name}\t{partition}\t{classes}\t{count}\t{within:.4f}\t{between:.4f}\t"
            f"{total:.4f}\t{eta2:.6f}\t{omega2:.6f}")
    say("")
    say("## growth curve, distinct codes every 5000 positions")
    for rung in E.LADDER:
        say(f"{rung}: " + " ".join(f"{a}:{b}" for a, b in curve[rung]))

    body = "\n".join(lines) + "\n"
    pathlib.Path(path).write_text(body)
    print(body, flush=True)


