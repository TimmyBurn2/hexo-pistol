"""Behaviour tests for the corpus walk: the census and the seed pilot.

Both are driven as the SHIPPED scripts over a synthetic corpus this file
writes, with a control run in each case so a pass cannot come from a routine
that reports the same numbers whatever it reads (tools/SHELL_CHECKLIST.md
item 10). The labelled corpus lives outside the repository (D-636), which is
why both scripts take their manifest and tranche template from argv.

Usage: test_census.py
Exit:  0 every check passed; 1 one did not.
"""

import pathlib
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import hexenum as E

SCRATCH = HERE.parents[1] / "artifacts" / "pistol-testscratch-hexenum"
FAILURES = []


def check(name, condition, detail=""):
    if condition:
        print(f"  ok: {name}")
    else:
        FAILURES.append(f"{name}: {detail}")
        print(f"  FAIL: {name}: {detail}")


CORPUS_HEADER = "# synthetic corpus for test_hex_enum.py\n"


def corpus_row(game, number, moves, to_move, kind, value):
    return "\t".join([str(game), str(number), moves, "-", f"k{number}", f"kf{number}",
                      to_move, kind, str(value), "-", "3", "1000", "0", "no",
                      "p1_win", "normal"])


def write_corpus(directory, rows):
    directory.mkdir(parents=True, exist_ok=True)
    manifest = directory / "manifest.txt"
    body = [CORPUS_HEADER]
    lines = ["# synthetic manifest\n"]
    for index, row in enumerate(rows, start=1):
        body.append(row + "\n")
        lines.append(f"1\t{index}\t-\t-\tkf{index}\t3\tp1_win\tnormal\n")
    (directory / "corpus-1.txt").write_text("".join(body))
    manifest.write_text("".join(lines))
    return manifest, str(directory / "corpus-{}.txt")


def run_census(directory, rows, tag):
    manifest, tranche = write_corpus(directory, rows)
    out = directory / f"{tag}.txt"
    result = subprocess.run(
        [sys.executable, str(HERE / "census.py"), str(out), "9",
         "--manifest", str(manifest), "--tranche", tranche, "--legal-stride", "1"],
        capture_output=True, text=True)
    if result.returncode != 0:
        return None, result
    return out.read_text(), result


def the_census_keeps_only_quiet_eval_rows_and_both_terms_add_up():
    SCRATCH.mkdir(parents=True, exist_ok=True)
    # p1 reaches five in a row on axis (1,0), so R6's filter drops this row.
    live = "0,0 10,10/11,11 1,0/2,0 12,12/13,13 3,0/4,0"
    rows = [
        corpus_row(0, 1, "0,0 1,0/-1,0 0,1/0,-1", "p1", "eval", 120),
        corpus_row(0, 2, "0,0 1,0/-1,0 0,1/0,-1", "p2", "mate_in", 3),
        corpus_row(1, 3, live, "p2", "eval", -400),
        corpus_row(1, 4, "0,0 2,2/-2,-2 4,4/-4,-4", "p1", "eval", -60),
    ]
    body, result = run_census(SCRATCH / "mixed", rows, "mixed")
    if body is None:
        check("the census runs over a synthetic corpus", False, result.stderr[-400:])
        return
    walked = next(l for l in body.splitlines() if l.startswith("quiet eval positions"))
    check("the census skips the mate row and the tactical row",
          walked.endswith(" 2"), f"got {walked!r}")
    bad = []
    for line in body.splitlines():
        parts = line.split("\t")
        if len(parts) == 9 and parts[0] in ("window", "code"):
            within, between, total = (float(parts[i]) for i in (4, 5, 6))
            if abs(within + between - total) > max(1e-3, 1e-9 * total):
                bad.append(line)
    check("every reported partition satisfies the law of total variance",
          not bad and len(body.splitlines()) > 10, f"{bad[:2]}")


