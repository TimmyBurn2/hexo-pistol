#!/usr/bin/env python3
#
# The WP-2.1 sweep's per-tranche arena config, written rather than hand-copied.
#
# WHY THIS EXISTS. The sweep is sixteen tranches of one shape, and sixteen
# near-identical committed documents is the two-documents-one-claim defect at
# scale (docs/decisions.md D-423). Worse, every one of them carries a
# `binary_sha256` that CANNOT BE TRUE until the binary it names exists — which
# is why the pilot deferred that one field to a slot pass
# (docs/experiments/wp20_pilot_prereg.md). This script is that slot pass,
# generalised: every value it writes is fixed in
# docs/experiments/wp21_prereg.md section 1, and the only things that vary
# between two of its outputs are the tranche's two integers.
#
# WHAT IT IS NOT. It is not a template engine and it takes no defaults. A
# missing argument is a named refusal, not a substitution (CLAUDE.md rule 1).
# It never overwrites: an existing --out is refused, because a config silently
# rewritten under a live run is docs/decisions.md D-199's own defect.
#
# THE PARTITION IS ARITHMETIC AND IS HERE RATHER THAN IN A TABLE, so that a
# reader can check a tranche's range by evaluating it rather than by trusting a
# transcription.
#
# Usage:
#   tools/wp21_tranche_config.py --tranche <1..16> --out <path>
#                                --binary-sha256 <64 hex>
#   tools/wp21_tranche_config.py --skip <n> --take <n> --out <path>
#                                --binary-sha256 <64 hex>
#
# THE SECOND FORM, AND WHY IT IS THE SAME SCRIPT. `wp21_prereg.md` revision 4's
# T-F re-captures a REGISTERED SUB-RANGE of tranche one — `skip 13, take 20` —
# and that config carries every value a tranche's does except the two integers.
# Hand-writing it is the defect this generator exists to prevent, so the window
# is an argument rather than a second document. It is refused if it reaches the
# reserved holdout, by the same rule the tranche form obeys.
#                                --binary-sha256 <64 hex>
#
# Exit:  0 the config was written; its own sha256 is printed
#        1 a named refusal — a bad argument, or an --out that already exists
#        2 THE RUN IS VOID: no config was written and no answer was taken
#          (tools/SHELL_CHECKLIST.md item 12). A void is not a refusal.

import argparse
import hashlib
import re
import sys
from pathlib import Path

VOID = 2

# The book, and the slice this registration consumes. Both are read from
# docs/experiments/wp21_prereg.md section 2 and neither is derived at run time.
#
# THE LAST 1,000 OPENINGS OF THE BOOK ARE A HOLDOUT AND THIS SWEEP MAY NOT
# REACH THEM (docs/decisions.md D-568): openings 3500..4499 are RESERVED FOR
# GOVERNED RUNS and are never labelled, so that the Stage-3 detector's SPRT and
# the WP-1.5d resolution run keep a slice each. The rule fixing the holdout —
# the LAST 1,000 — is stated before the sweep starts, because a holdout chosen
# after seeing which openings label well is not a holdout.
BOOK = "crates/pistol-cli/tests/fixtures/random_openings_v2.txt"
BOOK_OPENINGS = 4500
HOLDOUT = 1000
FIRST_OPENING = 13
OPENINGS = BOOK_OPENINGS - HOLDOUT - FIRST_OPENING
TRANCHES = 16

# Section 1's seat, in full. Nothing here has a default: a value absent from
# this table is a value this script cannot write.
ENGINE_CONFIG = "configs/instrument_v0.toml"
BINARY = "target/release/pistol"
TURN_CAP = 40
N_WORKERS = 1
HANG_TIMEOUT_MS = 120000
GAME_BUDGET_KIND = "nodes"
GAME_BUDGET_VALUE = 50000
SCHEMA_VERSION = 2
SPRT = {"elo0": "0.0", "elo1": "15.0", "alpha": "0.05", "beta": "0.05"}

SHA256 = re.compile(r"\A[0-9a-f]{64}\Z")


def refuse(why):
    print(f"wp21_tranche_config: REFUSED: {why}", file=sys.stderr)
    raise SystemExit(1)


def spelled(word, what, low):
    """A count whose SPELLING is validated, not only its value.

    `+4`, ` 4` and `04` all parse to four and would land in a receipt
    unnormalised, describing a run nobody reproduces by copying the line back
    (tools/SHELL_CHECKLIST.md item 8).
    """
    try:
        value = int(word)
    except ValueError:
        refuse(f"`{word}` is not a {what}")
    if str(value) != word:
        refuse(
            f"`{word}` is a {what} spelled a way this program will not echo back; "
            f"write it as `{value}`"
        )
    if value < low:
        refuse(f"a {what} of {value} is below {low}")
    return value


def slice_of(tranche):
    """The tranche's `(openings_skip, openings_take)`.

    The remainder is spread over the FIRST tranches rather than dropped into
    the last one, so no tranche is materially larger than any other and the
    wave arithmetic in section 3 holds for every one of them.
    """
    base, extra = divmod(OPENINGS, TRANCHES)
    index = tranche - 1
    if index < extra:
        return FIRST_OPENING + index * (base + 1), base + 1
    return FIRST_OPENING + extra * (base + 1) + (index - extra) * base, base


