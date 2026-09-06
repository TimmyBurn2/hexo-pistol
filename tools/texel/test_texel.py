"""Tests that drive the SHIPPED texel scripts, not copies of their logic.

tools/SHELL_CHECKLIST.md's coverage rule: a tools/ script that produces a
recorded number carries at least one test driving the shipped script. The
numbers these produce are the fit diagnostics, the oracle's disagreement count
and the census tallies, so all three are driven.
"""

import hashlib
import pathlib
import re
import subprocess
import sys
import tempfile

HERE = pathlib.Path(__file__).parent
sys.path.insert(0, str(HERE))

import features as F
import fit as FIT
import verify_against_engine as ORACLE

EVAL_MAX = 16000
failures = []


def check(name, condition, detail=""):
    if condition:
        print(f"  ok   {name}")
    else:
        print(f"  FAIL {name} {detail}")
        failures.append(name)


def synth_rows(path, weights, n=3000, quiet_only=True):
    """Rows generated FROM known weights, so the fit has a right answer.

    `quiet_only` writes rows the design's filter KEEPS, which is what the fit
    consumes; the tactical counts are zero there by construction.
    """
    lines = []
    state = 12345
    for i in range(n):
        a, b = [0] * 6, [0] * 6
        for k in range(3):
            state = (1103515245 * state + 12345) % (1 << 31)
            a[k] = state % 31
            state = (1103515245 * state + 12345) % (1 << 31)
            b[k] = state % 31
        if not quiet_only:
            a[3] = i % 2
        to_move = "p1" if i % 2 == 0 else "p2"
        f = [a[k] - b[k] for k in range(6)]
        g = f if to_move == "p1" else [-x for x in f]
        raw = sum(g[k] * weights[k] for k in range(5))
        label = max(-EVAL_MAX, min(EVAL_MAX, raw))
        key = hashlib.sha256(str(i).encode()).hexdigest()
        lines.append("\t".join([*map(str, a), *map(str, b), str(label), to_move,
                                "eval", "4", "no", "p1_win", key]))
    pathlib.Path(path).write_text("# synthetic\n" + "\n".join(lines) + "\n")


COMMITTED = FIT.committed_table("configs/eval_v0_weights.toml")


def test_one_stone_features():
    """A single stone sits in three axes x six offsets of windows."""
    f = F.features(F.stones_of("0,0"))
    check("one stone lies in 18 one-stone windows", f[1] == 18, f)
    check("one stone scores 18 x table[1]",
          F.score_p1(f, [2, 12, 60, 300, 1500]) == 36)


def test_turn_structure():
    """Turn 1 places ONE stone; every later turn is two, by the same mover."""
    stones = F.stones_of("0,0 1,0/2,0")
    check("turn 1 places one stone, turn 2 places two", len(stones) == 3, stones)
    check("the second turn's two stones are both p2",
          [p for _, p in stones] == [0, 1, 1], stones)


def test_dead_window_scores_zero():
    """A window holding both players is dead and contributes nothing."""
    mixed = [((0, 0), 0), ((1, 0), 1)]
    counts = F.window_counts(mixed)
    shared = [w for w, (a, b) in counts.items() if a and b]
    check("exactly five windows hold both stones", len(shared) == 5, len(shared))
    f = F.features(mixed)
    check("each side keeps 18 - 5 = 13 live one-stone windows, so they cancel",
          f[1] == 0, f)
    check("a symmetric dead-window position scores exactly zero",
          F.score_p1(f, [2, 12, 60, 300, 1500]) == 0)
    lone = F.features(mixed + [((0, 5), 0)])
    check("a third stone adds only windows no opponent stone touches",
          lone[1] > 0, lone)


def test_each_axis_is_pinned_separately():
    """An axis changed in `AXES` moves the window bookkeeping on that axis only.

    A count of eighteen windows through a lone stone survives any permutation of
    three directions, so it cannot see the change; two stones adjacent ALONG a
    given axis sharing exactly five windows can, once per axis.
    """
    # THE REFERENT IS THE ENGINE'S OWN SOURCE, not a second copy here and not
    # `F.AXES` itself: a fixture built from the thing under test moves with it,
    # which is how an axis mutant survived a version of this test.
    source = pathlib.Path("crates/pistol-core/src/axis.rs").read_text()
    body = source.split("pub const fn direction(self) -> Coord {", 1)[1].split("}", 1)[0]
    engine = tuple(
        (int(a), int(b))
        for a, b in re.findall(r"Coord::new\((-?\d+),\s*(-?\d+)\)", body)
    )
    check("the engine's own source states three directions", len(engine) == 3, engine)
    check("and features.py's AXES are those three, in that order",
          tuple(F.AXES) == engine, (F.AXES, engine))
    for index, (dq, dr) in enumerate(engine):
        pair = [((0, 0), 0), ((dq, dr), 0)]
        shared = [w for w, (a, b) in F.window_counts(pair).items() if a == 2]
        check(f"two stones one step apart along axis {index} share exactly five windows",
              len(shared) == 5, (index, (dq, dr), len(shared)))
        far = [((0, 0), 0), ((6 * dq, 6 * dr), 0)]
        check(f"and six steps apart along axis {index} they share none",
              not [w for w, (a, b) in F.window_counts(far).items() if a == 2], index)


def test_the_six_stone_window_is_spliced_at_the_decided_value():
    """`weights.rs` splices index 6 as the decided window's value, and the
    offline path must too. Dropping the splice scores a win as a pile of fives,
    which no table entry can express and no smaller fixture reveals."""
    # p1 takes 0,0 1,0 2,0 3,0 4,0 5,0 over four of its own turns; p2 plays far.
    six = "0,0 -30,1/-30,0 1,0/2,0 -29,1/-29,0 3,0/4,0 -28,1/-28,0 -40,0/5,0"
    f = F.features(F.stones_of(six))
    check("the fixture really does hold a six-stone window", f[6] >= 1, f)
    check("and the value saturates the band, which is what the splice delivers",
          F.score_p1(f, [2, 12, 60, 300, 1500]) == F.EVAL_MAX, f)
    check("without the splice the same position would score far below it",
          sum(f[k] * [2, 12, 60, 300, 1500][k - 1] for k in range(1, 6)) < F.EVAL_MAX)


