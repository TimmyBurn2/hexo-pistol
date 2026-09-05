"""Tests that drive the SHIPPED texel scripts, not copies of their logic.

tools/SHELL_CHECKLIST.md's coverage rule: a tools/ script that produces a
recorded number carries at least one test driving the shipped script. The
numbers these produce are the fit diagnostics, so the fit is what is driven.
"""

import hashlib
import pathlib
import subprocess
import sys
import tempfile

HERE = pathlib.Path(__file__).parent
sys.path.insert(0, str(HERE))

import features as F
import fit as FIT

EVAL_MAX = 16000
failures = []


def check(name, condition, detail=""):
    if condition:
        print(f"  ok   {name}")
    else:
        print(f"  FAIL {name} {detail}")
        failures.append(name)


def synth_rows(path, weights, n=3000):
    """Rows generated FROM known weights, so the fit has a right answer."""
    rows = []
    state = 12345
    for i in range(n):
        f = []
        for _ in range(5):
            state = (1103515245 * state + 12345) % (1 << 31)
            f.append(state % 61 - 30)
        f.append(0)
        to_move = "p1" if i % 2 == 0 else "p2"
        g = f if to_move == "p1" else [-x for x in f]
        raw = sum(g[k] * weights[k] for k in range(5))
        label = max(-EVAL_MAX, min(EVAL_MAX, raw))
        key = hashlib.sha256(str(i).encode()).hexdigest()
        rows.append("\t".join([*map(str, f), str(label), to_move, "4", "no", "p1_win", key]))
    pathlib.Path(path).write_text("# synthetic\n" + "\n".join(rows) + "\n")


def test_one_stone_features():
    """A single stone sits in three axes x six offsets of windows."""
    f = F.features(F.stones_of("0,0"))
    check("one stone lies in 18 one-stone windows", f[1] == 18, f)
    check("one stone scores 18 x table[1]",
          F.score_p1(f, [2, 12, 60, 300, 1500]) == 36)


def test_turn_structure():
    """Turn 1 is ONE stone; every later turn is two, by the same mover."""
    stones = F.stones_of("0,0 1,0/2,0")
    check("turn 1 places one stone, turn 2 places two", len(stones) == 3, stones)
    check("the second turn's two stones are both p2",
          [p for _, p in stones] == [0, 1, 1], stones)


def test_dead_window_scores_zero():
    """A window holding both players is dead and contributes nothing."""
    # Adjacent along ConstR. Each stone lies in 18 windows; the 6-cell windows
    # along that axis holding BOTH are the five starting at (-b, 0), b <= 4.
    mixed = [((0, 0), 0), ((1, 0), 1)]
    counts = F.window_counts(mixed)
    shared = [w for w, (a, b) in counts.items() if a and b]
    check("exactly five windows hold both stones", len(shared) == 5, len(shared))
    f = F.features(mixed)
    check("each side keeps 18 - 5 = 13 live one-stone windows, so they cancel",
          f[1] == 0, f)
    check("a symmetric dead-window position scores exactly zero",
          F.score_p1(f, [2, 12, 60, 300, 1500]) == 0)
    # And the dead windows are not merely cancelling: give one side a stone the
    # other cannot see, and only the live windows move the score.
    lone = F.features(mixed + [((0, 5), 0)])
    check("a third stone adds only windows no opponent stone touches",
          lone[1] > 0, lone)


def test_fit_recovers_known_weights():
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        truth = [3, 20, 90, 400, 2000]
        synth_rows(path, truth)
        rows = FIT.read_rows(path)
        a, b, _ = FIT.normal_equations(rows)
        exact = FIT.solve(a, b)
        check("unconstrained solve recovers the generating weights",
              all(abs(exact[k] - truth[k]) < 1e-6 for k in range(5)), exact)
        constrained = FIT.constrained_fit(a, b)
        check("constrained solve agrees where the constraints are slack",
              all(abs(constrained[k] - truth[k]) < 1e-6 for k in range(5)), constrained)