def document(tranche, skip, take, binary_sha256):
    holdout_first = FIRST_OPENING + OPENINGS
    which = (
        f"TRANCHE {tranche} of {TRANCHES}"
        if tranche is not None
        else f"WINDOW openings_skip {skip} openings_take {take}"
    )
    return f"""# WP-2.1 production label sweep — {which}.
#
# GENERATED by tools/wp21_tranche_config.py from the values
# docs/experiments/wp21_prereg.md section 1 fixes. Do not hand-edit: an edit
# here is an edit to a document nothing reviewed, and the generator is what the
# pre-registration binds.
#
# THIS TRANCHE CARRIES NO CENSUS and starts no clock against docs/decisions.md
# D-537's minimum. The gate is off in the engine config both seats name, so the
# token would turn on a recorder with nothing to record (D-563).
#
# THE HOLDOUT IS OUT OF REACH OF THIS FILE: openings {holdout_first}..{BOOK_OPENINGS - 1} are reserved
# for governed runs and are labelled by no tranche (docs/decisions.md D-568).
#
# NO STRENGTH CLAIM. Both seats are one engine, which is the only shape
# `arena --capture` accepts; every pair scores alike and the verdict is
# degenerate by construction (D-156).
#
# Validate this file with tools/config_check.sh.

schema_version = {SCHEMA_VERSION}

[run]
openings_file = "{BOOK}"

# THE TRANCHE'S SLICE. The whole registration consumes {FIRST_OPENING}..{FIRST_OPENING + OPENINGS - 1};
# this tranche is openings {skip}..{skip + take - 1} of the book.
openings_take = {take}
openings_skip = {skip}

# An evaluation horizon, never a game rule (game rule 6).
turn_cap = {TURN_CAP}

# ONE worker: eight tranches run at once, and a tranche that also fanned out
# would oversubscribe the box eight-fold. Pass 2 is serial whatever this says.
n_workers = {N_WORKERS}

# The liveness watchdog, checked against the LABEL budget rather than the game
# budget: pass 2 reads its watchdog out of the report this file writes, and a
# label ask is the longest single search this tranche makes (D-159).
hang_timeout_ms = {HANG_TIMEOUT_MS}

[budget]
# The GAME budget. The LABEL budget is not here — it is a command-line argument
# to `arena --capture`, and the pre-registration fixes it at `nodes 400000`.
kind = "{GAME_BUDGET_KIND}"
value = {GAME_BUDGET_VALUE}

[sprt]
# Present because the schema requires them, and read by nothing: a self-match
# crosses no bound.
elo0 = {SPRT["elo0"]}
elo1 = {SPRT["elo1"]}
alpha = {SPRT["alpha"]}
beta = {SPRT["beta"]}

# ONE ENGINE IN TWO SEATS. The two LABELS must differ and the two IDENTITIES
# must match, which is why a self-play report is the only capturable shape.
[engine_a]
label = "a"
binary = "{BINARY}"
binary_sha256 = "{binary_sha256}"
config = "{ENGINE_CONFIG}"

[engine_b]
label = "b"
binary = "{BINARY}"
binary_sha256 = "{binary_sha256}"
config = "{ENGINE_CONFIG}"
"""


def main():
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--tranche")
    parser.add_argument("--skip")
    parser.add_argument("--take")
    parser.add_argument("--out", required=True)
    parser.add_argument("--binary-sha256", required=True)
    args = parser.parse_args()

    window = args.skip is not None or args.take is not None
    if window and args.tranche is not None:
        refuse("--tranche and --skip/--take name the range two ways; give one form")
    if not window and args.tranche is None:
        refuse("give --tranche, or --skip and --take")

    if window:
        if args.skip is None or args.take is None:
            refuse("--skip and --take are one form and both are required")
        skip = spelled(args.skip, "skip", low=0)
        take = spelled(args.take, "take", low=1)
        if skip < FIRST_OPENING:
            refuse(
                f"skip {skip} reaches openings 0..{FIRST_OPENING - 1}, which the WP-2.0 pilot "
                f"consumed (docs/book_v2_ledger.md)"
            )
        if skip + take > BOOK_OPENINGS - HOLDOUT:
            refuse(
                f"skip {skip} + take {take} reaches opening {skip + take - 1}, and "
                f"{BOOK_OPENINGS - HOLDOUT}..  is the holdout reserved for governed runs and labelled "
                f"by no tranche (docs/decisions.md D-568)"
            )
        tranche = None
    else:
        tranche = spelled(args.tranche, "tranche number", low=1)
        if not 1 <= tranche <= TRANCHES:
            refuse(f"tranche {tranche} is outside 1..{TRANCHES}, which is the whole partition")
        skip, take = slice_of(tranche)
    if not SHA256.match(args.binary_sha256):
        refuse("--binary-sha256 is not sixty-four lower-case hex digits")

    out = Path(args.out)
    if out.exists():
        refuse(
            f"{out} already exists — a config rewritten under a live run is a document that "
            f"drifted from the run reading it (docs/decisions.md D-199)"
        )

    text = document(tranche, skip, take, args.binary_sha256)
    try:
        # `x` is the claim and the existence check in one syscall, so the check
        # above is the readable refusal and this one is the race-free guard.
        with open(out, "x", encoding="utf-8") as handle:
            handle.write(text)
    except FileExistsError:
        refuse(f"{out} appeared between the check and the write")
    except OSError as why:
        print(f"wp21_tranche_config: VOID: cannot write {out}: {why}", file=sys.stderr)
        raise SystemExit(VOID)

    digest = hashlib.sha256(text.encode("utf-8")).hexdigest()
    which = f"tranche {tranche} of {TRANCHES}" if tranche is not None else "window"
    print(
        f"wp21_tranche_config: {which}: openings_skip {skip} "
        f"openings_take {take} -> {out} sha256 {digest}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
