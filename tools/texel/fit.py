"""The quiet-term fit of the five handcrafted_v0 weights to search-score labels.

Operator ruling R6 (docs/decisions.md D-622) splits the table: `w4` and `w5`
name classes the search resolves inside its own window and are NOT fitted from
search labels, and the quiet entries are fitted on rows where neither side holds
an in-window forced win. On those rows the tactical regressors are identically
zero, so the fit is a three-parameter linear one with a closed form: no seed, no
learning rate and no stopping rule enters the answer.

The constraint set is handled by EXACT active-set enumeration with the bounds as
members. A projection onto the set is not the constrained optimum — a projected
point can sit on a face the true optimum never touches — and this module refuses
rather than projecting.
"""

import pathlib
import sys
import tomllib
from itertools import combinations

EVAL_MAX = 16000
QUIET_COUNTS = 3
COMMITTED_WEIGHTS = "configs/eval_v0_weights.toml"


class FitError(Exception):
    """A named refusal. Hard rule 3: no silent fallback and no projection."""


def committed_table(path=COMMITTED_WEIGHTS):
    """The committed five entries, read rather than duplicated in code."""
    with open(path, "rb") as handle:
        table = tomllib.load(handle)["table"]
    return [table[str(k)] for k in range(1, 6)]


def read_rows(path):
    """Rows as `extract.py` writes them: per-side counts, every score_kind."""
    rows = []
    with open(path) as handle:
        for line in handle:
            if line.startswith("#"):
                continue
            w = line.rstrip("\n").split("\t")
            rows.append({
                "a": [0] + [int(x) for x in w[0:6]],
                "b": [0] + [int(x) for x in w[6:12]],
                "label": int(w[12]),
                "to_move": w[13],
                "kind": w[14],
                "depth": int(w[15]),
                "book": w[16],
                "result": w[17],
                "key": w[18],
            })
    if not rows:
        raise FitError(f"fit: {path} holds no rows")
    return rows


def holds_forced_win(row):
    """Either side holds a live window of four or more (design §3 clause 2).

    A four-window has two empty cells and a turn places TWO stones (game rule
    3), so its owner completes it in one turn; a five-window completes in one
    stone. Both are decided by the search before any eval is consulted.
    """
    return any(row["a"][k] or row["b"][k] for k in (4, 5, 6))


def select(rows):
    """Design §3's three clauses, with the population of each reported."""
    counts = {"all": len(rows)}
    kept = [r for r in rows if r["kind"] == "eval"]
    counts["dropped_score_kind"] = len(rows) - len(kept)
    quiet = [r for r in kept if not holds_forced_win(r)]
    counts["dropped_forced_win"] = len(kept) - len(quiet)
    fitted = [r for r in quiet if abs(r["label"]) < EVAL_MAX]
    counts["dropped_saturated_label"] = len(quiet) - len(fitted)
    counts["fitted"] = len(fitted)
    if not fitted:
        raise FitError("fit: the row filter left no rows to fit")
    return fitted, counts


def signed(row):
    """Features oriented to the side to move, matching `score_sign`."""
    a, b = (row["a"], row["b"]) if row["to_move"] == "p1" else (row["b"], row["a"])
    return [a[k] - b[k] for k in range(1, 7)]


def normal_equations(rows, dim=QUIET_COUNTS):
    """Sum `g g^T` and `g y` over the fitted rows, REFUSING a dead regressor.

    D-621's condition is ONE-SIDEDNESS and not degeneracy: a regressor that never
    changes sign cannot report what such a window is worth to its OWNER, only
    what being on the wrong end of one costs. It leaves the normal matrix
    perfectly well conditioned, so a rank test does not see it -- measured, the
    two tactical regressors have diagonal entries of 132 953 and 4 692 over the
    eval rows while taking one sign in every one of them. The check is therefore
    on the SIGNS, which is the property the finding is about.
    """
    a = [[0.0] * dim for _ in range(dim)]
    b = [0.0] * dim
    for row in rows:
        g = signed(row)
        for i in range(dim):
            for j in range(dim):
                a[i][j] += g[i] * g[j]
            b[i] += g[i] * row["label"]
    signs = [set() for _ in range(dim)]
    for row in rows:
        g = signed(row)
        for i in range(dim):
            if g[i]:
                signs[i].add(g[i] > 0)
    for i in range(dim):
        if len(signs[i]) < 2:
            raise FitError(
                f"fit: regressor g{i + 1} takes {'no' if not signs[i] else 'ONE'} sign "
                f"on all {len(rows)} fitted rows, so it carries no evidence about what "
                "such a window is worth to its OWNER and must not be fitted (D-621)"
            )
    return a, b