def test_the_mover_alternates_per_TURN_and_three_turns_is_what_shows_it():
    """Two turns cannot tell per-turn from per-stone: both give [p1, p2, p2].
    The third turn is where they part."""
    two = [player for _, player in F.stones_of("0,0 1,0/2,0")]
    check("two turns cannot distinguish the two rules", two == [0, 1, 1], two)
    three = [player for _, player in F.stones_of("0,0 1,0/2,0 3,0/4,0")]
    check("three turns do: the third turn is the FIRST player's again",
          three == [0, 1, 1, 0, 0], three)
    four = [player for _, player in F.stones_of("0,0 1,0/2,0 3,0/4,0 5,0/6,0")]
    check("and the fourth is the second player's",
          four == [0, 1, 1, 0, 0, 1, 1], four)


def test_per_side_counts_name_the_owner_the_signed_vector_cannot():
    """The filter needs OWNERSHIP, and the signed difference destroys it."""
    both = [((0, 0), 0), ((0, 6), 1)]
    a, b = F.per_side_counts(both)
    check("each side owns its own one-stone windows", a[1] == 18 and b[1] == 18, (a, b))
    check("the signed vector reports zero where both sides hold one",
          F.features(both)[1] == 0)
    check("so a filter written on the signed vector would keep this position "
          "and one written on ownership would not", (a[1] > 0) != (F.features(both)[1] != 0))


def test_the_filter_is_a_predicate_on_ownership():
    """Design section 3 clause 2: EITHER side holding a live four is enough.

    It is a predicate on OWNERSHIP, not on forcedness -- D-626 measured that
    nothing it drops is forced.
    """
    row = {"a": [0] * 7, "b": [0] * 7, "kind": "eval", "label": 0}
    check("an empty board holds no one-sided tactical window",
          not FIT.has_one_sided_tactical_window(row))
    row["b"][4] = 1
    check("a live four owned by the NON-mover is one, and it is the case that "
          "actually occurs: the mover owns one in 0 of the corpus's eval rows",
          FIT.has_one_sided_tactical_window(row))
    row["b"][4], row["a"][5] = 0, 1
    check("the predicate is on either side, so a mover-owned five is one too — "
          "a configuration rule 4 makes impossible at a turn boundary, which is "
          "why the predicate and not the corpus is what this pins",
          FIT.has_one_sided_tactical_window(row))
    row["a"][5], row["a"][3] = 0, 9
    check("nine live threes are not one", not FIT.has_one_sided_tactical_window(row))


def test_the_filter_reports_every_clause_it_applies():
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, COMMITTED, n=40, quiet_only=False)
        rows = FIT.read_rows(path)
        rows[0]["kind"] = "mate_in"
        rows[1]["label"] = EVAL_MAX
        rows[1]["a"][4] = 0
        fitted, counts = FIT.select(rows)
        check("the score_kind clause is counted", counts["dropped_score_kind"] == 1, counts)
        check("the one-sided-window clause is counted",
              counts["dropped_one_sided_window"] > 0, counts)
        check("the saturated-label clause is counted",
              counts["dropped_saturated_label"] == 1, counts)
        check("the populations add up to the whole",
              counts["fitted"] + counts["dropped_score_kind"]
              + counts["dropped_one_sided_window"] + counts["dropped_saturated_label"]
              == counts["all"], counts)
        check("no fitted row carries a non-zero tactical regressor — which is what "
              "the clause delivers, and NOT that the population is free of forced "
              "wins: 5 339 mate rows pass it and clause 1 is what removes them",
              not any(FIT.has_one_sided_tactical_window(r) for r in fitted))


def test_an_empty_filtered_population_is_refused_not_defaulted():
    """Hard rule 3: a fit with nothing to fit raises rather than returning the
    committed table wearing a fit's name."""
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, COMMITTED, n=10)
        rows = FIT.read_rows(path)
        for row in rows:
            row["kind"] = "mate_in"
        try:
            FIT.select(rows)
            check("an empty filtered population is refused", False, "no exception")
        except FIT.FitError as why:
            check("an empty filtered population is refused by name",
                  "no rows to fit" in str(why), str(why))


def test_fit_recovers_known_quiet_weights():
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        truth = [3, 20, 90, 400, 2000]
        synth_rows(path, truth)
        rows = FIT.read_rows(path)
        fitted, _ = FIT.select(rows)
        a, b = FIT.normal_equations(fitted)
        exact = FIT.solve(a, b)
        check("unconstrained solve recovers the generating quiet weights",
              all(abs(exact[k] - truth[k]) < 1e-6 for k in range(3)), exact)
        w, _ = FIT.constrained_min(a, b, FIT.schema_constraints(3, truth[3]))
        check("the constrained solve agrees where the constraints are slack",
              all(abs(w[k] - truth[k]) < 1e-6 for k in range(3)), w)


def test_constrained_fit_respects_the_schema():
    """A target that WANTS a non-monotone table must still come back legal."""
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, [3, 90, 20, 400, 2000])
        rows = FIT.read_rows(path)
        fitted, _ = FIT.select(rows)
        a, b = FIT.normal_equations(fitted)
        exact = FIT.solve(a, b)
        check("the unconstrained answer is indeed illegal here", exact[2] < exact[1], exact)
        w, _ = FIT.constrained_min(a, b, FIT.schema_constraints(3, 400))
        check("the constrained answer strictly increases",
              all(w[k + 1] >= w[k] + 1 - 1e-9 for k in range(2)), w)
        check("the constrained answer stays inside the pinned ceiling", w[2] <= 399, w)
        check("the constrained objective is no better than the unconstrained one",
              FIT.objective(a, b, w) >= FIT.objective(a, b, exact) - 1e-6)


def test_a_BOUND_is_an_active_set_member_and_not_a_clamp():
    """The design review's M-4 reproducer, pinned.

    A data set that wants the first entry far BELOW its bound is where a
    projecting implementation and the exact one part: the projection clamps and
    keeps the other entries where the unconstrained solve left them, and that
    point is not the minimiser over the constraint set.
    """
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, [-40, 20, 90, 400, 2000])
        rows = FIT.read_rows(path)
        fitted, _ = FIT.select(rows)
        a, b = FIT.normal_equations(fitted)
        exact = FIT.solve(a, b)
        check("the data really does want w1 below its bound", exact[0] < 1, exact)
        clamped = [max(1.0, x) for x in exact]
        w, _ = FIT.constrained_min(a, b, FIT.schema_constraints(3, 400))
        check("the exact answer is feasible", w[0] >= 1 - 1e-9, w)
        check("the exact answer BEATS the clamped one",
              FIT.objective(a, b, w) < FIT.objective(a, b, clamped) - 1e-9,
              (w, clamped))
        check("and the two are different tables",
              any(abs(w[k] - clamped[k]) > 1e-6 for k in range(3)), (w, clamped))


