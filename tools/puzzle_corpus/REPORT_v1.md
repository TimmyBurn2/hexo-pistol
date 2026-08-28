# WP-P1 — hexo sandbox puzzle corpus, extraction only

Revision: working tree at `a14912a` + untracked `tools/puzzle_corpus/`, `corpus/`.
Corpus file `corpus/puzzles/hexo_discord_v1.jsonl`, sha256
`56a632e3c830403b414d47a13af0da9cd658b6039a8d40d52c4267d2a4816dd1`.

**The mapping gate FAILS. The corpus is not fixture-grade. See step 5.**

## Step 0 — reuse survey

The dispatch places the bridge at `src/hexo_bridge/...` in this repo. It is not
here: hexo-bridge is a **separate repository** at `~/Projects/hexo-bridge`, HEAD
`4a96d13`. The files named exist there.

| asked | answer |
| --- | --- |
| axial coordinate mapping | `core/move.py:32-63` — `Coord(q, r)`, documented "+q right, +r top-right". Declarative prose, no distance function, no axis vectors (`grep -rn distance src/` is empty). |
| player-to-colour mapping | **does not exist.** `bridge.py:415` maps play order to the engine alphabet (`p1 -> Side.X`). No colour term appears anywhere in `src/` or `docs/`. |

Two further facts that bear on scope:

- The bridge **does not ingest this platform's share links**. It ingests the
  live Bot API and the htttx websocket (`docs/data-flow.md:8-33`). `sandbox`
  and `share` appear nowhere in `src/`. The premise "it already ingests this
  platform" holds for gameplay, not for sandbox positions.
- So the coordinate mapping is *asserted* in the bridge and *never measured*
  against pistol. Step 5 became the only source of truth, as the dispatch
  anticipated.

Reuse actually taken: no geometry was invented. `Coord::distance`,
`LEGAL_RADIUS`, the three axis vectors and the turn grouping are transcribed
from `crates/pistol-core/src/{coord,rules,axis}.rs` and
`crates/pistol-cli/src/corpus/replay.rs` (`group_turns`), cited at each site.

## Step 1 — discovery

    $ curl -sS -D headers.txt -o page.html "https://hexo.did.science/sandbox/7d7ver4"
    HTTP/2 200
    content-type: text/html; charset=utf-8
    server: cloudflare
    $ wc -c page.html
    15565 page.html

    $ grep -c "__NEXT_DATA__" page.html      -> 0
    $ grep -c "self.__next_f" page.html      -> 0
    $ grep -c "application/json" page.html   -> 0
    $ grep -oE '\[-?[0-9]+,-?[0-9]+\]' page.html   -> (no output)
    $ grep -oE '(window|globalThis|self)\.[A-Za-z_$]+' page.html | sort -u
    window.__IH

The `<meta>` block carries counts only, and it corroborates the architect's
pre-fetch exactly:

    <meta name="description" content="Open the &quot;chaoticish vs .thatscrispy -
    Replay Move 53/67&quot; sandbox position with 45 placed cells. Player 2 to
    move with 2 placements remaining."/>

The cells are inline in one non-`src` script, as
`window.__IH3T_DEHYDRATED_STATE__` — a react-query dehydration payload. The
position sits under `queryKey: ["sandbox-position", "<id>"]`:

    gamePosition keys: ['cells', 'currentTurnPlayer', 'placementsRemaining']
    ncells: 45
    first: {'x': 0, 'y': 0, 'player': 'player-1', 'moveId': 1}
    last:  {'x': 3, 'y': -1, 'player': 'player-1', 'moveId': 45}
    {"currentTurnPlayer": "player-2", "placementsRemaining": 2}

**Cells are inline, so no API is needed and discovery stopped.** No JS chunk was
searched for `/api/` paths; none was required.

Two shape facts the schema depends on:

- The coordinate keys are **`x`/`y`, not `q`/`r`**. That naming is why step 5
  cannot be skipped.
- Every cell carries a **`moveId`**, so the listed order is *witnessed* rather
  than merely assumed. D-6 identifies the phase-1 stone as the last listed stone
  of the mover; the extractor asserts `moveId` order agrees with list order and
  would emit an `ORDER:` finding otherwise. It fired on none of the 48.

