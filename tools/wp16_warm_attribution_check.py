"""WP-1.6 §7A.1 Criterion 1'' — warm-replay attribution, checked end to end.

  usage: tools/wp16_warm_attribution_check.py <report> <replay> <engine-binary>

THIS IS AN INSTRUMENT. It produces the verdict `docs/experiments/wp16_sprt_prereg.md`
registers as its Criterion 1'', so CLAUDE.md names it there WITH ITS REVISION and a
change to it reopens that document's review exactly as an amendment would. It is
covered by `crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs`, which drives
this file — the shipped one — against documents it builds and against corrupted copies
of them, with a control run so a pass cannot come from a checker that refuses
everything (tools/SHELL_CHECKLIST.md item 10).

`tools/wp15b_attribution_check.py` IS NOT TOUCHED. It stays exactly as WP-1.5b's own
closed reviews left it, and every historical report that cites it keeps meaning what it
meant. This is a NEW, separately-named instrument, and links 1b and 1c below are carried
across from it deliberately unchanged — they were independently re-verified in that
document's own review rounds and re-deriving them would be re-opening settled work.

WHAT IS NEW, AND WHY. WP-1.5b's link 1a could replay only the two turns after the book,
because a fresh subprocess is COLD and a live engine's transposition table is WARM:
`crates/pistol-engine/src/instance.rs`'s `set_position` never clears the searcher and
only `new_game` does, so D-383 MEASURED a cold replay of any later turn disagreeing with
what the same engine, played live, actually answered. A game whose two checked turns did
not discriminate between the seats was therefore VACUOUS, and D-401's governed run came
out 31.2% vacuous — enough that adversarially reassigning the vacuous pairs moved the
verdict, which is what stopped that run being a measurement.

`arena --replay` removes the ceiling rather than raising it. It spawns both seats exactly
as a game does, feeds the RECORDED move list, and asks each seat at every one of its own
turns, so every engine sees precisely the exchanges it saw when the report was written
and its table is in precisely the state it was in. This file is the statistics layer over
what that pass found. It speaks no wire protocol of its own EXCEPT the one cold probe
below, which is named, owned here, and deliberately outside the warm chain.

THE THREE THINGS THIS CHECKS, each with a referent that does not share the suspect path:

  W   WARM REPLAY -> LABEL.  Every game's credited engine, re-driven warm, must answer
      every move the report attributes to it. The referent is the engine itself. A
      disagreement is classified, never left standing: ONE cold probe of the OTHER
      engine's config at the same prefix and the same budget settles it.
        - the other engine's answer IS the recorded move -> CONFIRMED INVERSION, the
          report's seats are the wrong way round (clause (a) fails).
        - neither engine's answer explains the recorded move -> DETERMINISM VIOLATION.
          Instrument mode guarantees an engine answers the same thing to the same input
          (CLAUDE.md rule 4); this is that guarantee failing. It is bigger than this WP,
          it exits on its own distinct code, and it is never folded into a FAIL count.
      Node counts are checked alongside: a clean, non-forfeit game's replay asks for the
      same searches in the same order at the same budget, so it must spend the same
      nodes. Equal moves with unequal nodes is the same guarantee failing more quietly.

  1b  MOVES -> RESULT.  Game rule 3: turn 1 is one stone, every later turn two, and a win
      completes on a placed stone, so the winner played the LAST turn and `result p1_win`
      holds exactly when the turn count is odd. The referent is the recorded move list
      read against a pinned game rule. Carried from WP-1.5b unchanged.

  1c  RESULT -> SCORE -> VERDICT.  Seat A's score is rebuilt from the `game` records and
      required to agree with `counts`, every `pair … bucket/score_a` and the pentanomial,
      all of which come through `GameRecord::score_a`, a disjoint function. The referent
      is the other code path. Carried from WP-1.5b unchanged.

CRITERION 1'', quoted verbatim from `docs/experiments/wp16_warm_replay_design.md` §4
point 4: "A report is a measurement iff (a) zero divergence-confirmed inversions — every
divergence found in point 2 above resolves to either 'no divergence' or 'confirmed
inversion' (the other-engine match case), never left unclassified — and (b) every
NON-INERT pair (point 3's exclusion, forfeits always non-inert) is directly attributed by
first divergence. A DETERMINISM VIOLATION (point 2's other branch) is checked FIRST and,
if found anywhere, stops the whole evaluation before (a)/(b) are even asked, per its own
exit code."

CLAUSE (b) ON A CLEAN REPORT — the satisfaction condition, written out, because a clause
with no satisfaction condition for the ordinary case is a clause that passes by silence.
There is no "first divergence" on a report with none, so what covers such a report is
point 3's theorem RUN IN REVERSE:

  Take any pair. Its two games share a book prefix and swap which label sits in which
  seat.

  - If their recorded move lists are IDENTICAL and neither game forfeited, the pair is
    INERT. Both games' credited engines warm-replayed every move, so the two engines are
    behaviourally indistinguishable at every position either game reached; swapping the
    labels could not have produced a different board at any ply, hence not a different
    result. Whichever PLAYER INDEX wins one game wins the other, and the two games swap
    which LABEL holds that index, so the pair is a FORCED 1-1 split — bucket p2, whichever
    engine is actually stronger. This file ASSERTS that bucket rather than assuming it:
    an inert pair recorded at anything but p2 contradicts the theorem, and that is a
    finding.

  - If their move lists DIFFER, let t be the first turn they differ at. Both games agree
    up to t, so the board at t is the same in both, and the mover at t is the same PLAYER
    INDEX in both — and that index's occupant has, in both games, searched exactly the
    same prefixes, so its warm table is in the same state in both. The replay measured
    the seat the report credits in game one answering m1 there, and the seat it credits
    in game two — the OTHER label — answering m2 != m1 there. If the labels were
    inverted, the same engine would have to answer both m1 and m2 to the same position
    with the same history, which instrument-mode determinism forbids. So the pair is
    DIRECTLY ATTRIBUTED at t, and no reassignment of it is available to an adversary.

  A non-inert pair with no such t — one game's moves a strict prefix of the other's,
  which only a forfeit or a cap can produce — is neither excluded nor attributed, and is
  reported as a clause (b) FAILURE by name rather than passed over.

Exit 0 the report is a measurement under Criterion 1'', 1 it is not (a confirmed
inversion, an unattributable pair, or a broken link), 2 THE ANSWER COULD NOT BE TAKEN
(unreadable or mutually unrelated documents, an incomplete replay pass, a missing or
unrunnable engine, a budget this cannot replay), 3 A DETERMINISM VIOLATION — bigger than
this WP, reported as such (tools/SHELL_CHECKLIST.md item 12: a void is not a finding, and
neither is an instrument failure).
"""