def test_rounding_refuses_at_BOTH_ends_and_the_floor_is_the_one_that_fires():
    """The ceiling guard is unreachable on the path this module runs: the last
    quiet entry sits at the pinned tactical value's own scale, far below it. The
    floor is where a silent clamp would actually happen."""
    try:
        FIT.round_to_schema([1.0, 2.0, 299.6], 300)
        check("a table rounding onto the pinned ceiling is refused", False, "no exception")
    except FIT.FitError as why:
        check("a table rounding onto the pinned ceiling is refused by name",
              "ceiling" in str(why), str(why))
    # A DRAW THAT CAN ACTUALLY REACH THE CASE. An earlier revision pinned the
    # opposite property over increments of 1/7, where a half-integer is
    # unreachable, so its assertion could not have failed.
    state, refused, ties = 99, 0, 0
    for _ in range(2000):
        real, floor = [], 1.0
        for _ in range(3):
            state = (1103515245 * state + 12345) % (1 << 31)
            floor += 1.0 + (state % 8) / 2.0
            real.append(floor)
        ties += any(abs(x - int(x) - 0.5) < 1e-12 for x in real)
        try:
            FIT.round_to_schema(real, 4000)
        except FIT.FitError:
            refused += 1
    check("the draw reaches the half-integer case the guard is about", ties > 0, ties)
    check("and the guard fires on it rather than clamping silently", refused > 0, refused)
    try:
        FIT.round_to_schema([0.9146, 11.8779, 60.0], 300)
        check("an infeasible real-valued answer is refused before rounding",
              False, "no exception")
    except FIT.FitError as why:
        check("an infeasible real-valued answer is refused by name",
              "not schema-feasible" in str(why), str(why))
    check("and a feasible answer still rounds",
          FIT.round_to_schema([4.8530, 33.6615, 60.0], 300) == [5, 34, 60])


def test_the_active_set_enumeration_agrees_with_a_brute_force_grid():
    """An ORACLE for the solver, not a restatement of it.

    The enumeration's claim is that it finds the constrained minimum. A grid
    search over the feasible box shares no code with it, so it can disagree.
    """
    a = [[7.0, 2.0, 1.0], [2.0, 9.0, 3.0], [1.0, 3.0, 11.0]]
    b = [40.0, 300.0, 700.0]
    cons = FIT.schema_constraints(3, 90)
    exact, _ = FIT.constrained_min(a, b, cons)
    best, best_value = None, None
    step = 0.25
    grid = [1.0 + step * i for i in range(int(88 / step))]
    for w1 in grid:
        for w2 in grid:
            if w2 < w1 + 1:
                continue
            for w3 in grid:
                if w3 < w2 + 1 or w3 > 89:
                    continue
                value = FIT.objective(a, b, [w1, w2, w3])
                if best_value is None or value < best_value:
                    best, best_value = [w1, w2, w3], value
    check("the enumeration is at least as good as the finest grid point",
          FIT.objective(a, b, exact) <= best_value + 1e-9, (exact, best))
    check("and it lands within a grid step of it",
          all(abs(exact[k] - best[k]) <= step + 1e-9 for k in range(3)), (exact, best))


def test_committed_weights_are_read_and_never_duplicated_in_code():
    """A table written twice is a table that will disagree with itself."""
    table = FIT.committed_table("configs/eval_v0_weights.toml")
    text = pathlib.Path("configs/eval_v0_weights.toml").read_text()
    stated = [int(line.split("=")[1]) for line in text.splitlines()
              if line[:1].isdigit() and "=" in line]
    check("committed_table returns exactly what the document states",
          table == stated, (table, stated))
    check("the fit module carries no second copy of the table",
          str(stated) not in pathlib.Path(HERE / "fit.py").read_text(), stated)


def test_dominance_is_checked_and_can_fail():
    check("the committed table dominates", FIT.dominance(COMMITTED) == (True, True))
    flat = FIT.dominance([1, 21, 22, 64, 65])
    check("a table whose top is flat fails the check at the entry that is flat",
          flat[1] is False, flat)
    check("and the check is not vacuous: it passes the entry that is not",
          flat[0] is True, flat)


def test_rank_correlation_averages_tied_ranks():
    """The no-ties formula is biased on data with many tied predictions, and an
    all-zero feature row is a tie by construction."""
    rows = [{"a": [0] * 7, "b": [0] * 7, "label": v, "to_move": "p1"} for v in (1, 2, 3, 4)]
    check("a constant prediction gives no correlation rather than a number",
          FIT.rank_correlation(rows, COMMITTED) != FIT.rank_correlation(rows, COMMITTED)
          or abs(FIT.rank_correlation(rows, COMMITTED)) < 1e-9,
          FIT.rank_correlation(rows, COMMITTED))
    check("ranks of a tied block are averaged",
          FIT.ranks([5, 5, 5, 9]) == [1.0, 1.0, 1.0, 3.0], FIT.ranks([5, 5, 5, 9]))


STUB = """import sys, pathlib
# An INDEPENDENT referent: 18 one-stone windows through a lone stone, times the
# first table entry, clamped. It shares no code with features.py.
w1 = None
for line in pathlib.Path(sys.argv[1]).read_text().splitlines():
    if line.startswith("1 ="):
        w1 = int(line.split("=")[1])
for line in pathlib.Path(sys.argv[2]).read_text().splitlines():
    turns = line.split()[2:]
    if not turns:
        print("0 p1")
    else:
        raw = max(-16000, min(16000, 18 * w1))
        print(f"{-raw if len(turns) % 2 else raw} {'p2' if len(turns) % 2 else 'p1'}")
"""


def _oracle_rows():
    """(tranche, key, moves, to_move, score_kind), as `corpus_rows` yields.

    Two positions only, and both trivial, because the stub that plays the engine
    computes its answer from a formula sharing no code with `features.py` — the
    whole point of it — and that formula is only defined for an empty board and
    a lone stone. The registered sample RULE is exercised on synthetic tuples
    below, where no engine is needed.
    """
    return [(1, "kA", "-", "p1", "eval"), (2, "kB", "0,0", "p2", "mate_in")]