## Findings

**F-1 (scope).** hexo-bridge is a separate repo and does not ingest share links;
no colour mapping exists there. Detail in step 0.

**F-2 (data, material).** The payload's `player-1`/`player-2` are **identity
labels, not play order.** 17 of the 48 positions open with `player-2` placing
the single first stone. All 48 match perfect turn alternation once the opener is
read from the position rather than from the label.

This was found the hard way: the first extractor run derived the mover from the
label and called those 17 invalid on V2 and V5 simultaneously — a systematic
signature, not 17 hand-edits. `mover_of_turn` now takes the opener as an
argument. The label is a *side identity*; it does not tell you who moved first,
and the bridge's `Side` docstring ("`p1` moves first in the turn sequence") is
true of the Bot API surface but **not** of this payload. Anything that joins the
two surfaces must not assume they agree.

**F-3 (claims).** 8 of 48 titles assert an outcome and are copied verbatim into
`claim` with `claim_status: "UNVERIFIED"`. **4 of those 8 name a colour**
(`forced win for yellow`, `yellow forced win`, `blue forced win`, `win for
blue`). No player-to-colour mapping exists in the bridge (F-1), and none is
recoverable from the extracted channels — the served JS entry point is a 9.4 kB
loader and the colour lives in a lazily-loaded chunk, which is out of scope
here. **Those four claims cannot currently be attributed to a player at all**,
independently of whether they are true. WP-P2 needs the colour mapping before it
can score a colour-named claim against a verdict.

**F-4 (measurement, D-218 flip condition — checked and NOT triggered).** A first
pass measured 12 placements at hex distance 9 from the board, which reads like
D-101's flip condition. It is an artifact of measuring against the *pre-turn*
board: for the last stone of a turn the D-218 witness must include its own
turn-mates. Measured correctly, the order-independent distribution over all 48
positions is

    1: 884   2: 217   3: 71   4: 37   5: 34   6: 14   7: 16   8: 35     MAX = 8

with **zero** placements beyond `LEGAL_RADIUS`. D-101 and D-218 stand; nothing
in pistol-core moves. Recorded because the wrong measurement is easy to make and
would have escalated a rules-truth amendment that is not there.

**V-failures: none.** All 48 records are `valid: true` with empty `findings`.
Because that is exactly what a suite of deleted checks would also produce, the
test suite carries negative controls: each of V1-V5, the duplicate-cell check
and both `ORDER` checks is fired by a position built to break it (11 controls,
all passing). The checks work; these positions are clean.

## Steps 3/4 — schema and validation

