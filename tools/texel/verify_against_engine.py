"""The oracle: the offline feature path must agree with the engine, exactly.

`features.py` re-implements the engine's window bookkeeping in another language.
A re-implementation that silently disagrees would produce a confident fit to the
wrong features, so nothing in `tools/texel/` is trusted until this has run.

THE SAMPLE IS REGISTERED, NOT DRAWN AT RUN TIME
(docs/experiments/wp22_phase1_design.md §5). An unstratified draw is nearly
blind to the top table entries — few positions carry a four- or five-stone
window at all — so the sample is the union of every position that carries one,
a stride draw across every tranche and every score kind, and a re-run under
weight tables that saturate the eval band on both sides.

Usage: tools/texel/verify_against_engine.py <static_eval binary> [stride]
"""

import pathlib
import subprocess
import sys
import tempfile

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import features as F

MANIFEST = "artifacts/arc3r_sweep_deduped_manifest.txt"
TRANCHE = "/home/tom/Projects/pistol-corpus/arc3r-sweep/tranche-{}/corpus.txt"
MOVES, KEY_FULL, TO_MOVE, SCORE_KIND = 2, 5, 6, 7
DEFAULT_STRIDE = 25

# Each must satisfy the schema (weights.rs): >= 1, strictly increasing, and
# every entry below the decided window's value. The second and third are here
# to force the clamp, which the committed table never reaches.
TABLES = {
    "committed": [2, 12, 60, 300, 1500],
    "clamp_low": [1, 10, 100, 2000, 15999],
    "clamp_high": [15995, 15996, 15997, 15998, 15999],
}


class OracleError(Exception):
    """A named refusal, and a single mismatch is one."""


def corpus_rows(manifest=MANIFEST, tranche=TRANCHE):
    """Every deduped position, as (key_full, moves, to_move, score_kind)."""
    wanted = {}
    with open(manifest) as handle:
        for line in handle:
            if line.startswith("#"):
                continue
            row = line.rstrip("\n").split("\t")
            wanted.setdefault(int(row[0]), set()).add((int(row[1]), row[4]))
    for index in sorted(wanted):
        with open(tranche.format(index)) as handle:
            body = [l.rstrip("\n").split("\t") for l in handle if not l.startswith("#")]
        for record_number, key_full in sorted(wanted[index]):
            rec = body[record_number - 1]
            if rec[KEY_FULL] != key_full:
                raise OracleError(
                    f"oracle: corpus {index} record {record_number} key_full mismatch")
            if rec[TO_MOVE] not in ("p1", "p2"):
                raise OracleError(
                    f"oracle: corpus {index} record {record_number} has to_move "
                    f"{rec[TO_MOVE]!r}; reading anything but p1 or p2 as p2 silently "
                    "negates the position's value")
            yield index, rec[KEY_FULL], rec[MOVES], rec[TO_MOVE], rec[SCORE_KIND]


def sample(rows, stride=DEFAULT_STRIDE):
    """The registered sample: the whole tactical stratum, plus a stride draw.

    NOTHING IS DEDUPED HERE. 2 369 manifest rows share a `key_full` with another
    (387 groups), so folding on it drops rows the registered sample names -- and
    the sample is over corpus RECORDS, which is the population the extractor
    walks.
    """
    chosen, passing = [], 0
    for position, (tranche, key, moves, to_move, kind) in enumerate(rows):
        a, b = F.per_side_counts(F.stones_of(moves))
        tactical = any(a[k] or b[k] for k in (4, 5, 6))
        if tactical or position % stride == 0:
            passing += 1
            chosen.append((key, moves, to_move, kind, tactical, tranche))
    return chosen, passing


def coverage(chosen):
    """What the drawn sample actually spans, as the registered rule names it."""
    return {
        "sampled": len(chosen),
        "tactical": sum(1 for c in chosen if c[4]),
        "tranches": sorted({c[5] for c in chosen}),
        "kinds": sorted({c[3] for c in chosen}),
    }


def check_sample_rule(chosen, tranches, kinds, tables=None, passing=None):
    """REFUSE a sample that does not meet design section 5's registered rule.

    Reporting the counts is not enough: the rule's two clauses -- the whole
    tactical stratum, and a stride draw across every tranche and every score
    kind -- are both inert on a small sample, so a mutation that dropped either
    left every check green. This is what makes them fire.
    """
    span = coverage(chosen)
    if len(span["tranches"]) != tranches:
        raise OracleError(
            f"oracle: the sample spans {len(span['tranches'])} tranche(s), "
            f"and the registered rule names {tranches}")
    if sorted(kinds) != span["kinds"]:
        raise OracleError(
            f"oracle: the sample spans score kinds {span['kinds']}, "
            f"and the registered rule names {sorted(kinds)}")
    if not span["tactical"]:
        raise OracleError(
            "oracle: the sample holds no position from the tactical stratum, which "
            "is the stratum the phase's own finding is about")
    if tables is not None:
        clamps = [name for name in tables if name.startswith("clamp")]
        if len(clamps) < 2:
            raise OracleError(
                f"oracle: the registered rule names a table saturating the band on "
                f"EACH side and {len(clamps)} clamp table(s) are present; dropping "
                "one removes a saturation regime the other does not reach")
    if passing is not None and len(chosen) != passing:
        raise OracleError(
            f"oracle: {passing} position(s) satisfy the registered predicate and "
            f"{len(chosen)} were taken, so the draw was folded. **A FOLD IS DETECTED "
            "AGAINST THE SOURCE, NOT AGAINST THE DRAW**: 2 369 corpus rows share a "
            "`key_full` and section 5 registers that nothing is deduped by it, so "
            "requiring the draw's keys to be DISTINCT asserts the opposite of the "
            "rule and refuses the registered workload — which is what it did")
    return span