def test_the_oracle_drives_the_engine_and_agrees():
    with tempfile.TemporaryDirectory() as tmp:
        stub = pathlib.Path(tmp) / "stub.py"
        stub.write_text(STUB)
        binary = pathlib.Path(tmp) / "stub.sh"
        binary.write_text(f'#!/bin/sh\nexec {sys.executable} {stub} "$@"\n')
        binary.chmod(0o755)
        report = ORACLE.run(str(binary), stride=1, rows=_oracle_rows())
        check("the oracle sampled every position", report["sampled"] == 2, report)
        # THE ENFORCEMENT IS DRIVEN THROUGH `run`, not only as a function: a
        # version that computed the rule and never applied it passed every gate,
        # because the suite's own call left the enforcement argument unset.
        check("run() APPLIES the rule and refuses when the draw fails it",
              _refuses(lambda: ORACLE.run(str(binary), stride=1, rows=_oracle_rows(),
                                          tranches=99, kinds=("eval", "mate_in")))
              is not None)
        check("and it REPORTS what the draw spans, which the run receipt did not",
              report["coverage"]["tranches"] == [1, 2]
              and report["coverage"]["kinds"] == ["eval", "mate_in"],
              report["coverage"])
        check("no table disagrees",
              all(t["disagreements"] == 0 for t in report["tables"].values()), report)
        check("at least one registered table saturates the band",
              any(t["saturating"] > 0 for t in report["tables"].values()), report)


def test_the_sample_takes_the_WHOLE_tactical_stratum_whatever_the_stride():
    """`sample()` is pure, so the clause that matters most needs no engine.

    The registered rule's first clause is EVERY position holding a four- or
    five-window, not a stride draw that happens to include some: an unstratified
    draw exercises the top table entries in a handful of positions, which is the
    defect design section 5 exists to close.
    """
    # p1 takes 0,0 1,0 2,0 3,0 4,0 while p2 plays far away: a live five-window.
    tactical = "0,0 -20,0/-20,1 1,0/2,0 -19,0/-19,1 3,0/4,0"
    a, b = F.per_side_counts(F.stones_of(tactical))
    check("the fixture really does hold a live five-window", a[5] > 0, (a, b))
    rows = [(1, f"k{i}", "-", "p1", "eval") for i in range(9)]
    rows.insert(5, (1, "kT", tactical, "p2", "eval"))     # index 5, not on the stride
    chosen = ORACLE.sample(rows, stride=4)
    check("the tactical position is taken even though its index is off the stride",
          any(c[0] == "kT" for c in chosen), [c[0] for c in chosen])
    check("and it is marked as the tactical stratum",
          [c[4] for c in chosen if c[0] == "kT"] == [True], chosen)
    check("while the stride still draws the quiet ones",
          sum(1 for c in chosen if not c[4]) == 3, [c[0] for c in chosen])
    # 2 369 corpus rows share a `key_full`, so a draw that folded on it would
    # silently shrink the registered sample. Two rows with ONE key must both
    # survive.
    shared = [(1, "same", tactical, "p2", "eval"), (1, "same", tactical, "p1", "eval")]
    check("two positions sharing a key_full are BOTH kept",
          len(ORACLE.sample(shared, stride=1)) == 2, ORACLE.sample(shared, stride=1))


def _refuses(call):
    """Run `call` and return the refusal it raises, or None if it did not."""
    try:
        call()
        return None
    except ORACLE.OracleError as why:
        return str(why)


def test_the_oracle_REFUSES_an_unknown_to_move_token():
    """The same hard-rule-3 hazard `read_rows` guards, on the oracle's own walk."""
    import tempfile as _t
    with _t.TemporaryDirectory() as tmp:
        manifest, tranche = _scratch_corpus(tmp)
        for index in (1, 2):
            path = pathlib.Path(tmp) / f"tranche-{index}" / "corpus.txt"
            path.write_text(path.read_text().replace("\tp2\t", "\tP2\t", 1))
        try:
            list(ORACLE.corpus_rows(manifest, tranche))
            check("the oracle refuses an unknown to_move token", False, "no exception")
        except ORACLE.OracleError as why:
            check("the oracle refuses an unknown to_move token by name",
                  "to_move" in str(why), str(why))


def test_the_oracle_REFUSES_a_sample_that_misses_the_registered_rule():
    """The rule's clauses are inert on a small draw, so a mutation that dropped
    the tactical stratum or a tranche left every check green. These fire."""
    # Synthetic tuples: the rule is about the DRAW, and needs no engine.
    chosen = [("k1", "-", "p1", "eval", True, 1), ("k2", "-", "p2", "mate_in", False, 2),
              ("k3", "-", "p1", "mated_in", False, 2)]
    check("the rule passes the draw it is written for",
          ORACLE.check_sample_rule(chosen, 2, ("eval", "mate_in", "mated_in"))["sampled"] == 3)
    for name, args, token in (
            ("a missing tranche", (3, ("eval", "mate_in", "mated_in")), "tranche"),
            ("a missing score kind", (2, ("eval", "mate_in", "mated_in", "other")), "score kinds"),
    ):
        try:
            ORACLE.check_sample_rule(chosen, *args)
            check(f"{name} is refused", False, "no exception raised")
        except ORACLE.OracleError as why:
            check(f"{name} is refused by name", token in str(why), str(why))
    check("the registered table set carries a clamp table on EACH side",
          len([n for n in ORACLE.TABLES if n.startswith("clamp")]) == 2,
          sorted(ORACLE.TABLES))
    check("a missing clamp table is refused",
          _refuses(lambda: ORACLE.check_sample_rule(
              chosen, 2, ("eval", "mate_in", "mated_in"),
              {"committed": [], "clamp_high": []})) is not None)
    check("a sample folded on key_full is refused",
          _refuses(lambda: ORACLE.check_sample_rule(
              [chosen[0], chosen[0], chosen[1], chosen[2]], 2,
              ("eval", "mate_in", "mated_in"))) is not None)
    flat = [(k, m, tm, s, False, n) for (k, m, tm, s, _, n) in chosen]
    try:
        ORACLE.check_sample_rule(flat, 2, ("eval", "mate_in", "mated_in"))
        check("an empty tactical stratum is refused", False, "no exception raised")
    except ORACLE.OracleError as why:
        check("an empty tactical stratum is refused by name",
              "tactical stratum" in str(why), str(why))