import hashlib
import math
import re
import subprocess
import sys

# Ported verbatim from crates/pistol-arena/src/sprt.rs and score.rs, for the inert-pair
# cross-check. NOT trusted on its own: `main()` self-checks it against the report's OWN
# printed `llr_pair`/`verdict` on the UNMODIFIED pentanomial before ever using it on a
# hypothetical one, and refuses (exit 2) if the two machineries disagree.
NELO_TO_T = math.log(10.0) / 800.0
PAIR_SCORES = (0.0, 0.25, 0.5, 0.75, 1.0)

# Exit codes, one per kind (tools/SHELL_CHECKLIST.md item 12 obligation 1).
ATTRIBUTABLE = 0
NOT_A_MEASUREMENT = 1
NO_ANSWER = 2
DETERMINISM_VIOLATION = 3


def pair_sample(buckets):
    """`Sample::of_pairs` — the five pentanomial counts to (n, mu, var)."""
    n = sum(buckets)
    if n == 0:
        return 0, 0.0, 0.0
    first = sum(count * score for count, score in zip(buckets, PAIR_SCORES))
    second = sum(count * score * score for count, score in zip(buckets, PAIR_SCORES))
    mu = first / n
    return n, mu, second / n - mu * mu


def recompute_verdict(buckets, elo0, elo1, alpha, beta):
    """`score::verdict`'s pair-unit path, off a pentanomial alone."""
    n, mu, var = pair_sample(buckets)
    if n == 0 or var <= 0.0:
        return "inconclusive_degenerate", None
    t_hat = (mu - 0.5) / math.sqrt(var)
    t0 = elo0 * NELO_TO_T * math.sqrt(2.0)
    t1 = elo1 * NELO_TO_T * math.sqrt(2.0)
    llr = n * ((t1 - t0) * t_hat - (t1 * t1 - t0 * t0) / 2.0)
    h0 = math.log(beta / (1.0 - alpha))
    h1 = math.log((1.0 - beta) / alpha)
    if llr >= h1:
        return "h1", llr
    if llr <= h0:
        return "h0", llr
    return "inconclusive_at_game_cap", llr