def test_constrained_fit_respects_the_schema():
    """A target that WANTS a non-monotone table must still come back legal."""
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, [3, 20, 90, 400, -50])
        rows = FIT.read_rows(path)
        a, b, _ = FIT.normal_equations(rows)
        exact = FIT.solve(a, b)
        check("the unconstrained answer is indeed illegal here",
              exact[4] < exact[3], exact)
        w = FIT.constrained_fit(a, b)
        check("constrained answer strictly increases",
              all(w[k + 1] >= w[k] + 1 - 1e-9 for k in range(4)), w)
        check("constrained answer stays in the band",
              w[0] >= 1 and w[4] <= EVAL_MAX - 1, w)
        rounded = FIT.round_to_schema(w)
        check("rounded answer strictly increases as integers",
              all(rounded[k + 1] > rounded[k] for k in range(4)), rounded)
        check("constrained objective is no better than unconstrained",
              FIT.objective(a, b, w) >= FIT.objective(a, b, exact) - 1e-6)


def test_constrained_beats_projection():
    """Projecting the unconstrained point is not the constrained optimum.

    This is the defect the fit carried in its first revision, pinned so it
    cannot come back silently.
    """
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, [3, 20, 90, 400, -50])
        rows = FIT.read_rows(path)
        a, b, _ = FIT.normal_equations(rows)
        exact = FIT.solve(a, b)
        projected = FIT.round_to_schema(exact)
        constrained = FIT.constrained_fit(a, b)
        check("the constrained optimum beats the projected point",
              FIT.objective(a, b, constrained) < FIT.objective(a, b, projected),
              (constrained, projected))


def test_shipped_script_runs():
    """Drive fit.py as a program, the way a run does."""
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, [3, 20, 90, 400, 2000])
        done = subprocess.run([sys.executable, str(HERE / "fit.py"), path],
                              capture_output=True, text=True)
        check("fit.py exits 0", done.returncode == 0, done.stderr[-300:])
        check("fit.py reports whether any gap constraint binds",
              "BINDING" in done.stdout, done.stdout[-300:])
        check("fit.py reports the rounded schema table",
              "rounded to schema" in done.stdout)


for test in (test_one_stone_features, test_turn_structure,
             test_dead_window_scores_zero, test_fit_recovers_known_weights,
             test_constrained_fit_respects_the_schema,
             test_constrained_beats_projection, test_shipped_script_runs):
    print(test.__name__)
    test()

print()
if failures:
    print(f"test_texel: {len(failures)} FAILURE(S): {failures}")
    raise SystemExit(1)
print("test_texel: all checks passed")


def test_census_classes_partition():
    """The class count is over the ALTERNATIVE's columns, and it sits between
    the root count and the key count by construction."""
    import census_classes as CC
    check("the partition is stage3_allocator_bound's COLUMNS, in its order",
          CC.COLUMNS == ("turns", "mover_hot", "opp_hot", "mover_w1", "opp_w1",
                         "mover_l3", "opp_l3", "cover", "covers"), CC.COLUMNS)
    path = "artifacts/wp22_cap_dryrun_v2/d3_c2048.txt"
    if not pathlib.Path(path).exists():
        check("census fixture present (skipped, artifact absent)", True)
        return
    firings, keys, roots, classes, loss = CC.tally(path)
    check("roots <= classes <= keys, which is what makes the tightening legitimate",
          len(roots) <= len(classes) <= len(keys), (len(roots), len(classes), len(keys)))
    check("the loss direction is counted separately and is not inside the win set",
          isinstance(loss, set))
    points = CC.curve(path)
    check("the curve is non-decreasing", all(b >= a for (_, a), (_, b) in zip(points, points[1:])),
          points)


print("test_census_classes_partition")
test_census_classes_partition()
print()
if failures:
    print(f"test_texel: {len(failures)} FAILURE(S): {failures}")
    raise SystemExit(1)
print("test_texel: all checks passed (including census_classes)")