def test_the_oracle_FAILS_on_a_disagreeing_engine():
    """The mutant that must die. A gate that cannot fail is testing nothing."""
    with tempfile.TemporaryDirectory() as tmp:
        stub = pathlib.Path(tmp) / "stub.py"
        stub.write_text(STUB.replace("18 * w1", "19 * w1"))
        binary = pathlib.Path(tmp) / "stub.sh"
        binary.write_text(f'#!/bin/sh\nexec {sys.executable} {stub} "$@"\n')
        binary.chmod(0o755)
        try:
            ORACLE.run(str(binary), stride=1, rows=_oracle_rows())
            check("a disagreeing engine stops the oracle", False, "no exception")
        except ORACLE.OracleError as why:
            check("a disagreeing engine stops the oracle by name",
                  "disagreement" in str(why), str(why))


def test_the_solve_is_invariant_under_the_labels_units():
    """The minimiser cannot depend on what the labels are measured in.

    An ABSOLUTE pivot threshold against a KKT system that mixes the normal
    matrix with constraint rows of O(1) made an earlier version discard almost
    every active set once the objective was scaled up, which is a wrong answer
    dressed as a singular one.
    """
    a = [[7.0, 2.0, 1.0], [2.0, 9.0, 3.0], [1.0, 3.0, 11.0]]
    b = [40.0, 300.0, 700.0]
    cons = FIT.schema_constraints(3, 300)
    base, _ = FIT.constrained_min(a, b, cons)
    # BOTH A AND b, because that is the only scaling that leaves the minimiser
    # where it was: scaling either alone is a different problem with a different
    # answer, and testing it would pin nothing.
    #
    # 1e9 IS THE POINT OF THE TEST. An absolute pivot threshold survives 1e3
    # through 1e6 and REFUSES a feasible problem at 1e9, so a test that stopped
    # below it passed on the routine it was written to convict.
    for factor in (1e3, 1e6, 1e9):
        try:
            scaled, _ = FIT.constrained_min([[x * factor for x in row] for row in a],
                                            [x * factor for x in b], cons)
        except FIT.FitError as why:
            check(f"scaling A and b together by {factor:.0e} leaves the minimiser alone",
                  False, f"refused a feasible problem: {why}")
            continue
        check(f"scaling A and b together by {factor:.0e} leaves the minimiser alone",
              all(abs(scaled[i] - base[i]) < 1e-6 * max(1.0, abs(base[i]))
                  for i in range(3)), (base, scaled))
    # AND THE LIMIT IS STATED RATHER THAN IMPLIED: at 1e12 the constraint rows
    # are cancelled away in double precision and the routine REFUSES. That is a
    # loud refusal, not a wrong answer, and it is what the claim is scoped to.
    try:
        FIT.constrained_min([[x * 1e12 for x in row] for row in a],
                            [x * 1e12 for x in b], cons)
        check("the invariance has a measured upper limit", False, "1e12 did not refuse")
    except FIT.FitError as why:
        check("beyond the measured range the routine REFUSES rather than answering",
              "no feasible point" in str(why), str(why))


def test_a_ONE_SIDED_regressor_is_refused_and_a_rank_test_would_not_see_it():
    """D-621's condition is one-sidedness, not degeneracy.

    A regressor that never changes sign leaves the normal matrix perfectly well
    conditioned, so a diagonal-is-zero test passes on exactly the case the
    finding is about. This pins the check against that.
    """
    rows = []
    for index in range(1, 50):
        row = {"a": [0] * 7, "b": [0] * 7, "label": 10 * index, "to_move": "p1"}
        # g1 and g2 take both signs; only g3 is one-sided, so the refusal has to
        # name g3 rather than fire on the first regressor it meets.
        row["a"][1], row["b"][1] = index, 50 - index
        row["a"][2], row["b"][2] = 50 - index, index
        row["b"][3] = index % 5          # g3 is negative or zero, never positive
        rows.append(row)
    diagonal = sum(FIT.signed(r)[2] ** 2 for r in rows)
    check("the one-sided regressor's normal-matrix diagonal is far from zero",
          diagonal > 100, diagonal)
    try:
        FIT.normal_equations(rows)
        check("a one-sided regressor is refused", False, "no exception raised")
    except FIT.FitError as why:
        check("a one-sided regressor is refused by name, citing the finding",
              "ONE sign" in str(why) and "g3" in str(why) and "D-621" in str(why), str(why))
    for row in rows:
        row["b"][3] = 0                  # now g3 is identically zero
    try:
        FIT.normal_equations(rows)
        check("a dead regressor is refused too", False, "no exception raised")
    except FIT.FitError as why:
        check("a dead regressor is refused too", "no sign" in str(why), str(why))


def test_the_tempo_term_is_fitted_so_it_is_not_absorbed_into_the_weights():
    """The model that ships has no constant, so a systematic offset in the
    labels can only reach it through the weights. Generated FROM a known
    offset, the pinned-intercept fit recovers the weights and the no-intercept
    fit does not."""
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        truth, tempo = [3, 20, 90, 400, 2000], 500
        synth_rows(path, truth, n=4000)
        rows = FIT.read_rows(path)
        for row in rows:
            # THE MOVER IS SYSTEMATICALLY BEHIND, as at a real turn boundary:
            # without that the offset has nothing to correlate with and the
            # no-intercept fit absorbs it harmlessly, so the test would pass on
            # data that cannot show the defect.
            behind = row["b"] if row["to_move"] == "p1" else row["a"]
            for k in (1, 2, 3):
                behind[k] += 6
            g = FIT.signed(row)
            row["label"] = sum(g[k] * truth[k] for k in range(3)) + tempo
        fitted, _ = FIT.select(rows)
        top, total = float(truth[2]), float(sum(truth[:3]))
        answer, _ = FIT.constrained_min(*FIT.tempo_normal_equations(fitted, top, total),
                                        FIT.tempo_constraints(top, total))
        check("the pinned-intercept fit recovers the generating first weight",
              abs(answer[0] - truth[0]) < 1e-6, answer)
        check("and recovers the offset it was given",
              abs(answer[1] - tempo) < 1e-6, answer[1])
        plain = FIT.solve(*FIT.normal_equations(fitted))
        check("while the no-intercept fit moves the weights instead",
              any(abs(plain[k] - truth[k]) > 1.0 for k in range(3)), plain)


