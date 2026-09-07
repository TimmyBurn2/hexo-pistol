"""How much a SPLIT SEED moves a closed-form fit, and how that scales with K.

The matrix needs a seed budget per row, and for the closed-form rows the only
seeded thing is the train/validation split — `tools/texel/fit.py` is explicit
that its own fit has "no seed, no learning rate, no stopping rule", and
`training_pipeline_2026-09.md` §1 rules that the split must be by GAME. So the
question this answers is: over S game-level splits, how far apart are the
fitted weights and the validation MSE?

The row priced is R-A5-TOPK's shape — a free weight per top-K folded window
code plus one rare bucket, over the (axis, start) windows the committed census
instrument counts. The fit is ordinary least squares against the search label,
solved from per-GAME sufficient statistics so that S splits cost one corpus
walk rather than S.

NOTHING HERE GATES ANYTHING (D-614) and no weight table it prints is a
candidate: it is a variance measurement of a procedure.

Usage: seed_pilot.py <out.txt> <L> <K> [<K> ...] [--seeds N] [--limit N]
                     [--manifest PATH] [--tranche TEMPLATE]
"""

import collections
import pathlib
import random
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "texel"))
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import features as F
import hexenum as E

MANIFEST = "artifacts/arc3r_sweep_deduped_manifest.txt"
TRANCHE = "/home/tom/Projects/pistol-corpus/arc3r-sweep/tranche-{}/corpus.txt"
GAME, MOVES, TO_MOVE, SCORE_KIND, SCORE_VALUE = 0, 2, 6, 7, 8
HOLDOUT = 8


def quiet(stones):
    a, b = F.per_side_counts(stones)
    return not any(a[k] or b[k] for k in (4, 5, 6))


def window_codes(stones, to_move, length):
    """Folded codes of every length-L window holding a stone, one per (axis, start).

    The enumeration and the fold are `artifacts/research_2026-09/
    probe_patterns_folded.py`'s, so the counts this walk produces are in the
    same unit as `eval_families_2026-09.md` §0.2's receipt.
    """
    owner = {}
    for (q, r), player in stones:
        owner[(q, r)] = 1 if (player == 0) == (to_move == "p1") else 2
    seen = set()
    for (q, r) in owner:
        for axis, (dq, dr) in enumerate(F.AXES):
            for back in range(length):
                seen.add((axis, q - back * dq, r - back * dr, dq, dr))
    out = []
    for _axis, sq, sr, dq, dr in seen:
        code = tuple(owner.get((sq + i * dq, sr + i * dr), 0) for i in range(length))
        out.append(min(code, code[::-1]))
    return out


def walk(manifest, tranche, length, limit):
    """`[(game, label, [codes])]` over the quiet population, one pass."""
    wanted = {}
    with open(manifest) as handle:
        for line in handle:
            if line.startswith("#"):
                continue
            row = line.rstrip("\n").split("\t")
            wanted.setdefault(int(row[0]), set()).add((int(row[1]), row[4]))
    rows = []
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
            rows.append((f"{index}:{record[GAME]}", int(record[SCORE_VALUE]),
                         window_codes(stones, record[TO_MOVE], length)))
            if limit and len(rows) >= limit:
                return rows
    return rows


def per_game_statistics(rows, top):
    """`{game: (xtx, xty, yy, n)}` — enough to solve any subset of games."""
    index = {code: i for i, code in enumerate(top)}
    width = len(top) + 1
    rare = len(top)
    stats = {}
    for game, label, codes in rows:
        vector = [0] * width
        for code in codes:
            vector[index.get(code, rare)] += 1
        entry = stats.get(game)
        if entry is None:
            entry = ([0.0] * (width * width), [0.0] * width, 0.0, 0)
            stats[game] = entry
        xtx, xty, _yy, _n = entry
        for i in range(width):
            vi = vector[i]
            if not vi:
                continue
            base = i * width
            for j in range(i, width):
                xtx[base + j] += vi * vector[j]
            xty[i] += vi * label
        stats[game] = (xtx, xty, entry[2] + label * label, entry[3] + 1)
    for game, (xtx, xty, yy, n) in stats.items():
        for i in range(len(xty)):
            for j in range(i):
                xtx[i * len(xty) + j] = xtx[j * len(xty) + i]
    return stats


def solve(xtx, xty, width, ridge=1e-6):
    """Gaussian elimination with partial pivoting on a ridged normal matrix."""
    a = [xtx[r * width:(r + 1) * width] + [xty[r]] for r in range(width)]
    for r in range(width):
        a[r][r] += ridge
    for col in range(width):
        pivot = max(range(col, width), key=lambda r: abs(a[r][col]))
        if abs(a[pivot][col]) < 1e-12:
            return None
        a[col], a[pivot] = a[pivot], a[col]
        scale = a[col][col]
        row = a[col]
        for j in range(col, width + 1):
            row[j] /= scale
        for r in range(width):
            if r == col or a[r][col] == 0.0:
                continue
            factor = a[r][col]
            for j in range(col, width + 1):
                a[r][j] -= factor * row[j]
    return [a[r][width] for r in range(width)]