def solve(m, rhs):
    """Gaussian elimination with IMPLICIT (scale-relative) partial pivoting.

    The KKT systems below mix the normal matrix, whose entries run to 1e7 on a
    real corpus, with constraint rows of O(1). An ABSOLUTE pivot threshold reads
    the small rows as singular, and whether it does so depends on the objective's
    units rather than on the problem: multiplying every label by a constant, which
    cannot move the minimiser, made an earlier version of this routine discard
    fourteen of fifteen active sets. Each row is therefore weighed against its own
    largest entry, so the test is invariant under scaling.

    Returns `None` only for a genuinely singular system, which for an active set
    means its constraints are linearly dependent — a legitimate skip, counted by
    the caller rather than swallowed.
    """
    n = len(rhs)
    aug = [list(row) + [rhs[i]] for i, row in enumerate(m)]
    scale = [max((abs(x) for x in row[:n]), default=0.0) for row in aug]
    if any(s == 0.0 for s in scale):
        return None
    for col in range(n):
        pivot = max(range(col, n), key=lambda r: abs(aug[r][col]) / scale[r])
        if abs(aug[pivot][col]) / scale[pivot] < 1e-12:
            return None
        aug[col], aug[pivot] = aug[pivot], aug[col]
        scale[col], scale[pivot] = scale[pivot], scale[col]
        for r in range(n):
            if r == col:
                continue
            factor = aug[r][col] / aug[col][col]
            for c in range(col, n + 1):
                aug[r][c] -= factor * aug[col][c]
    return [aug[i][n] / aug[i][i] for i in range(n)]


def objective(a, b, w):
    """(1/2) w^T A w - b^T w, the least-squares objective up to a constant."""
    n = len(w)
    quad = sum(w[i] * sum(a[i][j] * w[j] for j in range(n)) for i in range(n))
    return 0.5 * quad - sum(b[i] * w[i] for i in range(n))


def schema_constraints(dim, ceiling):
    """`c . w >= d` for the schema, with the BOUNDS as members not a clamp.

    weights.rs requires `w1 >= 1`, strict increase, and every entry strictly
    below the decided window's value. `ceiling` is the first entry above this
    block — the pinned `w4` for a quiet fit, or the decided window's value for a
    full one — and the last free entry must stay strictly under it.
    """
    out = [([1.0] + [0.0] * (dim - 1), 1.0)]
    for k in range(dim - 1):
        row = [0.0] * dim
        row[k], row[k + 1] = -1.0, 1.0
        out.append((row, 1.0))
    last = [0.0] * dim
    last[dim - 1] = -1.0
    out.append((last, -(float(ceiling) - 1.0)))
    return out


def constrained_min(a, b, constraints, tol=1e-7):
    """The EXACT minimiser of the objective over `c . w >= d`, by active sets.

    EVERY constraint is an active-set member, bounds included. Each subset gives
    an equality-constrained KKT system solved exactly; the feasible minimiser
    over all subsets is the constrained optimum, because the true optimum's own
    active set is among them and its solution is feasible. No step size, no
    tolerance on the answer, no seed.
    """
    n = len(b)
    best, best_value, skipped = None, None, []
    for size in range(min(n, len(constraints)) + 1):
        for active in combinations(range(len(constraints)), size):
            dim = n + len(active)
            m = [[0.0] * dim for _ in range(dim)]
            rhs = [0.0] * dim
            for i in range(n):
                m[i][:n] = list(a[i])
                rhs[i] = b[i]
            for t, index in enumerate(active):
                cvec, dval = constraints[index]
                for j in range(n):
                    m[j][n + t] = -cvec[j]
                    m[n + t][j] = cvec[j]
                rhs[n + t] = dval
            answer = solve(m, rhs)
            if answer is None:
                skipped.append(active)
                continue
            w = answer[:n]
            if any(sum(c[j] * w[j] for j in range(n)) < d - tol for c, d in constraints):
                continue
            value = objective(a, b, w)
            if best_value is None or value < best_value:
                best, best_value = w, value
    if best is None:
        raise FitError(
            f"fit: no feasible point over {len(constraints)} constraint(s); "
            f"{len(skipped)} active set(s) were singular"
        )
    return best, skipped