CORPUS_HEADER = "# param label_go go nodes 400000\n"


def _corpus_record(game, turns, moves, key_full, to_move, kind, value):
    """One record in `labels_file.rs`'s column order, which is what extract reads."""
    return "\t".join([
        str(game), str(turns), moves, "-", "kp", key_full, to_move, kind, str(value),
        "-", "4", "400384", "0", "yes", "p1_win", "normal",
    ]) + "\n"


def _scratch_corpus(root):
    """A two-tranche corpus and the manifest that indexes it."""
    rows = [
        _corpus_record(0, 1, "0,0", "0,0:p1", "p2", "eval", 300),
        _corpus_record(0, 2, "0,0 1,0/2,0", "0,0:p1 1,0:p2 2,0:p2", "p1", "mate_in", 5),
        _corpus_record(0, 3, "0,0 1,0/2,0 0,1/0,2", "K3", "p2", "eval", -120),
    ]
    for index in (1, 2):
        path = pathlib.Path(root) / f"tranche-{index}"
        path.mkdir(parents=True, exist_ok=True)
        (path / "corpus.txt").write_text(CORPUS_HEADER + "".join(rows))
    manifest = pathlib.Path(root) / "manifest.txt"
    lines = ["# scratch manifest\n"]
    for index in (1, 2):
        for record, key in ((1, "0,0:p1"), (2, "0,0:p1 1,0:p2 2,0:p2"), (3, "K3")):
            lines.append(f"{index}\t{record}\t-\tkp\t{key}\t4\tp1_win\tnormal\n")
    manifest.write_text("".join(lines))
    return str(manifest), str(pathlib.Path(root) / "tranche-{}" / "corpus.txt")


def test_extract_is_driven_and_keeps_what_the_design_says_it_keeps():
    """`extract.py` writes the row file every other number here rests on, and
    nothing drove it. Its two load-bearing properties are the per-row join check
    and that mate rows are KEPT -- design section 4, stage 2."""
    import extract as EX
    with tempfile.TemporaryDirectory() as tmp:
        manifest, tranche = _scratch_corpus(tmp)
        out = f"{tmp}/rows.txt"
        written = EX.main(out, manifest, tranche)
        check("extract writes one row per manifest entry", written == 6, written)
        body = [l.rstrip("\n").split("\t") for l in open(out) if not l.startswith("#")]
        kinds = [r[14] for r in body]
        check("MATE ROWS ARE KEPT, which is the property the design registers",
              kinds.count("mate_in") == 2, kinds)
        check("and the score kinds come from the corpus, not from a default",
              set(kinds) == {"eval", "mate_in"}, set(kinds))
        one_stone = [r for r in body if r[12] == "300"][0]
        check("a lone stone's own-side one-window count reaches the row file",
              one_stone[0] == "18", one_stone[:6])
        # THE WHOLE COLUMN MAP, not the columns that happen to be looked at:
        # extract reads the corpus by INDEX, so an off-by-one anywhere in that
        # map writes a well-formed row carrying another column's value.
        check("every non-feature column is the corpus's own",
              one_stone[13:18] == ["p2", "eval", "4", "yes", "p1_win"], one_stone[13:18])
        check("and the key column is a digest of key_full, not key_full",
              len(one_stone[18]) == 64 and all(c in "0123456789abcdef" for c in one_stone[18]),
              one_stone[18])
        check("the header names every corpus digest it read",
              sum(1 for l in open(out) if l.startswith("# corpus ")) == 2)


def test_extract_REFUSES_a_join_that_addresses_the_wrong_record():
    """The mutant that must die: a join to the wrong record produces well-formed
    rows whose features belong to another position, and nothing downstream could
    tell."""
    import extract as EX
    with tempfile.TemporaryDirectory() as tmp:
        manifest, tranche = _scratch_corpus(tmp)
        text = pathlib.Path(manifest).read_text().replace("\tK3\t", "\tNOT-THE-KEY\t", 1)
        pathlib.Path(manifest).write_text(text)
        try:
            EX.main(f"{tmp}/rows.txt", manifest, tranche)
            check("a mismatched key_full is refused", False, "no exception raised")
        except SystemExit as why:
            check("a mismatched key_full is refused by name",
                  "key_full" in str(why) and "manifest says" in str(why), str(why))


def test_options_is_driven_and_every_pin_is_a_MINIMISER():
    """`options.py` regenerates the option matrix's own tables, so it produces
    recorded numbers and carries a test driving the shipped script.

    The property that matters is that each row is a constrained MINIMISER under
    its pin and not a rescale of a free solve — the defect the matrix was
    corrected for — so the pin is checked to hold EXACTLY in the real answer.
    """
    import options as OPT
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, [3, 20, 90, 400, 2000], n=4000)
        rows = FIT.read_rows(path)
        for row in rows:
            behind = row["b"] if row["to_move"] == "p1" else row["a"]
            for k in (1, 2, 3):
                behind[k] += 6
            g = FIT.signed(row)
            row["label"] = sum(g[k] * [3, 20, 90][k] for k in range(3)) + 500
        fitted, _ = FIT.select(rows)
        train, _ = FIT.split(fitted)
        committed = FIT.committed_table()
        for label, kind, index, value in OPT.PINS:
            quiet, tempo = OPT.solve_pinned(train, kind, index, value, committed)
            if kind == "index":
                check(f"pin '{label.strip()}' holds its entry exactly",
                      abs(quiet[index] - value) < 1e-9, (label, quiet))
                # HOLDING THE PIN IS WHAT A RESCALE DOES TOO, so it cannot tell a
                # minimiser from one — and a rescale of the free solve is exactly
                # the construction the matrix was corrected for. The defining
                # property of a least-squares answer is that its RESIDUAL is
                # orthogonal to every free direction, which a rescale violates.
                free = [k for k in range(3) if k != index]
                residuals = []
                for row in train:
                    g = FIT.signed(row)
                    residuals.append(row["label"] - tempo
                                     - sum(g[k] * quiet[k] for k in range(3)))
                scale = sum(abs(r) for r in residuals) + 1.0
                worse = all(
                    abs(sum(r * FIT.signed(row)[k]
                            for r, row in zip(residuals, train))) < 1e-6 * scale
                    for k in free
                ) and abs(sum(residuals)) < 1e-6 * scale
                check(f"pin '{label.strip()}' is a MINIMISER and not a rescale",
                      worse, (label, quiet))
            elif kind == "sum":
                check(f"pin '{label.strip()}' holds the sum exactly",
                      abs(sum(quiet) - value) < 1e-9, (label, quiet))
            elif kind == "both":
                check("the two-pin row holds the top entry AND the sum, which is "
                      "the whole reason it is on the matrix",
                      abs(quiet[2] - committed[2]) < 1e-9
                      and abs(sum(quiet) - sum(committed[:3])) < 1e-9, (label, quiet))
            else:
                check("the unpinned solve recovers the generating weights",
                      all(abs(quiet[k] - [3, 20, 90][k]) < 1e-6 for k in range(3)), quiet)
                check("and recovers the offset it was given", abs(tempo - 500) < 1e-6, tempo)
        done = subprocess.run([sys.executable, str(HERE / "options.py"), path],
                              capture_output=True, text=True)
        check("options.py exits 0", done.returncode == 0, done.stderr[-300:])
        check("and prints the exchange-rate columns the matrix turns on",
              "w3/w4" in done.stdout and "sum/w4" in done.stdout, done.stdout[-200:])