def die(why):
    print(f"warm_attribution_check: CANNOT READ: {why}")
    raise SystemExit(NO_ANSWER)


def violation(why):
    print(f"warm_attribution_check: DETERMINISM VIOLATION: {why}")
    print(
        "warm_attribution_check: this is the engine's own instrument-mode guarantee failing "
        "(CLAUDE.md rule 4), not an attribution question. It is bigger than this work package "
        "and it is NOT counted as an attribution failure."
    )
    raise SystemExit(DETERMINISM_VIOLATION)


def fields(line):
    """`game 0 opening 3 p1 r2 …` -> {'game': '0', 'opening': '3', 'p1': 'r2', …}.

    ONE SPELLING PER NUMBER (item 8): a repeated key is refused rather than silently
    last-wins, which would let a human read the first `result` and this read the second.
    """
    parts = line.split()
    out = {}
    for at in range(0, len(parts) - 1, 2):
        key, value = parts[at], parts[at + 1]
        if key in out:
            die(f"the key `{key}` appears twice on one record: `{line}`")
        out[key] = value
    return out


def only(text, pattern, what):
    """The one match of an anchored pattern, or a refusal naming what was wanted."""
    found = re.findall(pattern, text, re.M)
    if not found:
        die(f"no {what}")
    if len(found) > 1:
        die(f"more than one {what}, so there is no one answer to read")
    return found[0]


def slurp(path, what):
    try:
        with open(path, "rb") as handle:
            return handle.read()
    except OSError as why:
        die(f"the {what} `{path}` could not be read: {why}")


def cold_answer(engine, config, budget, prefix, why):
    """One fresh, one-shot process asked one position at the run's own budget.

    Deliberately COLD and deliberately OUTSIDE the warm chain: this is a diagnostic
    probe of an engine the warm pass did not drive at this position, not a link in the
    chain being verified. It is the only wire protocol this file speaks, and it is the
    same shape `tools/wp15b_attribution_check.py`'s link 1a already sends.
    """
    line = "position start moves" + ("" if not prefix else " " + " ".join(prefix))
    try:
        said = subprocess.run(
            [engine, "--config", config],
            input=f"{line}\n{budget}\nquit\n",
            capture_output=True,
            text=True,
        )
    except OSError as reason:
        die(f"`{engine}` could not be run: {reason}")
    best = [x for x in said.stdout.splitlines() if x.startswith("bestmove ")]
    if len(best) != 1:
        die(
            f"{why}: `{engine} --config {config}` answered no single bestmove; it said: "
            f"{said.stdout.strip()} {said.stderr.strip()}"
        )
    return best[0].split()[1]


