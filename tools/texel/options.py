"""Regenerate the OPTION MATRIX's own tables, from the shipped fit.

`docs/experiments/matrix_wp22_quiet_scale.md` carries measured tables, and a
matrix whose numbers no committed instrument can reproduce is a matrix a
successor has to take on trust. This is that instrument: every row of the
matrix's option table and of its normalisation table is printed here, by the
same `fit.py` the phase registers.

It replaces two scratchpad harnesses that a `constrained_min` signature change
had left unrunnable -- and one of which still computed a table the matrix had
withdrawn, so the document and its stated provenance disagreed without saying so.

Usage: tools/texel/options.py <rows file>
"""

import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import fit as FIT

# Every option the matrix scores, as the RULE that fixes the quiet entries.
# A pin is applied INSIDE the solve, which is what makes each row a minimiser
# rather than a rescale of one.
PINS = (
    ("J   w3 = 60", "index", 2, 60.0),
    ("    w1 = 2", "index", 0, 2.0),
    ("    w2 = 12", "index", 1, 12.0),
    ("    sum = 74", "sum", None, 74.0),
    ("K   unpinned", "free", None, None),
)


def solve_pinned(train, kind, index, value, committed):
    """The intercept model's exact constrained minimiser under one pin."""
    if kind == "free":
        rows_a = [[0.0] * 4 for _ in range(4)]
        rows_b = [0.0] * 4
        for row in train:
            g = FIT.signed(row)
            x = [g[0], g[1], g[2], 1.0]
            for i in range(4):
                for j in range(4):
                    rows_a[i][j] += x[i] * x[j]
                rows_b[i] += x[i] * row["label"]
        answer = FIT.solve(rows_a, rows_b)
        return answer[:3], answer[3]
    if kind == "sum":
        rows_a = [[0.0] * 3 for _ in range(3)]
        rows_b = [0.0] * 3
        for row in train:
            g = FIT.signed(row)
            x = [g[0] - g[2], g[1] - g[2], 1.0]
            target = row["label"] - value * g[2]
            for i in range(3):
                for j in range(3):
                    rows_a[i][j] += x[i] * x[j]
                rows_b[i] += x[i] * target
        answer = FIT.solve(rows_a, rows_b)
        return [answer[0], answer[1], value - answer[0] - answer[1]], answer[2]
    free = [k for k in range(3) if k != index]
    rows_a = [[0.0] * 3 for _ in range(3)]
    rows_b = [0.0] * 3
    for row in train:
        g = FIT.signed(row)
        x = [g[free[0]], g[free[1]], 1.0]
        target = row["label"] - g[index] * value
        for i in range(3):
            for j in range(3):
                rows_a[i][j] += x[i] * x[j]
            rows_b[i] += x[i] * target
    answer = FIT.solve(rows_a, rows_b)
    quiet = [0.0] * 3
    quiet[index] = value
    quiet[free[0]], quiet[free[1]] = answer[0], answer[1]
    return quiet, answer[2]


def diagnostics(rows, table):
    """What the matrix reports, and every one of them GATES NOTHING (D-614)."""
    errors = [FIT.predict(FIT.signed(r), table) - r["label"] for r in rows]
    count = len(errors)
    mean_square = sum(e * e for e in errors) / count
    bias = sum(errors) / count
    return mean_square, bias, mean_square - bias * bias, FIT.rank_correlation(rows, table)


def main(rows_path):
    committed = FIT.committed_table()
    rows = FIT.read_rows(rows_path)
    fitted, _counts = FIT.select(rows)
    train, val = FIT.split(fitted)
    every = [r for r in rows if r["kind"] == "eval"]
    all_val = [r for r in every if r["key"][-1] in "01"]

    header = (f"{'option':<16}{'table':<26}{'val_MSE':>11}{'bias':>9}"
              f"{'resid var':>12}{'quiet rho':>11}{'ALL rho':>9}"
              f"{'w3/w4':>8}{'sum/w4':>8}")
    print(header)
    print("-" * len(header))
    rows_out = [("--  committed", committed, None)]
    for label, kind, index, value in PINS:
        quiet, tempo = solve_pinned(train, kind, index, value, committed)
        try:
            table = FIT.round_to_schema(quiet, committed[FIT.QUIET_COUNTS])
        except FIT.FitError as why:
            # A PIN CAN LAND OUTSIDE THE SCHEMA, and that is a result about the
            # option rather than a failure of this script: an option whose
            # answer the weights document cannot hold is not an option.
            print(f"{label:<16}{'INFEASIBLE':<26}{str(why).split(';')[0][5:]}")
            continue
        rows_out.append((label, table + committed[FIT.QUIET_COUNTS:], tempo))
    for label, table, tempo in rows_out:
        mse, bias, variance, rho = diagnostics(val, table)
        _m, _b, _v, all_rho = diagnostics(all_val, table)
        print(f"{label:<16}{str(table):<26}{mse:>11.1f}{bias:>9.1f}{variance:>12.1f}"
              f"{rho:>11.4f}{all_rho:>9.4f}"
              f"{table[2] / table[3]:>8.4f}{sum(table[:3]) / table[3]:>8.4f}")
    print()
    print("The `w3/w4` and `sum/w4` columns are the point: the filtered rows carry")
    print("no evidence about ANY quiet-to-tactical exchange rate, and each pin holds")
    print("one of them while moving the others.")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: tools/texel/options.py <rows file>")
    main(sys.argv[1])