def test_fit_END_TO_END_ships_the_pinned_model_and_not_the_contrast():
    """`fit()` produces the phase's registered number and had no test.

    Three mutants inside it changed that number with every gate green: shipping
    the no-intercept CONTRAST instead of the answer, fitting on the VALIDATION
    slice, and skipping the schema rounding. These drive `fit()` itself.
    """
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        truth, tempo = [3, 20, 90, 400, 2000], 500
        synth_rows(path, truth, n=4000)
        rows = FIT.read_rows(path)
        for row in rows:
            behind = row["b"] if row["to_move"] == "p1" else row["a"]
            for k in (1, 2, 3):
                behind[k] += 6
            g = FIT.signed(row)
            row["label"] = sum(g[k] * truth[k] for k in range(3)) + tempo
            # THE TWO SLICES MUST DISAGREE, or fitting the wrong one is invisible:
            # on noiseless data every subset gives the same answer, which is how a
            # `fit on the validation slice` mutant survived.
            if row["key"][-1] in "01":
                row["label"] += 5000
        answer = FIT.fit(rows, truth)
        check("fit() recovers the generating weights under its own pins",
              all(abs(answer["quiet"][k] - truth[k]) < 1e-6 for k in range(3)),
              answer["quiet"])
        check("and the tempo term it discards", abs(answer["tempo"] - tempo) < 1e-6,
              answer["tempo"])
        # THE CONTRAST IS A DIFFERENT ANSWER, so shipping it is detectable.
        check("the no-intercept contrast is NOT the answer",
              any(abs(answer["no_intercept"][k] - answer["quiet"][k]) > 1.0
                  for k in range(3)), (answer["no_intercept"], answer["quiet"]))
        # FITTING ON THE VALIDATION SLICE gives a different answer on real data;
        # here the data is noiseless, so the split is pinned by its populations.
        check("the fit consumes the TRAIN slice and not the validation one",
              len(answer["train"]) > len(answer["val"]) * 5,
              (len(answer["train"]), len(answer["val"])))
        check("the assembled table is integers the schema can hold",
              all(isinstance(x, int) for x in answer["table"])
              and all(answer["table"][k + 1] > answer["table"][k] for k in range(4)),
              answer["table"])
        check("the reported populations are the ones select() actually applied",
              answer["counts"]["fitted"] == len(answer["train"]) + len(answer["val"]),
              answer["counts"])


def test_the_committed_candidate_satisfies_the_pins_it_is_registered_under():
    """The registered candidate is pinned by nothing but this.

    Editing `configs/eval_v0_quiet_fit_weights.toml` to any other legal table
    passed every gate. What defines the candidate is not its digits but the two
    pins the matrix selects it under, and those are checkable without the corpus.
    """
    committed = FIT.committed_table("configs/eval_v0_weights.toml")
    candidate = FIT.committed_table("configs/eval_v0_quiet_fit_weights.toml")
    check("the candidate holds the tactical entries verbatim (R6, D-622)",
          candidate[3:] == committed[3:], (candidate, committed))
    check("it holds the top quiet entry at the committed value",
          candidate[2] == committed[2], (candidate, committed))
    check("it holds the quiet SUM at the committed value",
          sum(candidate[:3]) == sum(committed[:3]), (candidate, committed))
    check("and it is not the committed table, or the SPRT would be a self-match",
          candidate != committed, candidate)


def test_read_rows_REFUSES_an_unknown_to_move_token():
    """Hard rule 3. Without this guard `signed()` reads any non-`p1` token as p2
    and returns a schema-legal, dominance-passing, WRONG table in silence."""
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, FIT.committed_table("configs/eval_v0_weights.toml"), n=8)
        text = pathlib.Path(path).read_text().replace("\tp1\t", "\tP1\t", 1)
        pathlib.Path(path).write_text(text)
        try:
            FIT.read_rows(path)
            check("an unknown to_move token is refused", False, "no exception raised")
        except FIT.FitError as why:
            check("an unknown to_move token is refused by name",
                  "to_move" in str(why) and "p1 and p2" in str(why), str(why))


def test_shipped_script_runs():
    """Drive fit.py as a program, the way a run does."""
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/rows.txt"
        synth_rows(path, [3, 20, 90, 400, 2000])
        done = subprocess.run([sys.executable, str(HERE / "fit.py"), path],
                              capture_output=True, text=True)
        check("fit.py exits 0", done.returncode == 0, done.stderr[-300:])
        check("fit.py reports the population of every filter clause",
              "dropped: score_kind" in done.stdout and "one_sided_window" in done.stdout,
              done.stdout[-400:])
        check("fit.py reports whether a gap constraint binds",
              "BINDING" in done.stdout, done.stdout[-300:])
        check("fit.py reports the tempo term it fits and discards",
              "tempo term" in done.stdout, done.stdout[-400:])
        check("fit.py reports the no-intercept solve as the CONTRAST",
              "CONTRAST" in done.stdout, done.stdout[-400:])
        check("fit.py reports the dominance check", "check: dominance" in done.stdout)
        check("fit.py names the pinned entries", "pinned" in done.stdout)