def read_report(path):
    """The source report, as everything downstream needs off it."""
    raw = slurp(path, "report")
    digest = hashlib.sha256(raw).hexdigest()
    try:
        whole = raw.decode("utf-8")
    except UnicodeDecodeError as why:
        die(f"{path} is not UTF-8: {why}")
    # The verdict block only. Everything past the marker `report.rs` declares
    # "excluded from every comparison", and a criterion resting on it rests on the
    # machine it ran on.
    text = whole.split("\n# timing")[0]
    if not text.startswith("arena_report "):
        die(f"{path} is not a report carrying a verdict (its first token is not arena_report)")

    engines = {}
    for slot in ("a", "b"):
        found = re.search(
            rf"^engine {slot} label (\S+) binary (\S+) binary_sha256 (\S+) config (\S+) "
            rf"config_sha256 (\S+) weights_sha256 (\S+)$",
            text,
            re.M,
        )
        if not found:
            die(f"no readable `engine {slot}` line")
        engines[slot] = dict(
            zip(
                ("label", "binary", "binary_sha256", "config", "config_sha256", "weights_sha256"),
                found.groups(),
            )
        )
    if engines["a"]["label"] == engines["b"]["label"]:
        die(f"both seats carry the label `{engines['a']['label']}`, so nothing can be attributed")

    budget_kind, budget_value = only(
        text, r"^budget (\S+) (\d+)$", "`budget` line, so the probe has no budget to be taken at"
    )
    if budget_kind != "nodes":
        die(
            f"this checks only a `nodes` budget and the run used `{budget_kind}`; a budget that is "
            "not reproducible cannot carry a replay at all"
        )

    games, moves = [], {}
    # `split("\n")` and NOT `splitlines()`: the latter also breaks on \r, \x0b, \x0c,
    # U+2028 and U+0085, while every other read here is `re.M`, which breaks on \n alone.
    # Two notions of "line" over one document is how an engine's VERBATIM refusal — free
    # text this format copies through unquoted — injects a record (item 9).
    for line in text.split("\n"):
        if line.startswith("game "):
            games.append(fields(line))
        elif line.startswith("moves "):
            parts = line.split()
            moves[parts[1]] = parts[2:]
    if not games:
        die("the report records no games")
    if len(games) % 2 != 0:
        die(f"{len(games)} games is not an even number, so pairing is undefined")

    sprt = re.search(r"^sprt elo0 (\S+) elo1 (\S+) alpha (\S+) beta (\S+)$", text, re.M)
    if not sprt:
        die("no `sprt` line, so no bounds exist to recompute a verdict against")

    return {
        "path": path,
        "sha256": digest,
        "text": text,
        "engines": engines,
        "experiment_sha256": only(text, r"^experiment_sha256 (\S+)$", "`experiment_sha256` line"),
        "budget": f"go nodes {budget_value}",
        "budget_value": budget_value,
        "opening_turns": int(only(text, r"^opening_turns (\d+)$", "`opening_turns` line")),
        "turn_cap": only(text, r"^turn_cap (\d+)$", "`turn_cap` line"),
        "verdict": only(text, r"^verdict (\S+)$", "`verdict` line"),
        "forfeits": int(only(text, r"^counts .* forfeits (\d+) ", "`forfeits` field on `counts`")),
        "sprt": tuple(float(x) for x in sprt.groups()),
        "games": games,
        "moves": moves,
    }