def write_weights(path, table):
    lines = ["schema_version = 1", 'backend = "handcrafted_v0"', "[table]"]
    lines += [f"{k} = {table[k - 1]}" for k in range(1, 6)]
    pathlib.Path(path).write_text("\n".join(lines) + "\n")


def engine_values(binary, weights_path, chosen, work):
    """The engine's own `HandcraftedV0::value` for every sampled position."""
    fixture = pathlib.Path(work) / "fixture.txt"
    # The corpus spells the empty board "-"; the engine's fixture reader spells
    # it as no turns at all, and a lone "-" is a stone token it refuses.
    fixture.write_text("".join(
        "start moves\n" if moves == "-" else f"start moves {moves}\n"
        for _, moves, _, _, _, _ in chosen))
    done = subprocess.run([binary, weights_path, str(fixture)],
                          capture_output=True, text=True, check=False)
    if done.returncode != 0:
        raise OracleError(f"oracle: static_eval refused: {done.stderr.strip()}")
    out = done.stdout.split("\n")
    values = [line.split() for line in out if line.strip()]
    if len(values) != len(chosen):
        raise OracleError(
            f"oracle: engine answered {len(values)} of {len(chosen)} positions")
    return values


def run(binary, stride=DEFAULT_STRIDE, rows=None, tranches=None, kinds=None):
    chosen, passing = sample(rows if rows is not None else corpus_rows(), stride)
    span = coverage(chosen)
    if tranches is not None:
        check_sample_rule(chosen, tranches, kinds, TABLES, passing)
    tactical = span["tactical"]
    print(f"oracle: draw = the whole tactical stratum plus every {stride}th position; "
          f"tranches {len(span['tranches'])}, score kinds {','.join(span['kinds'])}")
    report = {"sampled": len(chosen), "tactical_stratum": tactical,
              "coverage": span, "tables": {}}
    with tempfile.TemporaryDirectory() as work:
        for name, table in TABLES.items():
            weights_path = f"{work}/{name}.toml"
            write_weights(weights_path, table)
            answered = engine_values(binary, weights_path, chosen, work)
            disagreements, saturating = 0, 0
            for (key, moves, to_move, _kind, _t, _n), got in zip(chosen, answered):
                stones = F.stones_of(moves)
                f = F.features(stones)
                mine = F.value(f, table, 0 if to_move == "p1" else 1)
                raw = sum(f[k] * table[k - 1] for k in range(1, 6)) + f[6] * F.EVAL_MAX
                saturating += abs(raw) >= F.EVAL_MAX
                if int(got[0]) != mine or got[1] != to_move:
                    disagreements += 1
                    if disagreements == 1:
                        first = f"{key[:16]} engine {got[0]}/{got[1]} offline {mine}/{to_move}"
            report["tables"][name] = {"disagreements": disagreements,
                                      "saturating": saturating}
            print(f"oracle: table {name:<11} sampled {len(chosen)} "
                  f"saturating {saturating:>6} disagreements {disagreements}")
            if disagreements:
                raise OracleError(f"oracle: {disagreements} disagreement(s), first {first}")
    # THE THIRD CLAUSE OF THE REGISTERED RULE, checked rather than assumed: at
    # least one table must drive the value onto the band edge, or the clamp --
    # the one place the offline path and the engine could differ without either
    # being obviously wrong -- is never exercised at all.
    saturating = [name for name, seen in report["tables"].items() if seen["saturating"]]
    if not saturating:
        raise OracleError(
            "oracle: no registered table saturated the eval band on any sampled "
            "position, so the clamp was never exercised and the run checks less "
            "than the rule registers")
    print(f"oracle: {len(chosen)} position(s), {tactical} in the tactical stratum, "
          f"{len(TABLES)} weight tables, {len(saturating)} of them saturating, "
          "0 disagreements")
    return report


if __name__ == "__main__":
    if len(sys.argv) < 2:
        raise SystemExit("usage: verify_against_engine.py <static_eval binary> [stride]")
    # The registered rule, enforced on the corpus path: all sixteen tranches and
    # all three score kinds (docs/experiments/wp22_phase1_design.md section 5).
    run(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else DEFAULT_STRIDE,
        tranches=16, kinds=("eval", "mate_in", "mated_in"))