for test in (test_one_stone_features, test_turn_structure,
             test_dead_window_scores_zero,
             test_each_axis_is_pinned_separately,
             test_the_six_stone_window_is_spliced_at_the_decided_value,
             test_the_mover_alternates_per_TURN_and_three_turns_is_what_shows_it,
             test_per_side_counts_name_the_owner_the_signed_vector_cannot,
             test_the_filter_is_a_predicate_on_ownership,
             test_the_filter_reports_every_clause_it_applies,
             test_an_empty_filtered_population_is_refused_not_defaulted,
             test_fit_recovers_known_quiet_weights,
             test_constrained_fit_respects_the_schema,
             test_a_BOUND_is_an_active_set_member_and_not_a_clamp,
             test_rounding_refuses_at_BOTH_ends_and_the_floor_is_the_one_that_fires,
             test_the_active_set_enumeration_agrees_with_a_brute_force_grid,
             test_committed_weights_are_read_and_never_duplicated_in_code,
             test_dominance_is_checked_and_can_fail,
             test_rank_correlation_averages_tied_ranks,
             test_the_solve_is_invariant_under_the_labels_units,
             test_a_ONE_SIDED_regressor_is_refused_and_a_rank_test_would_not_see_it,
             test_the_tempo_term_is_fitted_so_it_is_not_absorbed_into_the_weights,
             test_extract_is_driven_and_keeps_what_the_design_says_it_keeps,
             test_extract_REFUSES_a_join_that_addresses_the_wrong_record,
             test_the_oracle_drives_the_engine_and_agrees,
             test_the_sample_takes_the_WHOLE_tactical_stratum_whatever_the_stride,
             test_the_oracle_REFUSES_an_unknown_to_move_token,
             test_the_oracle_REFUSES_a_sample_that_misses_the_registered_rule,
             test_the_oracle_FAILS_on_a_disagreeing_engine,
             test_fit_END_TO_END_ships_the_pinned_model_and_not_the_contrast,
             test_the_committed_candidate_satisfies_the_pins_it_is_registered_under,
             test_read_rows_REFUSES_an_unknown_to_move_token,
             test_options_is_driven_and_every_pin_is_a_MINIMISER,
             test_shipped_script_runs):
    print(test.__name__)
    test()

print()
if failures:
    print(f"test_texel: {len(failures)} FAILURE(S): {failures}")
    raise SystemExit(1)
print("test_texel: all checks passed")


def _census_line(entry, key, cols, att, deff="false"):
    """One `trigger_census` row in the shipped print format."""
    c = dict(zip(("turns", "mover_hot", "opp_hot", "mover_w1", "opp_w1",
                  "mover_l3", "opp_l3", "cover", "covers"), cols))
    return (f"trigger_census: row entry {entry} key {key} key_pos kp{key} "
            f"turns {c['turns']} mover_hot {c['mover_hot']} opp_hot {c['opp_hot']} "
            f"mover_w1 {c['mover_w1']} opp_w1 {c['opp_w1']} mover_l3 {c['mover_l3']} "
            f"opp_l3 {c['opp_l3']} cover {c['cover']} covers {c['covers']} "
            f"att_visits 10 att_proved {att} def_asked true def_visits 5 def_proved {deff}\n")


def test_census_classes_partition_matches_its_source():
    """The partition is checked against stage3_allocator_bound.py ITSELF.

    Pinning it against a second hard-coded copy would pass while the two
    drifted, which is the whole defect the pin exists to prevent.
    """
    import census_classes as CC
    source = pathlib.Path("tools/stage3_allocator_bound.py").read_text()
    body = source.split("COLUMNS = (", 1)[1].split(")", 1)[0]
    named = tuple(w.strip().strip('",\'') for w in body.split() if w.strip(' ,').startswith('"'))
    check("the class partition equals stage3_allocator_bound.py's own COLUMNS",
          CC.COLUMNS == named, (CC.COLUMNS, named))


def test_census_classes_counts_hermetically():
    """Driven on rows this test writes, so it cannot pass by finding no data."""
    import census_classes as CC
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/census.txt"
        cols_a = (0, 1, 2, 0, 0, 3, 4, "minimal", 1)
        cols_b = (1, 1, 2, 0, 0, 3, 4, "minimal", 1)
        with open(path, "w") as sink:
            # two roots; root 1 carries two keys in ONE class, root 2 one key
            # in a second class. So roots=2, classes=2, keys=3.
            sink.write(_census_line(1, "k1", cols_a, "true"))
            sink.write(_census_line(1, "k2", cols_a, "true"))
            sink.write(_census_line(2, "k3", cols_b, "true"))
            sink.write(_census_line(3, "k4", cols_a, "false", deff="true"))
        firings, keys, roots, classes, loss = CC.tally(path)
        check("every row is counted as a firing", firings == 4, firings)
        check("win keys are 3", len(keys) == 3, keys)
        check("win roots are 2", len(roots) == 2, roots)
        check("classes collapse the two same-column keys to 1, giving 2",
              len(classes) == 2, classes)
        check("the loss direction is separate and not in the win set",
              len(loss) == 1 and "k4" not in keys, (loss, keys))
        points = CC.curve(path, step=1)
        check("the curve is non-decreasing",
              all(b >= a for (_, a), (_, b) in zip(points, points[1:])), points)


def test_ordering_violation_is_refused_not_assumed():
    """classes <= keys is contingent (turns is root-relative), so a run that
    breaks it must be REFUSED rather than reported."""
    import census_classes as CC
    with tempfile.TemporaryDirectory() as tmp:
        path = f"{tmp}/census.txt"
        with open(path, "w") as sink:
            # ONE key appearing in two different classes -> classes(2) > keys(1)
            sink.write(_census_line(1, "k1", (0, 1, 2, 0, 0, 3, 4, "minimal", 1), "true"))
            sink.write(_census_line(1, "k1", (5, 1, 2, 0, 0, 3, 4, "minimal", 1), "true"))
        try:
            CC.tally(path)
            check("a classes > keys run is refused", False, "no exception raised")
        except CC.OrderingViolated as why:
            check("a classes > keys run is refused by name", "tightening" in str(why), str(why))


for extra in (test_census_classes_partition_matches_its_source,
              test_census_classes_counts_hermetically,
              test_ordering_violation_is_refused_not_assumed):
    print(extra.__name__)
    extra()
print()
if failures:
    print(f"test_texel: {len(failures)} FAILURE(S): {failures}")
    raise SystemExit(1)
print("test_texel: all checks passed (including census_classes)")