def mse(weights, xtx, xty, yy, n, width):
    """`(w'X'Xw - 2w'X'y + y'y) / n` — the fit's own objective on a subset."""
    quadratic = 0.0
    for i in range(width):
        wi = weights[i]
        if wi == 0.0:
            continue
        base = i * width
        for j in range(width):
            quadratic += wi * xtx[base + j] * weights[j]
    linear = sum(weights[i] * xty[i] for i in range(width))
    return (quadratic - 2 * linear + yy) / n


def combine(stats, games, width):
    xtx = [0.0] * (width * width)
    xty = [0.0] * width
    yy = 0.0
    n = 0
    for game in games:
        g_xtx, g_xty, g_yy, g_n = stats[game]
        for i in range(width * width):
            xtx[i] += g_xtx[i]
        for i in range(width):
            xty[i] += g_xty[i]
        yy += g_yy
        n += g_n
    return xtx, xty, yy, n


def main(argv):
    args = list(argv)
    out_path = args.pop(0)
    length = int(args.pop(0))
    ks = []
    seeds = 8
    limit = None
    manifest, tranche = MANIFEST, TRANCHE
    while args:
        token = args.pop(0)
        if token == "--seeds":
            seeds = int(args.pop(0))
        elif token == "--limit":
            limit = int(args.pop(0))
        elif token == "--manifest":
            manifest = args.pop(0)
        elif token == "--tranche":
            tranche = args.pop(0)
        else:
            ks.append(int(token))
    if not ks:
        raise SystemExit("seed_pilot: name at least one K")

    rows = walk(manifest, tranche, length, limit)
    games = sorted({game for game, _label, _codes in rows})
    if len(games) < HOLDOUT:
        raise SystemExit(f"seed_pilot: {len(games)} games cannot make a 1-in-{HOLDOUT} split")
    frequency = collections.Counter()
    for _game, _label, codes in rows:
        frequency.update(codes)
    print(f"walked {len(rows)} quiet rows over {len(games)} games, "
          f"{len(frequency)} distinct folded L{length} codes", flush=True)

    lines = [f"# seed pilot — R-A5-TOPK shape, length {length}",
             f"quiet rows: {len(rows)}   games: {len(games)}   "
             f"distinct folded codes: {len(frequency)}",
             f"split: 1-in-{HOLDOUT} of GAMES, {seeds} seeds",
             "",
             "K\tseed\tparams\ttrain_mse\tval_mse",
             ]
    summary = []
    for k in ks:
        top = [code for code, _count in frequency.most_common(k)]
        width = len(top) + 1
        stats = per_game_statistics(rows, top)
        print(f"K={k}: per-game statistics built ({width} parameters)", flush=True)
        vals = []
        tables = []
        for seed in range(seeds):
            order = list(games)
            random.Random(1000 + seed).shuffle(order)
            cut = len(order) // HOLDOUT
            validation, train = order[:cut], order[cut:]
            t_xtx, t_xty, t_yy, t_n = combine(stats, train, width)
            weights = solve(t_xtx, t_xty, width)
            if weights is None:
                lines.append(f"{k}\t{seed}\t{width}\tSINGULAR\tSINGULAR")
                continue
            v_xtx, v_xty, v_yy, v_n = combine(stats, validation, width)
            train_mse = mse(weights, t_xtx, t_xty, t_yy, t_n, width)
            val_mse = mse(weights, v_xtx, v_xty, v_yy, v_n, width)
            vals.append(val_mse)
            tables.append(weights)
            lines.append(f"{k}\t{seed}\t{width}\t{train_mse:.1f}\t{val_mse:.1f}")
        if vals:
            spread = max(vals) - min(vals)
            centre = sum(vals) / len(vals)
            worst = max(max(abs(t[i] - sum(u[i] for u in tables) / len(tables))
                            for t in tables)
                        for i in range(width))
            scale = max(abs(w) for t in tables for w in t) or 1.0
            summary.append(f"{k}\t{width}\t{centre:.1f}\t{spread:.1f}\t"
                           f"{100 * spread / centre:.3f}\t{100 * worst / scale:.2f}")
    lines.append("")
    lines.append("K\tparams\tval_mse_mean\tval_mse_spread\tspread_pct\tworst_weight_dev_pct")
    lines.extend(summary)
    body = "\n".join(lines) + "\n"
    pathlib.Path(out_path).write_text(body)
    print(body, flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