def read_replay(path):
    """The warm-replay document, as everything downstream needs off it."""
    raw = slurp(path, "replay document")
    try:
        whole = raw.decode("utf-8")
    except UnicodeDecodeError as why:
        die(f"{path} is not UTF-8: {why}")
    text = whole.split("\n# timing")[0]
    if text.startswith("warm_replay_aborted "):
        die(
            f"{path} is an ABANDONED pass: it does not cover every game of its report, and a "
            "criterion over some of a report's games is a criterion over a sample nobody "
            "registered"
        )
    if not text.startswith("warm_replay 1\n"):
        die(f"{path} is not a schema-1 warm-replay document (its first line is not `warm_replay 1`)")

    engines = {}
    for slot in ("a", "b"):
        found = re.search(
            rf"^engine {slot} label (\S+) binary_sha256 (\S+) config_sha256 (\S+) "
            rf"weights_sha256 (\S+)$",
            text,
            re.M,
        )
        if not found:
            die(f"the replay document has no readable `engine {slot}` line")
        engines[slot] = dict(
            zip(("label", "binary_sha256", "config_sha256", "weights_sha256"), found.groups())
        )

    replays, divergences = {}, []
    for line in text.split("\n"):
        if line.startswith("replay "):
            record = fields(line)
            index = record["replay"]
            if index in replays:
                die(f"the replay document records game {index} twice")
            replays[index] = record
        elif line.startswith("divergence "):
            divergences.append(fields(line))
    if not replays:
        die("the replay document records no replayed games")

    return {
        "path": path,
        "source_sha256": only(text, r"^source_report_sha256 (\S+)$", "`source_report_sha256` line"),
        "source_experiment_sha256": only(
            text, r"^source_experiment_sha256 (\S+)$", "`source_experiment_sha256` line"
        ),
        "budget_value": only(text, r"^budget nodes (\d+)$", "`budget nodes` line"),
        "opening_turns": int(only(text, r"^opening_turns (\d+)$", "`opening_turns` line")),
        "turn_cap": only(text, r"^turn_cap (\d+)$", "`turn_cap` line"),
        "games": int(only(text, r"^games (\d+)$", "`games` line")),
        "divergence_count": int(only(text, r"^divergences (\d+)$", "`divergences` line")),
        "engines": engines,
        "replays": replays,
        "divergences": divergences,
    }


def bind(report, replay):
    """The two documents must be about each other, and the referent that says so is one
    neither of them computes about itself: the report's own file digest."""
    if replay["source_sha256"] != report["sha256"]:
        die(
            f"the replay document was taken from a report whose sha256 is "
            f"{replay['source_sha256']}, and `{report['path']}` hashes to {report['sha256']} — the "
            "two documents are not about each other"
        )
    for what, mine, theirs in (
        ("experiment_sha256", report["experiment_sha256"], replay["source_experiment_sha256"]),
        ("budget", report["budget_value"], replay["budget_value"]),
        ("opening_turns", str(report["opening_turns"]), str(replay["opening_turns"])),
        ("turn_cap", report["turn_cap"], replay["turn_cap"]),
    ):
        if mine != theirs:
            die(f"the two documents disagree on {what}: `{mine}` against `{theirs}`")
    for slot in ("a", "b"):
        for key in ("label", "binary_sha256", "config_sha256", "weights_sha256"):
            if report["engines"][slot][key] != replay["engines"][slot][key]:
                die(
                    f"the two documents disagree on seat {slot}'s {key}: "
                    f"`{report['engines'][slot][key]}` against `{replay['engines'][slot][key]}`"
                )
    if replay["games"] != len(report["games"]):
        die(
            f"the replay covers {replay['games']} game(s) and the report records "
            f"{len(report['games'])}"
        )
    if len(replay["replays"]) != len(report["games"]):
        die(
            f"the replay document holds {len(replay['replays'])} `replay` record(s) for "
            f"{len(report['games'])} game(s)"
        )


def check_coverage(report, replay, notes):
    """Every game replayed, every move list the same length, every clean game's nodes
    equal. The node equality is the referent this file does not compute about itself."""
    for game in report["games"]:
        index = game["game"]
        record = replay["replays"].get(index)
        if record is None:
            die(f"game {index} has no `replay` record")
        played = report["moves"].get(index)
        if played is None:
            die(f"game {index} has no `moves` line")
        if len(played) != int(game["turns"]):
            die(f"game {index}: `turns {game['turns']}` against {len(played)} recorded turns")
        if int(record["recorded_turns"]) != len(played):
            die(
                f"game {index}: the replay saw {record['recorded_turns']} recorded turn(s) and the "
                f"report holds {len(played)}"
            )
        if record["status"] != "clean":
            continue
        if int(record["replayed_turns"]) != len(played):
            die(
                f"game {index}: the replay reports no divergence yet fed only "
                f"{record['replayed_turns']} of {len(played)} recorded turn(s)"
            )
        # A clean game's replay asked for the same searches, in the same order, at the
        # same budget — so it spent the same nodes. A forfeited game is the one exception
        # and it is a bounded one: the forfeiting engine's last, refused ask has no
        # recorded move to replay against, so only the OTHER seat's count is comparable
        # and the forfeiter's may only be lower.
        forfeiter = None
        if game["end"] == "forfeit":
            forfeiter = "a" if game["forfeit_by"] == report["engines"]["a"]["label"] else "b"
        for slot in ("a", "b"):
            ran, again = int(game[f"nodes_{slot}"]), int(record[f"nodes_{slot}"])
            if slot == forfeiter:
                if again > ran:
                    violation(
                        f"game {index}: seat {slot} forfeited after {ran} node(s) in the run and "
                        f"spent {again} replaying the turns it did complete, which is more than "
                        "the whole game cost it"
                    )
                continue
            if ran != again:
                violation(
                    f"game {index}: seat {slot} spent {ran} node(s) in the run and {again} "
                    "replaying the identical sequence of positions at the identical budget"
                )
    notes.append(
        f"W coverage: {len(report['games'])} game(s) replayed in full, every clean game's node "
        "counts equal to the run's"
    )