def a_corpus_of_one_label_has_no_between_class_variance():
    """The control: without it, a between term could be anything at all."""
    rows = [corpus_row(0, 1, "0,0 1,0/-1,0 0,1/0,-1", "p1", "eval", 77),
            corpus_row(1, 2, "0,0 2,2/-2,-2 4,4/-4,-4", "p2", "eval", 77)]
    body, result = run_census(SCRATCH / "flat", rows, "flat")
    if body is None:
        check("the control census runs", False, result.stderr[-400:])
        return
    worst = 0.0
    for line in body.splitlines():
        parts = line.split("\t")
        if len(parts) == 9 and parts[0] in ("window", "code"):
            worst = max(worst, abs(float(parts[5])))
    check("one label everywhere gives zero between-class variance", worst < 1e-9,
          f"largest between term {worst}")


def run_pilot(directory, rows, extra=()):
    manifest, tranche = write_corpus(directory, rows)
    out = directory / "pilot.txt"
    result = subprocess.run(
        [sys.executable, str(HERE / "seed_pilot.py"), str(out), "9", "2",
         "--seeds", "2", "--manifest", str(manifest), "--tranche", tranche, *extra],
        capture_output=True, text=True)
    return out, result


LINE_LABEL = 500


def spread_corpus(games):
    """One quiet position per game, in two shapes whose labels differ by sign.

    Half the games put the mover's three stones in a row and half scatter them,
    so the two halves have very different window-code counts and a fit that
    reads its features at all can separate them. A corpus of noise would let a
    solver returning zeros pass the control that follows.
    """
    rows = []
    for game in range(games):
        far = 100 + 4 * game
        if game % 2:
            moves = f"0,0 {far},{far}/{far + 1},{far + 1} 1,0/2,0"
            label = LINE_LABEL
        else:
            moves = f"0,0 {far},{far}/{far + 1},{far + 1} 0,20/20,0"
            label = -LINE_LABEL
        rows.append(corpus_row(game, game + 1, moves, "p1", "eval", label))
    return rows


def the_pilot_refuses_a_corpus_it_cannot_split():
    _out, result = run_pilot(SCRATCH / "tiny", spread_corpus(3))
    check("a corpus of fewer games than the holdout is refused by name",
          result.returncode != 0 and "cannot make a 1-in-8 split" in result.stderr,
          f"rc {result.returncode}, stderr {result.stderr[-200:]!r}")


def the_pilot_is_deterministic_at_a_fixed_seed():
    first, result = run_pilot(SCRATCH / "det", spread_corpus(24))
    if result.returncode != 0:
        check("the pilot runs on a splittable corpus", False, result.stderr[-400:])
        return
    body = first.read_text()
    _second, again = run_pilot(SCRATCH / "det", spread_corpus(24))
    check("two runs at the same seeds agree byte for byte",
          again.returncode == 0 and first.read_text() == body,
          "the second run differs")


def a_fit_beats_the_labels_own_variance_on_its_training_games():
    """The control: a solver returning zeros would pass every check above."""
    rows = spread_corpus(24)
    # The trivial model is `w = 0`, whose objective is the labels' mean square;
    # a solver that returned zeros would match it exactly.
    trivial = LINE_LABEL * LINE_LABEL
    out, result = run_pilot(SCRATCH / "fitq", rows)
    if result.returncode != 0:
        check("the control pilot runs", False, result.stderr[-400:])
        return
    trains = [float(l.split("\t")[3]) for l in out.read_text().splitlines()
              if l[:1].isdigit() and len(l.split("\t")) == 5 and "SINGULAR" not in l]
    check("a fitted row beats the zero table on its own training games",
          trains and max(trains) < 0.5 * trivial,
          f"train MSEs {trains} against the zero table's {trivial}")


def main():
    for test in (the_census_keeps_only_quiet_eval_rows_and_both_terms_add_up,
                 a_corpus_of_one_label_has_no_between_class_variance,
                 the_pilot_refuses_a_corpus_it_cannot_split,
                 the_pilot_is_deterministic_at_a_fixed_seed,
                 a_fit_beats_the_labels_own_variance_on_its_training_games):
        print(test.__name__)
        test()
    if FAILURES:
        print(f"\n{pathlib.Path(__file__).name}: {len(FAILURES)} FAILED")
        for line in FAILURES:
            print(f"  {line}")
        return 1
    print(f"\n{pathlib.Path(__file__).name}: all passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
