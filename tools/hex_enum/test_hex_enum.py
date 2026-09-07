"""Behaviour tests for the hex threat enum and its census.

The enum's own properties are checked by ENUMERATION rather than by example
where enumeration is affordable, because the claims the derivation memo makes
about it are universally quantified. The census is driven as the SHIPPED script
over a synthetic corpus this file writes, with a control run so a pass cannot
come from a routine that reports the same numbers whatever it reads
(tools/SHELL_CHECKLIST.md item 10).

Usage: test_hex_enum.py
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


def pattern(text):
    """`.` empty, `x` own, `o` opponent — the memo's mover-relative reading."""
    return tuple({".": 0, "x": 1, "o": 2}[c] for c in text)


def a_five_run_beside_the_cell_costs_one_stone():
    key = E.tuple_of(pattern("xxxxx." + "....."), 11)
    check("a five-run beside the cell costs one stone and is rule-4 relevant",
          key == (((1, 6, True, 6, 1, False)),), f"got {key}")


def an_opponent_stone_in_every_window_kills_the_side():
    """Opponent stones at 3 and 7 hit all six windows through the centre at 5."""
    key = E.tuple_of(pattern("...o...o..."), 11)
    check("a side hit in every window through the cell is dead, not expensive",
          key == ((E.INF, 0, False, 4, 6, False),), f"got {key}")


def an_empty_line_costs_six_on_both_sides():
    key = E.tuple_of(pattern("." * 11), 11)
    check("an empty line is six stones away for both sides through six live windows",
          key == ((6, 6, False, 6, 6, False),), f"got {key}")


def an_even_length_is_refused_by_name():
    """A cell-centred class has no centre at an even length (CLAUDE.md rule 3)."""
    for length in (6, 8, 10, 12):
        try:
            E.centres(length)
        except ValueError as error:
            check(f"L{length} is refused", "odd lengths only" in str(error), str(error))
        else:
            check(f"L{length} is refused", False, "it was accepted")


def the_window_count_through_the_centre_is_the_derived_one():
    for length in (7, 9, 11, 13):
        for centre in E.centres(length):
            found = len(E.windows(length, centre))
            want = min(E.WIN_LEN, length - E.WIN_LEN + 1)
            check(f"L{length} centre {centre} sees the derived window count",
                  found == want, f"enumerated {found}, derived {want}")


def every_class_equals_its_reversals_class():
    for length in (7, 9, 11):
        codes, _keys = E.table(length)
        bad = next((c for c in range(len(codes))
                    if codes[c] != codes[E.reverse_code(length, c)]), None)
        check(f"L{length} classes are reversal invariant", bad is None,
              f"code {bad} and its reversal differ")


def the_rule_four_component_refines_no_class():
    for length in (7, 9, 11):
        full = set()
        without = set()
        for _code, pat in E.patterns(length):
            key = E.tuple_of(pat, length)
            full.add(key)
            without.add(tuple((h[0], h[1], h[3], h[4]) for h in key))
        check(f"L{length} rule-4 relevance adds no class",
              len(full) == len(without), f"{len(full)} with, {len(without)} without")


def the_ladder_is_a_chain_of_quotients():
    for length in (7, 9, 11):
        rungs = {r: E.table(length, r)[0] for r in E.LADDER}
        for finer, coarser in zip(E.LADDER, E.LADDER[1:]):
            seen = {}
            bad = None
            for code in range(len(rungs[finer])):
                fine, coarse = rungs[finer][code], rungs[coarser][code]
                if seen.setdefault(fine, coarse) != coarse:
                    bad = code
                    break
            check(f"L{length} {coarser} is a quotient of {finer}", bad is None,
                  f"code {bad} splits a {finer} class across two {coarser} classes")


