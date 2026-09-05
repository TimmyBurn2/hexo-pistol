"""Texel-style fit of the five handcrafted_v0 weights to search-score labels.

The model is LINEAR in the five weights (docs/experiments/wp22_phase1_premise.md
§2), so the squared-error fit has a closed form and no seed, no learning rate
and no stopping rule enter the answer. Where the constraint set binds, a
deterministic projected descent finishes from the committed weights.
"""

import hashlib
import sys

EVAL_MAX = 16000
COMMITTED = [2, 12, 60, 300, 1500]


def read_rows(path):
    rows = []
    for line in open(path):
        if line.startswith("#"):
            continue
        w = line.rstrip("\n").split("\t")
        f = [int(w[i]) for i in range(6)]
        rows.append((f, int(w[6]), w[7], int(w[8]), w[9], w[10], w[11]))
    return rows


def split(rows):
    """Train/validation split by the position's own key digest.

    Content-derived, so the split is a function of the corpus and moves with
    neither the row order nor a seed. The last hex digit gives a 1-in-16 slice.
    """
    train = [r for r in rows if r[6][-1] not in "01"]
    val = [r for r in rows if r[6][-1] in "01"]
    return train, val


def predict(f, w):
    raw = sum(f[k] * w[k] for k in range(5)) + f[5] * EVAL_MAX
    return max(-EVAL_MAX, min(EVAL_MAX, raw))


def signed(f, to_move):
    """Features oriented to the side to move, matching `score_sign`."""
    return f if to_move == "p1" else [-x for x in f]


def solve(a, b):
    """Gaussian elimination with partial pivoting on a small dense system."""
    n = len(b)
    m = [list(row) + [b[i]] for i, row in enumerate(a)]
    for col in range(n):
        pivot = max(range(col, n), key=lambda r: abs(m[r][col]))
        if abs(m[pivot][col]) < 1e-12:
            return None
        m[col], m[pivot] = m[pivot], m[col]
        for r in range(n):
            if r == col:
                continue
            factor = m[r][col] / m[col][col]
            for c in range(col, n + 1):
                m[r][c] -= factor * m[col][c]
    return [m[i][n] / m[i][i] for i in range(n)]


def normal_equations(rows):
    """Sum f f^T and f y over rows the clamp does not touch.

    A saturated row carries no gradient in w — its prediction is the band edge
    whatever the weights do — so including it would fit the clamp rather than
    the position.
    """
    a = [[0.0] * 5 for _ in range(5)]
    b = [0.0] * 5
    used = 0
    for f, label, to_move, _depth, _book, _result, _key in rows:
        if abs(label) >= EVAL_MAX:
            continue
        g = signed(f, to_move)
        target = label - g[5] * EVAL_MAX
        for i in range(5):
            for j in range(5):
                a[i][j] += g[i] * g[j]
            b[i] += g[i] * target
        used += 1
    return a, b, used


def objective(a, b, w):
    """(1/2) w^T A w - b^T w, the least-squares objective up to a constant."""
    quad = sum(w[i] * sum(a[i][j] * w[j] for j in range(5)) for i in range(5))
    return 0.5 * quad - sum(b[i] * w[i] for i in range(5))