def classify(report, replay, engine, notes):
    """Every divergence resolved, by ONE cold probe of the other engine."""
    label_a = report["engines"]["a"]["label"]
    inversions = []
    for found in replay["divergences"]:
        index = found["divergence"]
        at_turn = int(found["at_turn"])
        played = report["moves"][index]
        if at_turn <= report["opening_turns"]:
            die(
                f"game {index}: a divergence is reported at turn {at_turn}, which is book — no "
                "engine was ever asked about it"
            )
        if at_turn > len(played):
            die(f"game {index}: a divergence at turn {at_turn} of a {len(played)}-turn game")
        credited = found["mover"]
        recorded = played[at_turn - 1]
        if found["recorded"] != recorded:
            die(
                f"game {index}: the replay says turn {at_turn} recorded `{found['recorded']}` and "
                f"the report's own move list says `{recorded}`"
            )
        other = "b" if credited == label_a else "a"
        answer = cold_answer(
            engine,
            report["engines"][other]["config"],
            report["budget"],
            played[: at_turn - 1],
            f"game {index} turn {at_turn}",
        )
        if answer == recorded:
            inversions.append(
                f"game {index} turn {at_turn}: the report credits `{recorded}` to `{credited}`, "
                f"which answered `{found['answered']}` warm, and the other seat "
                f"(`{report['engines'][other]['label']}`, {report['engines'][other]['config']}) "
                f"answers `{answer}` — the seats are the wrong way round"
            )
        else:
            violation(
                f"game {index} turn {at_turn}: the report records `{recorded}`; the credited seat "
                f"`{credited}`, replayed WARM at exactly the state the run had, answers "
                f"`{found['answered']}`, and the other seat, asked cold at the same prefix and the "
                f"same budget, answers `{answer}`. Nothing known explains what was played."
            )
    notes.append(
        f"W classification: {len(replay['divergences'])} divergence(s), {len(inversions)} confirmed "
        "inversion(s), 0 unexplained"
    )
    return inversions