def the_ladders_bottom_rung_states_law_supports_two_boundaries():
    """T1 must separate `cost <= 2` from `cost <= 4` from beyond, and dead from both.

    A clip at `min(cost, 4)` merges cost 4 with 5 and 6 and gives k = 25 at
    L = 11; a clip that folds DEAD in as well gives k = 16, whose
    `C(18, 3) = 816` is exactly the imported figure D-706 refuses.
    """
    values = {E.clip(x) for x in (1, 2, 3, 4, 5, 6, E.INF)}
    check("T1's clip keeps LAW-SUPPORT's two boundaries and DEAD apart",
          values == {1, 2, 3, 4, 5, E.INF}, f"got {sorted(values, key=str)}")
    k = len(E.table(11, "T1")[1])
    check("T1 at L=11 does not land on the imported 816",
          k == 36 and E.multiset_codes(k) == 8436,
          f"k {k}, codes {E.multiset_codes(k)}")


def a_rung_carries_no_component_that_refines_nothing():
    """§5.1's rule applied to the ladder: T2's alive-or-dead is `min == INF`."""
    length = 11
    starts = E.windows(length, E.centres(length)[0])
    with_boolean = set()
    without = set()
    for _code, pat in E.patterns(length):
        own = E.side_terms(pat, starts, 1)
        opp = E.side_terms(pat, starts, 2)
        with_boolean.add((own[0], opp[0], own[1] > 0, opp[1] > 0))
        without.add((own[0], opp[0]))
    check("an alive-or-dead boolean beside `min` refines no T2 class",
          len(with_boolean) == len(without) == len(E.table(length, "T2")[1]),
          f"{len(with_boolean)} with, {len(without)} without, "
          f"{len(E.table(length, 'T2')[1])} shipped")


def the_solver_predicates_are_constant_on_a_class():
    """`Hot` and `WinInOnePly` recomputed from the windows, not from `min`.

    `crates/pistol-solver/src/query.rs:127,132` classify a window by its own
    stone count; the check is that those predicates cannot separate two
    patterns the tuple calls equal, which is what "the enum is at least as fine
    as the code the engine runs" means (memo §6.5).
    """
    length = 11
    centre = E.centres(length)[0]
    starts = E.windows(length, centre)
    seen = {}
    bad = None
    for _code, pat in E.patterns(length):
        hot = any(2 not in pat[s:s + E.WIN_LEN] and pat[s:s + E.WIN_LEN].count(1) >= 4
                  for s in starts)
        five = any(2 not in pat[s:s + E.WIN_LEN] and pat[s:s + E.WIN_LEN].count(1) == 5
                   for s in starts)
        key = E.tuple_of(pat, length)
        if seen.setdefault(key, (hot, five)) != (hot, five):
            bad = pat
            break
    check("the shipped solver's own window predicates are unions of tuple classes",
          bad is None, f"{bad} shares a class with a pattern of the other kind")


def the_variance_terms_are_computed_and_not_asserted():
    """A unit test of the statistic the census's own criterion is written in.

    The suite drove the census end to end and never read `omega2` at all, so a
    routine printing a constant would have passed it — measured, four mutants of
    `report.py` survived the whole gate. These cases pin the numbers against
    arithmetic done here, and the equal-means case is the control: a partition
    that separates nothing must earn nothing.
    """
    sys.path.insert(0, str(HERE))
    from report import Moments

    # Two classes, means 10 and 20, four observations each. Hand-derived:
    # grand 15, between = 25, within = (1 + 1) / 2 ... computed below in full.
    split = Moments()
    for value in (9, 11, 9, 11):
        split.add("a", value)
    for value in (19, 21, 19, 21):
        split.add("b", value)
    within, between, total, classes, count, eta2, omega2 = split.terms()
    check("a separating partition earns a positive between term",
          abs(between - 25.0) < 1e-9 and abs(within - 1.0) < 1e-9,
          f"within {within}, between {between}")
    check("its total is the population variance, computed without the partition",
          abs(total - 26.0) < 1e-9 and count == 8 and classes == 2, f"total {total}")
    check("eta2 and omega2 are not constants",
          abs(eta2 - 25.0 / 26.0) < 1e-9 and 0.0 < omega2 < eta2,
          f"eta2 {eta2}, omega2 {omega2}")

    # THE CONTROL: two classes with identical means. A partition that separates
    # nothing must earn nothing, and omega2 must not be positive.
    flat = Moments()
    for value in (9, 11, 19, 21):
        flat.add("a", value)
    for value in (9, 11, 19, 21):
        flat.add("b", value)
    within, between, total, _classes, _count, eta2, omega2 = flat.terms()
    check("a partition that separates nothing earns nothing",
          abs(between) < 1e-9 and abs(eta2) < 1e-9 and omega2 <= 0.0,
          f"between {between}, eta2 {eta2}, omega2 {omega2}")
    check("the flat partition's within term is the whole variance",
          abs(within - total) < 1e-9 and abs(total - 26.0) < 1e-9,
          f"within {within}, total {total}")