def constrained_fit(a, b):
    """The EXACT minimiser over the schema's constraint set, by active sets.

    weights.rs:141-177 requires 1 <= w1, strict increase, and every entry below
    EVAL_MAX. PROJECTING the unconstrained solution onto that set is NOT the
    constrained optimum and is the defect this replaces: the projected point
    can sit on a face the true optimum never touches.

    With five weights there are four gap constraints w[k+1] >= w[k] + 1, so
    there are sixteen candidate active sets. Each one ties a block of weights
    to a single free base, which is a reduced least-squares problem solved
    exactly. Enumerating all sixteen and keeping the feasible minimiser is
    exhaustive rather than iterative: no step size, no tolerance, no seed.
    """
    best, best_value = None, None
    for pattern in range(16):
        # blocks[i] = index of the free base this weight is tied to.
        base, blocks, offset = 0, [], []
        for k in range(5):
            if k > 0 and not (pattern >> (k - 1)) & 1:
                base += 1
            blocks.append(base)
            offset.append(k)
        free = base + 1
        # w[k] = z[blocks[k]] + (offset[k] - offset[first of block])
        first = {}
        for k in range(5):
            first.setdefault(blocks[k], k)
        shift = [offset[k] - offset[first[blocks[k]]] for k in range(5)]

        ra = [[0.0] * free for _ in range(free)]
        rb = [0.0] * free
        for i in range(5):
            for j in range(5):
                ra[blocks[i]][blocks[j]] += a[i][j]
                rb[blocks[i]] -= a[i][j] * shift[j]
            rb[blocks[i]] += b[i]
        z = solve(ra, rb)
        if z is None:
            continue
        w = [z[blocks[k]] + shift[k] for k in range(5)]
        w = [max(1.0, min(float(EVAL_MAX - 1), x)) for x in w]
        if any(w[k + 1] < w[k] + 1 - 1e-9 for k in range(4)):
            continue
        value = objective(a, b, w)
        if best_value is None or value < best_value:
            best, best_value = w, value
    return best


def round_to_schema(w):
    """Integers the document can hold, keeping the strict increase."""
    out, floor = [], 1
    for value in w:
        v = max(floor, int(round(value)))
        out.append(min(v, EVAL_MAX - 1))
        floor = out[-1] + 1
    return out


def loss(rows, w):
    total, n = 0.0, 0
    for f, label, to_move, _d, _b, _r, _k in rows:
        g = signed(f, to_move)
        total += (predict(g, w) - label) ** 2
        n += 1
    return total / n if n else float("nan")


def rank_correlation(rows, w):
    """Spearman over a capped sample, deterministic in the row order given."""
    sample = rows[:20000]
    pred = sorted(range(len(sample)), key=lambda i: predict(signed(sample[i][0], sample[i][2]), w))
    obs = sorted(range(len(sample)), key=lambda i: sample[i][1])
    rp, ro = [0] * len(sample), [0] * len(sample)
    for rank, i in enumerate(pred):
        rp[i] = rank
    for rank, i in enumerate(obs):
        ro[i] = rank
    n = len(sample)
    if n < 2:
        return float("nan")
    d2 = sum((rp[i] - ro[i]) ** 2 for i in range(n))
    return 1 - 6 * d2 / (n * (n * n - 1))


def main(rows_path):
    rows = read_rows(rows_path)
    train, val = split(rows)
    a, b, used = normal_equations(train)
    raw = solve(a, b)
    constrained = constrained_fit(a, b)
    fitted = round_to_schema(constrained) if constrained else list(COMMITTED)

    print(f"fit: rows {len(rows)} train {len(train)} val {len(val)} unclamped_train {used}")
    print(f"fit: unconstrained solution {['%.4f' % x for x in raw] if raw else 'SINGULAR'}")
    print(f"fit: constrained optimum   {['%.4f' % x for x in constrained]}")
    print(f"fit: rounded to schema {fitted}")
    binding = [k + 1 for k in range(4) if constrained[k + 1] - constrained[k] < 1 + 1e-6]
    print(f"fit: gap constraints BINDING between entries {binding or 'none'}")
    print(f"fit: committed {COMMITTED}")
    for name, w in (("committed", COMMITTED), ("fitted", fitted)):
        print(f"diag: {name} train_mse {loss(train, w):.1f} val_mse {loss(val, w):.1f} "
              f"val_spearman {rank_correlation(val, w):.4f}")
    ratios = [fitted[i + 1] / fitted[i] for i in range(4)]
    print(f"diag: fitted ratios {['%.3f' % r for r in ratios]}")
    print(f"diag: committed ratios {['%.3f' % (COMMITTED[i+1]/COMMITTED[i]) for i in range(4)]}")


if __name__ == "__main__":
    main(sys.argv[1])