def round_to_schema(w, ceiling):
    """Integers the document can hold, REFUSING rather than clamping.

    THE INPUT IS CHECKED, NOT THE OUTPUT. An infeasible real-valued answer is
    refused here rather than rounded into feasibility, which is the projection
    this module exists without; and a feasible one cannot round out of the schema,
    so there is no clamp and no guard pretending to catch one. The remaining
    refusal is the ceiling, which on the registered path sits far above the quiet
    entries and is kept for a caller that pins a smaller one.
    """
    if w[0] < 1.0 - 1e-9 or any(w[k + 1] < w[k] + 1 - 1e-9 for k in range(len(w) - 1)):
        raise FitError(
            f"fit: the real-valued answer {['%.4f' % x for x in w]} is not schema-feasible; "
            "rounding an infeasible point is the projection this module exists without"
        )
    # NO CLAMP AND NO GUARD AGAINST ONE. Rounding to nearest is monotone and
    # carries a gap of at least one across, so a feasible real answer rounds to a
    # feasible integer one and the clamp `max(floor, ...)` this used to apply
    # could never have fired. A guard that cannot fire is the defect the ceiling
    # guard below is kept honest about, and the test pins the property instead.
    out = [int(round(value)) for value in w]
    if out[-1] >= ceiling:
        raise FitError(
            f"fit: rounded table {out} reaches the pinned ceiling {ceiling}; "
            "the schema needs a strict increase and this is not one"
        )
    return out


def dominance(table):
    """R6's registered check: each tactical class exceeds everything below it."""
    return table[3] > sum(table[:3]), table[4] > sum(table[:4])


def predict(g, table):
    raw = sum(g[k] * table[k] for k in range(5)) + g[5] * EVAL_MAX
    return max(-EVAL_MAX, min(EVAL_MAX, raw))


def saturates(g, table):
    raw = sum(g[k] * table[k] for k in range(5)) + g[5] * EVAL_MAX
    return abs(raw) >= EVAL_MAX


def mean_squared_error(rows, table):
    if not rows:
        return float("nan")
    return sum((predict(signed(r), table) - r["label"]) ** 2 for r in rows) / len(rows)


def ranks(values):
    """Competition-free ranks with TIES AVERAGED, so Spearman is unbiased."""
    order = sorted(range(len(values)), key=lambda i: values[i])
    out = [0.0] * len(values)
    start = 0
    while start < len(order):
        stop = start
        while stop + 1 < len(order) and values[order[stop + 1]] == values[order[start]]:
            stop += 1
        share = (start + stop) / 2.0
        for i in range(start, stop + 1):
            out[order[i]] = share
        start = stop + 1
    return out


def rank_correlation(rows, table):
    """Spearman with tied ranks averaged, deterministic in the row order given."""
    if len(rows) < 2:
        return float("nan")
    rp = ranks([predict(signed(r), table) for r in rows])
    ro = ranks([r["label"] for r in rows])
    n = len(rows)
    mp, mo = sum(rp) / n, sum(ro) / n
    num = sum((rp[i] - mp) * (ro[i] - mo) for i in range(n))
    dp = sum((rp[i] - mp) ** 2 for i in range(n))
    do = sum((ro[i] - mo) ** 2 for i in range(n))
    return num / (dp * do) ** 0.5 if dp and do else float("nan")


def split(rows):
    """Train/validation by the position's own key digest, a 1-in-8 slice.

    Content-derived, so the split is a function of the corpus and moves with
    neither the row order nor a seed. Per POSITION and not per game.
    """
    train = [r for r in rows if r["key"][-1] not in "01"]
    val = [r for r in rows if r["key"][-1] in "01"]
    return train, val