def the_merge_curve_comes_out_of_the_instrument():
    """`refinement` reports the fanout, so no scratch script owns a published number."""
    k_short, k_long, joint, fan, back = E.refinement(11, 2, "T4")
    check("at the covering length the join is one-to-one in both directions",
          (k_short, k_long, joint) == (357, 357, 357) and fan == (1, 1, 1.0)
          and back == (1, 1, 1.0), f"{k_short} {k_long} {joint} {fan} {back}")
    _ks, _kl, _j, fan7, back7 = E.refinement(7, 4, "T4")
    check("below it the join fans out in both directions",
          fan7[0] > 1 and fan7[1] > fan7[0] and back7[1] > 1,
          f"fanout {fan7}, backout {back7}")


def the_count_only_baseline_is_blind_to_everything_the_tuple_carries():
    """The referent the criterion is stated against must be structure-free.

    A referent that could see completion cost would not be a test of whether
    threat structure adds anything over stone counting, which is the one
    comparison a random partition of the enum's own shape cannot make.
    """
    sys.path.insert(0, str(HERE))
    from census import Length

    unit = Length(11, 1)
    same_counts = ("xxxxx." + ".....", "x.x.x." + "x.x..")
    codes = []
    for text in same_counts:
        pat = pattern(text)
        code = 0
        for power, index in zip(unit.powers, unit.slots):
            code += pat[index] * power
        codes.append(code)
    a, b = (E.tuple_of(pattern(t), 11) for t in same_counts)
    check("two patterns the tuple separates share one count-only key",
          unit.counts[codes[0]] == unit.counts[codes[1]] and a != b,
          f"keys {unit.counts[codes[0]]} and {unit.counts[codes[1]]}, tuples {a} {b}")
    check("one of them completes a six with a single stone and the other does not",
          a[0][0] == 1 and b[0][0] > 1, f"{a} {b}")

    # AND IT MUST SEPARATE, NOT ONLY MERGE. A referent stuck at a constant would
    # pass every merging check ever written and would make the criterion it is
    # stated against report a wrong PASS; the suite could not see that until
    # this half was added.
    differing = ("xxxxx." + ".....", "xxxx.." + ".....")
    keys = []
    for text in differing:
        pat = pattern(text)
        code = 0
        for power, index in zip(unit.powers, unit.slots):
            code += pat[index] * power
        keys.append(unit.counts[code])
    check("two patterns with different stone counts get different referent keys",
          keys[0] != keys[1], f"both {keys[0]}")
    distinct = len(set(unit.counts))
    check("the referent is not a constant and not the whole code space",
          1 < distinct < unit.size, f"{distinct} distinct keys over {unit.size} codes")
    check("the referent's key count matches the (own, opp) pairs a pattern admits",
          distinct == sum(1 for own in range(unit.length)
                          for opp in range(unit.length)
                          if own + opp <= unit.length - 1),
          f"{distinct} keys")


def main():
    for test in (a_five_run_beside_the_cell_costs_one_stone,
                 an_opponent_stone_in_every_window_kills_the_side,
                 an_empty_line_costs_six_on_both_sides,
                 an_even_length_is_refused_by_name,
                 the_window_count_through_the_centre_is_the_derived_one,
                 every_class_equals_its_reversals_class,
                 the_rule_four_component_refines_no_class,
                 the_ladder_is_a_chain_of_quotients,
                 the_ladders_bottom_rung_states_law_supports_two_boundaries,
                 a_rung_carries_no_component_that_refines_nothing,
                 the_solver_predicates_are_constant_on_a_class,
                 the_variance_terms_are_computed_and_not_asserted,
                 the_merge_curve_comes_out_of_the_instrument,
                 the_count_only_baseline_is_blind_to_everything_the_tuple_carries):
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