Schema as dispatched, unchanged, one object per line sorted by id. `player` and
`to_move` are emitted as `1`/`2` (the payload's `player-N`, D-56: one spelling).

V1 is implemented as **parity consistent with `placements_left`**, not the
literal "cell_count is odd". The literal form is only correct at a turn
boundary; a position with `placements_left == 1` is mid-turn and necessarily has
an even cell count, and the literal rule would call every such position invalid.
**The deviation is moot for v1**: all 48 positions have `placements_left == 2`,
where the two forms coincide. Flagged because it will not stay moot.

## Step 5 — MAPPING GATE: **UNVERIFIED, escalating**

Candidate mapping: `q = x`, `r = y` (identity), with `player-N` as an identity
label per F-2. `tools/puzzle_corpus/mapping_gate.py` answers two questions.

**BASIS leg — PASS.** Re-reading all 48 positions under a wrong hex basis (third
axis `(1,1)` rather than pistol's `(1,-1)`, i.e. `r -> -r`) and counting pinned-
rule violations:

    basis               rule-5 turns   rule-2 six-runs
    identity  (q, r)               0                 0
    reflected (r, q)               0                 0
    negate-r  (q,-r)              10                 3
    negate-q  (-q,r)              10                 3

The wrong basis produces 10 turns with no legal ordering under rule 5 and 3
six-runs that would be `IllegalPosition` under D-6. Identity and its reflection
are clean. **The payload's `(x, y)` is an axial pair in pistol's lattice.** This
leg does not depend on corpus coverage.

**WITNESS leg — FAIL.** Human corpus pin `b2fe61eb...` verified before reading.
Of 17 replay-kind records, **16 resolve to no corpus game under any of the 12
lattice symmetries** (a set of n cells can only equal a prefix-set of length n,
so the record's own cell count is the only prefix worth testing) — the corpus is
a snapshot and these Discord shares are largely not in it. One resolves:

    n2sqzrf  n=11  AMBIGUOUS under 3 symmetries
        rot0         [('917f33efd73933bb', 29)]  <- move count corroborates
        rot240+refl  [('149c4d508cdbef0b', 95)]
        rot300       [('eed4639fe21a2af7', 239)]

The dispatch required two independent witnesses precisely because one match can
be coincidence under a lattice symmetry. **That risk is now measured, not
hypothesised, and it is real**: this 11-cell position is set-equal to the
opening of three different corpus games under three different symmetries. Only
the title corroborates identity — `"Replay Move 11/29"` against a matched game
of length 29, where the two rivals are 95 and 239 moves. That is a title
agreeing with a number, not a geometric witness.

So: **one witness, and it is ambiguous. Zero unambiguous witnesses; two
required. The gate fails.** No mapping variant was adopted to make anything fit;
the variants were run to *falsify*, which is what produced the BASIS result.

**What is escalated to the architect.** The mapping is verified up to a lattice
symmetry and unverified beyond it. The open question is narrow: identity versus
`rot240+refl` versus `rot300`. Worth noting for the decision — a rotation or
reflection is a symmetry of the game, and pistol has `symmetry.rs`; if the
corpus is to be used only for positions and not for cross-referencing corpus
games, the residual may cost nothing. That is the architect's call, not this
package's. Two routes to a second witness: a corpus snapshot contemporaneous
with these shares, or any share link whose game is known to be in `b2fe61eb...`.

## Step 6 — emit

`corpus/puzzles/hexo_discord_v1.jsonl` is emitted (48 records).

The golden set-position text fixture **is not emitted**, for two reasons, either
sufficient. First, step 5 says not to commit the corpus as fixture-grade and the
gate failed. Second, the existing format (`crates/pistol-core/tests/fixtures/
golden_boards_v1.txt`) requires `expect win|no-win` and a `last` stone per case;
emitting into it means editing pistol-core and its pinned SHA, which the
dispatch's NON-SCOPE forbids. No text format was invented.

## Tests

`tools/puzzle_corpus/tests/run_tests.sh` — suite exit 0. **Nothing ships red.**
T3 was converted per WP-P1d step 0: it asserts the STATE of the evidence
(`mapping_status: UNVERIFIED`, POSITION-GRADE, no fixture-grade artifact) rather
than failing because a witness has not landed. T5 asserts the D-447 horizon
criterion is the one applied. Both fail on a mutant that moves the grade without
a witness — verified, three assertions fire.

| test | result |
| --- | --- |
| T1 extractor unit, fixture -> record byte-exact | pass |
| T1b 11 negative controls (each V-check fires) | pass |
| T2 every valid record representable on pistol's lattice | pass |
| T4 determinism, two cached runs byte-identical | pass (`56a632e3c830403b`) |
| T4b placement distance via `PlacementDistances` (D-451) | pass |
| T3 status UNVERIFIED, nothing claims fixture-grade | pass |
| T5 the D-447 horizon criterion is applied | pass |

T2 is the representability precondition (i16 lattice per D-34, no cell twice),
not a Rust `Board::from` test: wiring an unverified corpus into pistol-core's
test tree is both NON-SCOPE and what step 5 forbids. It is the one deliverable
reduced by the gate, and it is named here rather than quietly dropped.

Re-runs are fully offline from `local/puzzle_cache/` (gitignored via `/local/*`);
network fetch is rate-limited to 1 req/s and each page has a sidecar carrying
`fetched_at`, so a cached re-run reproduces byte-identical output.

## What this package does NOT establish

No mate is proven. No claim is verified — all 8 carry `claim_status:
"UNVERIFIED"`, 4 of them cannot even be attributed to a player (F-3), and there
is no verdict field in v1 by design. No difficulty is measured. No solution is
shown to be unique, or to exist. The coordinate mapping is established only up
to a lattice symmetry, so no position here may be cross-referenced against a
named human game. Nothing here is a strength claim of any kind, about pistol or
about any player, and no position is fixture-grade.
