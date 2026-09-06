"""The quiet-term fit of the five handcrafted_v0 weights to search-score labels.

Operator ruling R6 (docs/decisions.md D-622) splits the table: `w4` and `w5`
name classes the search resolves inside its own window and are NOT fitted from
search labels, and the quiet entries are fitted on the rows where neither
tactical regressor is non-zero. D-626 measured that this predicate removes
nothing FORCED -- every dropped row's threats die to the two stones the turn
provides -- and kept it on the ground that does hold: both tactical regressors
take ONE SIGN, so neither can report what such a window is worth to its owner.
On the kept rows they are identically zero, so the fit is a three-parameter
linear one with a closed form: no seed, no learning rate, no stopping rule.

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
        for number, line in enumerate(handle, start=1):
            if line.startswith("#"):
                continue
            w = line.rstrip("\n").split("\t")
            if w[13] not in ("p1", "p2"):
                raise FitError(
                    f"fit: {path} line {number} has to_move {w[13]!r}; the only tokens "
                    "the corpus schema defines are p1 and p2, and reading anything else "
                    "as p2 silently negates the row's features"
                )
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


def has_one_sided_tactical_window(row):
    """Either side holds a live window of four or more (design §3 clause 2).

    NOT "holds a forced win", which is what this was called and is measurably
    false of it: D-626 found the mover owns such a window in 0 of the 29 401
    rows this drops, and that every one of them dies to the two stones the turn
    provides. What it removes is the rows on which `g4` or `g5` is non-zero --
    regressors that take one sign across the whole corpus and so could only ever
    be fitted backwards.
    """
    return any(row["a"][k] or row["b"][k] for k in (4, 5, 6))


def select(rows):
    """Design §3's three clauses, with the population of each reported."""
    counts = {"all": len(rows)}
    kept = [r for r in rows if r["kind"] == "eval"]
    counts["dropped_score_kind"] = len(rows) - len(kept)
    quiet = [r for r in kept if not has_one_sided_tactical_window(r)]
    counts["dropped_one_sided_window"] = len(kept) - len(quiet)
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
    """Gaussian elimination with partial pivoting on a small dense system.

    THE PIVOT THRESHOLD IS ABSOLUTE AND SMALL, AND THAT IS THE WHOLE MECHANISM.
    An earlier revision carried a row EQUILIBRATION here and a comment claiming
    it "is what makes the pivot test invariant". Measured, on the KKT systems
    `constrained_min` actually builds, it does the opposite: dividing a normal
    row by its own largest entry drives that row's O(1) constraint coefficients
    down to O(1e-10), so the smallest column maximum goes from 1.0 WITHOUT
    equilibration to 1.4e-10 WITH it. The routine tolerated label scaling only
    because 1e-12 sits far below either. The mechanism claim was false, the code
    implementing it was worse than nothing, and both are gone rather than
    refined.

    What is true is a MEASURED range, pinned by `test_texel.py`: the minimiser is
    unchanged when the objective is scaled by up to 1e9, and beyond about 1e11
    the constraint rows are cancelled away in double precision and the routine
    REFUSES rather than answering.

    Returns `None` for a singular system, which for an active set means its
    constraints are linearly dependent — a legitimate skip, counted by the
    caller rather than swallowed.
    """
    n = len(rhs)
    aug = [list(row) + [rhs[i]] for i, row in enumerate(m)]
    for col in range(n):
        pivot = max(range(col, n), key=lambda r: abs(aug[r][col]))
        if abs(aug[pivot][col]) < 1e-12:
            return None
        aug[col], aug[pivot] = aug[pivot], aug[col]
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

    BOTH THE INPUT AND THE OUTPUT ARE CHECKED. An infeasible real-valued answer
    is refused rather than rounded into feasibility, which is the projection this
    module exists without. And a feasible one CAN round out of the schema —
    Python rounds half to even, so 1.5 and 2.5 both become 2 — which an earlier
    revision asserted was impossible; the output loop below is the guard for it.
    The third refusal is the ceiling, which on the registered path sits far above
    the quiet entries and is kept for a caller that pins a smaller one.
    """
    if w[0] < 1.0 - 1e-9 or any(w[k + 1] < w[k] + 1 - 1e-9 for k in range(len(w) - 1)):
        raise FitError(
            f"fit: the real-valued answer {['%.4f' % x for x in w]} is not schema-feasible; "
            "rounding an infeasible point is the projection this module exists without"
        )
    # THE OUTPUT IS CHECKED, because the theorem that made this unnecessary is
    # FALSE. Python rounds half to EVEN, so a feasible pair exactly one apart on
    # a half-integer -- 1.5 and 2.5 -- rounds to 2 and 2 and comes out flat where
    # the schema needs a strict increase. An earlier revision asserted the
    # opposite in a design document and pinned it with a draw that could not
    # produce a .5 tie: a guard that cannot fire standing in for a property that
    # does not hold.
    out = [int(round(value)) for value in w]
    for index in range(len(out) - 1):
        if out[index + 1] <= out[index]:
            raise FitError(
                f"fit: the feasible answer {['%.4f' % x for x in w]} rounds to {out}, "
                f"flat at entries {index + 1} and {index + 2}; rounding to nearest is "
                "half-to-even and does not preserve a unit gap"
            )
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


def tempo_normal_equations(rows, top, total):
    """Normal equations for the model the ANSWER comes from.

    `label = c + w1 g1 + (total - top - w1) g2 + top g3`, free in `(w1, c)`.

    THREE THINGS ARE DELIBERATE. `c` is a mover-relative tempo term the v0 schema
    has no entry for: on the fitted population the mover is behind in window
    count and ahead in label, and a model without `c` absorbs that by moving the
    weights. `top` holds the third quiet entry at its committed value and
    `total` holds the quiet SUM at its committed value, because the filtered rows
    carry no evidence about ANY quiet-to-tactical exchange rate -- the tactical
    regressors are identically zero on every one of them -- and each of those two
    pins holds one such rate where it already was. What is left is the single
    degree of freedom the corpus can speak to: how the committed quiet total
    divides between the one- and two-stone entries.
    """
    a = [[0.0] * 2 for _ in range(2)]
    b = [0.0] * 2
    for row in rows:
        g = signed(row)
        x = [g[0] - g[1], 1.0]
        target = row["label"] - top * g[2] - (total - top) * g[1]
        for i in range(2):
            for j in range(2):
                a[i][j] += x[i] * x[j]
            b[i] += x[i] * target
    return a, b


def tempo_constraints(top, total):
    """The schema, on the one free weight `w1`, with `w2 = total - top - w1`.

    `w1 >= 1` and `w2 >= w1 + 1`, the latter being `total - top - 2*w1 >= 1`.
    """
    return [([1.0, 0.0], 1.0),
            ([-2.0, 0.0], 1.0 - (float(total) - float(top)))]


def fit(rows, committed):
    """The whole answer, and the contrast that says why it is that answer."""
    fitted, counts = select(rows)
    train, val = split(fitted)
    top = float(committed[QUIET_COUNTS - 1])
    total = float(sum(committed[:QUIET_COUNTS]))

    # The shipping model, kept only as the contrast: no intercept, free scale.
    plain_a, plain_b = normal_equations(train)
    plain = solve(plain_a, plain_b)

    answer, skipped = constrained_min(*tempo_normal_equations(train, top, total),
                                      tempo_constraints(top, total))
    quiet = [answer[0], total - top - answer[0], top]
    table = round_to_schema(quiet, committed[QUIET_COUNTS]) + committed[QUIET_COUNTS:]
    return {
        "counts": counts, "train": train, "val": val,
        "no_intercept": plain, "quiet": quiet, "tempo": answer[1], "table": table,
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
          f"  one_sided_window {counts['dropped_one_sided_window']}"
          f"  saturated_label {counts['dropped_saturated_label']}")
    print(f"fit: train {len(train)} val {len(val)}")
    print(f"fit: no-intercept free-scale solve {['%.4f' % x for x in answer['no_intercept']]}"
          f"   (the CONTRAST, not the answer)")
    print(f"fit: tempo term c {answer['tempo']:+.2f}   (fitted, then discarded: the v0 "
          f"schema has no entry for it)")
    print(f"fit: constrained quiet optimum {['%.4f' % x for x in answer['quiet']]}")
    print(f"fit: committed {committed}")
    print(f"fit: fitted table {table}   (entries 4 and 5 pinned by R6/D-622; "
          f"entry 3 and the quiet SUM pinned at the committed values, which leaves "
          f"one degree of freedom)")
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