def tempo_normal_equations(rows, pin):
    """Normal equations for the model the ANSWER comes from.

    `label = c + w1 g1 + w2 g2 + pin * g3`, free in `(w1, w2, c)`.

    TWO THINGS ARE DELIBERATE HERE. `c` is a mover-relative tempo term the v0
    schema has no entry for: on the fitted population the mover is behind in
    window count and ahead in label, because it is about to place two stones,
    and a model without `c` can only absorb that by moving the weights. `pin`
    holds the top quiet entry at its committed value because the filtered rows
    carry no evidence at all about the quiet-to-tactical balance -- the tactical
    regressors are identically zero on every one of them.
    """
    a = [[0.0] * 3 for _ in range(3)]
    b = [0.0] * 3
    for row in rows:
        g = signed(row)
        x = [g[0], g[1], 1.0]
        target = row["label"] - g[2] * pin
        for i in range(3):
            for j in range(3):
                a[i][j] += x[i] * x[j]
            b[i] += x[i] * target
    return a, b


def tempo_constraints(pin):
    """The schema, on the two free weights, with the third held at `pin`."""
    return [([1.0, 0.0, 0.0], 1.0),
            ([-1.0, 1.0, 0.0], 1.0),
            ([0.0, -1.0, 0.0], -(float(pin) - 1.0))]


def fit(rows, committed):
    """The whole answer, and the contrast that says why it is that answer."""
    fitted, counts = select(rows)
    train, val = split(fitted)
    pin = float(committed[QUIET_COUNTS - 1])

    # The shipping model, kept only as the contrast: no intercept, free scale.
    plain_a, plain_b = normal_equations(train)
    plain = solve(plain_a, plain_b)

    answer, skipped = constrained_min(*tempo_normal_equations(train, pin),
                                      tempo_constraints(pin))
    quiet = [answer[0], answer[1], pin]
    table = round_to_schema(quiet, committed[QUIET_COUNTS]) + committed[QUIET_COUNTS:]
    return {
        "counts": counts, "train": train, "val": val,
        "no_intercept": plain, "quiet": quiet, "tempo": answer[2], "table": table,
        "singular_active_sets": skipped,
    }


def main(rows_path):
    committed = committed_table()
    rows = read_rows(rows_path)
    answer = fit(rows, committed)
    counts, train, val, table = (answer["counts"], answer["train"],
                                 answer["val"], answer["table"])

    print(f"filter: rows {counts['all']} -> fitted {counts['fitted']}"
          f"  dropped: score_kind {counts['dropped_score_kind']}"
          f"  forced_win {counts['dropped_forced_win']}"
          f"  saturated_label {counts['dropped_saturated_label']}")
    print(f"fit: train {len(train)} val {len(val)}")
    print(f"fit: no-intercept free-scale solve {['%.4f' % x for x in answer['no_intercept']]}"
          f"   (the CONTRAST, not the answer)")
    print(f"fit: tempo term c {answer['tempo']:+.2f}   (fitted, then discarded: the v0 "
          f"schema has no entry for it)")
    print(f"fit: constrained quiet optimum {['%.4f' % x for x in answer['quiet']]}")
    print(f"fit: committed {committed}")
    print(f"fit: fitted table {table}   (entries 4 and 5 pinned, R6/D-622)")
    gaps = [k + 1 for k in range(QUIET_COUNTS - 1)
            if answer["quiet"][k + 1] - answer["quiet"][k] < 1 + 1e-6]
    print(f"fit: gap constraints BINDING between quiet entries {gaps or 'none'}")
    print(f"fit: singular active sets skipped {len(answer['singular_active_sets'])} "
          f"of the enumeration")
    four, five = dominance(table)
    print(f"check: dominance w4>sum(w1..w3) {four}  w5>sum(w1..w4) {five}")
    for name, w in (("committed", committed), ("fitted", table)):
        sat = sum(1 for r in train + val if saturates(signed(r), w))
        print(f"diag: {name} train_mse {mean_squared_error(train, w):.1f} "
              f"val_mse {mean_squared_error(val, w):.1f} "
              f"val_spearman {rank_correlation(val, w):.4f} saturating_rows {sat}")
    print(f"diag: fitted ratios {['%.3f' % (table[i+1]/table[i]) for i in range(4)]}")
    print(f"diag: committed ratios {['%.3f' % (committed[i+1]/committed[i]) for i in range(4)]}")
    for depth in sorted({r["depth"] for r in val}):
        slice_ = [r for r in val if r["depth"] == depth]
        print(f"diag: depth {depth} n {len(slice_)} "
              f"committed_val_mse {mean_squared_error(slice_, committed):.1f} "
              f"fitted_val_mse {mean_squared_error(slice_, table):.1f}")


if __name__ == "__main__":
    main(sys.argv[1])