def clause_b(report, replay, buckets, notes, failures):
    """Every pair either excluded by the inert theorem or directly attributed."""
    games, moves = report["games"], report["moves"]
    inert, attributed, unattributable = [], [], []
    for pair in range(len(games) // 2):
        first, second = games[2 * pair], games[2 * pair + 1]
        one, two = moves[first["game"]], moves[second["game"]]
        forfeited = "forfeit" in (first["end"], second["end"])
        if one == two and not forfeited:
            inert.append(pair)
            if buckets[pair] != 2:
                failures.append(
                    f"(b) pair {pair} (opening {first['opening']}) has two identical move lists, "
                    f"neither forfeited and both replayed clean — the inert-pair theorem forces a "
                    f"1-1 split — and the report records bucket p{buckets[pair]}"
                )
            continue
        witness = next(
            (
                at
                for at in range(min(len(one), len(two)))
                if one[at] != two[at] and at >= report["opening_turns"]
            ),
            None,
        )
        if witness is None:
            unattributable.append(
                f"(b) pair {pair} (opening {first['opening']}) is not inert "
                f"({'a forfeit ended one of its games' if forfeited else 'its move lists differ'}) "
                "and its two games never differ at a turn either engine searched, so no replayed "
                "turn tells the two seats apart in it"
            )
        else:
            attributed.append(pair)
    failures.extend(unattributable)
    notes.append(
        f"(b): {len(inert)} inert pair(s) excluded by theorem, {len(attributed)} pair(s) directly "
        f"attributed at their first differing searched turn, {len(unattributable)} unattributable"
    )
    return inert


def cross_check(report, buckets, inert, notes, failures):
    """The old adversarial reassignment, run over the INERT pairs alone.

    Expected a no-op, and expected by ARITHMETIC rather than by hope: the theorem forces
    an inert pair to p2 and `4 - 2` is `2`. It is run and cited anyway, as confirming
    evidence, because a cross-check that is only argued is a cross-check nobody ran.
    """
    elo0, elo1, alpha, beta = report["sprt"]
    if not inert:
        notes.append("cross-check: no inert pairs — the exclusion changed nothing")
        return
    if report["forfeits"] > 0:
        notes.append(
            f"cross-check: {report['forfeits']} forfeit(s) already make this run `invalid_forfeit` "
            "by the arena's own scoring rule, and this pentanomial-only recomputation has no "
            "notion of a forfeit — skipped, not silently passed"
        )
        return
    counts = [sum(1 for bucket in buckets if bucket == slot) for slot in range(5)]
    honest, _ = recompute_verdict(counts, elo0, elo1, alpha, beta)
    if honest != report["verdict"]:
        die(
            f"this tool's ported sprt.rs/score.rs arithmetic recomputes `{honest}` off the "
            f"report's own unmodified pentanomial, against its printed `verdict "
            f"{report['verdict']}` — the two machineries disagree on the honest input, so nothing "
            "computed from a hypothetical one can be trusted"
        )
    flipped = list(counts)
    for pair in inert:
        flipped[buckets[pair]] -= 1
        flipped[4 - buckets[pair]] += 1
    token, _ = recompute_verdict(flipped, elo0, elo1, alpha, beta)
    if token == report["verdict"]:
        notes.append(
            f"cross-check: reassigning all {len(inert)} inert pair(s) leaves the verdict "
            f"`{report['verdict']}` unchanged, as the theorem says it must"
        )
    else:
        failures.append(
            f"cross-check: reassigning the {len(inert)} pair(s) the inert theorem excludes moves "
            f"the verdict from `{report['verdict']}` to `{token}` — the theorem says this is "
            "impossible, so either the exclusion or the arithmetic is wrong"
        )


def link_1b(report, notes, failures):
    """MOVES -> RESULT, by game rule 3. Carried from WP-1.5b unchanged."""
    judged = 0
    for game in report["games"]:
        if game["result"] == "capped" or game["end"] != "normal":
            continue
        judged += 1
        by_p1 = len(report["moves"][game["game"]]) % 2 == 1
        if by_p1 != (game["result"] == "p1_win"):
            failures.append(
                f"1b game {game['game']}: {game['turns']} turns were played, so the last turn was "
                f"{'p1' if by_p1 else 'p2'}'s, and the report records `result {game['result']}`"
            )
    notes.append(f"1b: {judged} decided non-forfeit game(s) adjudicated against the move list")
    if judged == 0:
        failures.append("1b is VACUOUS on this input: no game was decided by the rules")


def link_1c(report, scores, buckets, notes, failures):
    """RESULT -> SCORE -> VERDICT, off the score_a path. Carried from WP-1.5b unchanged."""
    text, games = report["text"], report["games"]
    derived = {
        "n": len(games),
        "wins_a": sum(1 for score in scores if score == 1.0),
        "losses_a": sum(1 for score in scores if score == 0.0),
        "capped": sum(1 for game in games if game["result"] == "capped"),
    }
    counts = fields(only(text, r"^counts (.*)$", "`counts` line"))
    for key, value in derived.items():
        if key not in counts:
            die(f"the `counts` line carries no `{key}`")
        if int(counts[key]) != value:
            failures.append(
                f"1c `counts {key} {counts[key]}` against {value} rebuilt from the `game` lines"
            )
    printed = [fields(line) for line in text.split("\n") if line.startswith("pair ")]
    if len(printed) != len(buckets):
        failures.append(f"1c {len(printed)} `pair` lines against {len(buckets)} pairs of games")
    for pair, bucket in zip(printed, buckets):
        if pair["bucket"] != f"p{bucket}" or abs(float(pair["score_a"]) - bucket / 4) > 1e-9:
            failures.append(
                f"1c `pair {pair['pair']} bucket {pair['bucket']} score_a {pair['score_a']}` "
                f"against p{bucket} / {bucket / 4:.9f} rebuilt from the `game` lines"
            )
    pentanomial = fields(only(text, r"^pentanomial (.*)$", "`pentanomial` line"))
    for slot in range(5):
        mine = sum(1 for bucket in buckets if bucket == slot)
        if f"p{slot}" not in pentanomial:
            die(f"the `pentanomial` line carries no `p{slot}`")
        if int(pentanomial[f"p{slot}"]) != mine:
            failures.append(
                f"1c `pentanomial p{slot} {pentanomial[f'p{slot}']}` against {mine} rebuilt from "
                "the `game` lines"
            )
    notes.append(f"1c: {len(games)} game(s) and {len(buckets)} pair(s) rebuilt off the score_a path")


def main():
    if len(sys.argv) != 4:
        die("usage: wp16_warm_attribution_check.py <report> <replay> <engine-binary>")
    report = read_report(sys.argv[1])
    replay = read_replay(sys.argv[2])
    engine = sys.argv[3]
    bind(report, replay)

    label_a = report["engines"]["a"]["label"]

    def score_a(game):
        if game["result"] == "capped":
            return 0.5
        winner = game["p1"] if game["result"] == "p1_win" else game["p2"]
        return 1.0 if winner == label_a else 0.0

    scores = [score_a(game) for game in report["games"]]
    buckets = [
        round((scores[at] + scores[at + 1]) * 2) for at in range(0, len(report["games"]), 2)
    ]

    notes, failures = [], []
    check_coverage(report, replay, notes)
    # DETERMINISM FIRST. `classify` and `check_coverage` both exit on their own code
    # before anything below can turn an instrument failure into an attribution count.
    inversions = classify(report, replay, engine, notes)
    if inversions:
        failures.extend(f"(a) {line}" for line in inversions)
    else:
        inert = clause_b(report, replay, buckets, notes, failures)
        cross_check(report, buckets, inert, notes, failures)
    link_1b(report, notes, failures)
    link_1c(report, scores, buckets, notes, failures)

    if replay["divergence_count"] != len(replay["divergences"]):
        die(
            f"the replay document says `divergences {replay['divergence_count']}` and carries "
            f"{len(replay['divergences'])} `divergence` record(s)"
        )

    for note in notes:
        print(f"warm_attribution_check: {note}")
    for failure in failures:
        print(f"warm_attribution_check: FAIL {failure}")
    print(
        f"warm_attribution_check: {'PASS' if not failures else 'FAIL'} — {len(failures)} failure(s)"
    )
    return ATTRIBUTABLE if not failures else NOT_A_MEASUREMENT


if __name__ == "__main__":
    # A FIELD THAT IS NOT THERE, OR IS NOT A NUMBER, IS A VOID AND NOT A FINDING (item
    # 12). Every read above names its record, so the refusal names the key; what this
    # catches is the reads too numerous to guard one at a time, and it catches them into
    # exit 2 rather than letting a traceback exit 1 and read as "the run's seats are
    # mis-attributed".
    try:
        raise SystemExit(main())
    except (KeyError, ValueError, IndexError) as why:
        die(f"a record in one of the documents is malformed: {why!r}")
