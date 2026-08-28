# WP-P1b — corpus symmetry census, then witness retry

Revision: working tree over `a14912a`; step-1 review pin `b798db18`.
Human corpus `b2fe61eb…` verified before every read and **never modified**
(sha identical after the run).

**Step 1 HARD STOP: does NOT fire.** Steps 4 and 5 complete. **Steps 2 and 3
are BLOCKED** — see below; nothing was faked around the block.

## Step 1 — corpus symmetry census

`crates/pistol-cli/src/corpus/census.rs` + `bin/corpus-census.rs`. Rule 2 is
clean: the group, `transform`, `canonical_form` and `canonical_sequence` are
pistol-core's; the turn grouping is `replay::group_turns`. `Symmetry::ALL`
already exposes all twelve elements, so nothing needed extending.

    $ corpus-census --corpus <corpus> --expect-sha b2fe61eb…
      games read              8698
      games the turn grouping refused  0

    key                                          classes  games  identity  non-trivial
    canonical sequence (whole game)                    0      0         -            -
    canonical final position                           0      0         -            -
    canonical final position, colour discarded         0      0         -            -
    canonical early position (turn 6, 11 stones)     212    543        74          138
    canonical early position, colour discarded       360   1005        73          287

**Hypothesis verdict.** (a) exact-duplicate ingestion: EXCLUDED — the reader
refuses a repeated `game_hash` and never fired, and the corpus's own
`n_duplicates_dropped` is 0. (b) inconsistent orientation normalization:
EXCLUDED — 0 sequence classes, from an invariant demonstrated live (below).
(c) genuine line repetition: the surviving explanation, and the collisions are
openings that provably diverge (0 sequence AND 0 final-position classes means
every colliding pair differs later and ends on a different board).

**The detector is not a null instrument.** Injecting a real duplicate of
`520a38b9d548f723` under `refl-rot3` into a COPY of the corpus produces
`size 2  identity-equal false  elements [refl-rot3]`, fires the HARD STOP and
exits 1. Rotations, both reflection forms, within-turn reorder and exact
duplicates are all caught.

**The collision profile over depth — the result that actually licenses the
leakage claim.** The first draft measured ONE cut and said "only 11-stone
openings collide". That was never measured and is false. `--early-turns` now
makes the cut a flag, and the sweep is:

    turn   stones   coloured cls/games   uncoloured cls/games
    2       3        179/8569             179/8569
    3       5        969/6452             909/6916
    4       7        885/3642             901/4374
    5       9        499/1418             634/2217
    6      11        212/543              360/1005
    7      13         64/150              141/340
    9      17         12/27                22/48
   11      21          4/9                  5/11
   13      25          1/2                  1/2
   14      27          0/0                  0/0
   15+     29+         0/0                  0/0

Collisions exist at every prefix from turn 2 to turn 13, decay monotonically,
and **vanish from turn 14 on: the deepest position any two corpus games share
is 25 stones, shared by exactly two games.** That bound, not the single cut, is
what says a Stage-2 teacher/holdout split is not sharing positions of
consequence. Named limit: the key is binary (same-or-not) and says nothing
about NEAR-duplicates, so 1005 games sharing an 11-stone opening remain
correlated across a split even though none is a duplicate.

**Named residual.** "Distinct players" — the third conjunct of genuine line
repetition — is NOT establishable: the corpus carries `game_hash`, `moves`,
`winner`, `source`, `elo` and nothing else. No names, no timestamps, no game
ids. The negative claims (not (a), not (b)) are fully supported; the positive
label "genuine line repetition" is supported modulo that unverifiable conjunct.

## Reviews, and what they found

Both fresh-context reviews ran against `b798db18` and **independently found the
same MAJOR defect**, which is now fixed:

- **The Sequence census described its classes with BOARDS, not sequences.** Two
  games whose boards coincide under a symmetry but whose move orders differ were
  reported `identity-equal true` — asserting an exact duplicate where the truth
  is an orientation duplicate, which INVERTS the (a)-versus-(b) discrimination
  this census exists to make, on the one key that drives the HARD STOP. Zero
  effect on this corpus (0 sequence classes), wrong on the escalation path.
  `describe` is now generic over the image function, so every census compares
  the shape its key was built from. This was the third instance of one bug; the
  genericity removes the shape in which it recurs.
- **`--expect-sha` silently stopped gating when given twice** (`if let Ok(..)`
  swallowed `flags::one`'s "given more than once"). Rule 3, on the gate whose
  whole purpose is that a census cannot be read as a measurement of a document
  it was not taken over. Fixed with a shared `flags::optional`.
- **The exit code spoke for two of five censuses** and `report_classes` omitted
  the uncoloured final-position kind — the one that would catch a colour-swapped
  whole-game duplicate. Both now cover all five, and USAGE says what the code
  means.
- **A schema-valid game could panic** (`Symmetry::apply` overflows on a stone
  the i16 reader accepts). Now a named refusal, `OFF_ADDRESSABLE_LATTICE`.
  Unreachable on this corpus (max |q| 135, |r| 150).
- **Six surviving mutants**, including `owner_of` always P1, the final-position
  key not canonicalized, and the ENTIRE rendered `Display` block — the block
  every reported number was transcribed from. That is verbatim the gap D-219
  already paid for in this module. Tests added; **all nine mutants now die**
  (six from the review, the MAJOR-1 regression, and both descriptive ones).
  The census tests went 6 -> 12 and every number above is now bound.

Re-run after every fix: **the numbers did not move.** They are now bound.

## Step 4 — colour mapping: RESOLVED

Source: `https://hexo.did.science/assets/chunk-BvUA9bc6.js`, sha256
`c56bc5ac…`, app build `5d2fce4`. The chain, exact fields:

    zm = ['#fbbf24','#38bdf8','#f472b6','#34d399','#c084fc','#fb7185']
    Vm(seat)   = zm[Math.min(seat, zm.length-1)]
    Hm(ids)    = Object.fromEntries(ids.map((id,i) => [id, {color: Vm(i)}]))
    uh(...)    { e.playerTiles = Hm(t) }              // t = the player-id array
    h8         = [{id:'sandbox-player-1',…}, {id:'sandbox-player-2',…}]
    b8('player-1') = h8[0].id                          // seat 0

**player-1 -> `#fbbf24` (amber/yellow); player-2 -> `#38bdf8` (sky blue).**
Colour is assigned by seat index; no per-player override exists.

Independent corroboration, not part of the derivation: all **4 of 4** colour-
naming claims agree with side-to-move under this mapping (`forced win for
yellow` at `to_move=1`; `yellow forced win` at 1; `blue forced win` at 2; `win
for blue` at 2). Under the opposite mapping all four would assert a forced win
for the side NOT to move.

The same chunk also confirms F-2 at source level:
`player: t.playerId === e.players[0].playerId ? 'player-1' : 'player-2'` — the
label is a seat index into `players[]`, and the opener is derived separately
from `moves[0].playerId`. WP-P2 may attribute the 4 colour claims; it still may
not score them.

Caveat: this is a client build artifact whose filename carries a build hash. The
mapping is pinned to build `5d2fce4` and is not a platform contract.

## Step 5 — D-440 consolidation: DONE

`PlacementDistances` (`corpus/distance.rs`) was already the one implementation
and was already correct — it measures `moves[..index]`, which includes
turn-mates. WP-P1's F-4 artifact was in a throwaway analysis script, not in
shipped code. `tools/puzzle_corpus` owns no distance code and now CALLS the
shipped one: `crates/pistol-cli/tests/puzzle_distance_tests.rs` is run by
`tools/puzzle_corpus/tests/run_tests.sh`, and it pins the two criteria apart —

    shipped (turn-mate inclusive):  MAX 8, 0 beyond LEGAL_RADIUS
    pre-turn-board variant:         MAX 9, 12 beyond LEGAL_RADIUS

so the artifact is reproduced in a test that fails if the shipped criterion ever
drifts toward it. The puzzle corpus is SHA-pinned inside that test (D-37).

## Step 3 — witness retry: the ambiguity is RESOLVED, the gate still fails

Selection rule, **pre-registered before measuring** and printed by the gate:
replay records, trivial stabilizer first, then highest stone count, then id.
All 17 have trivial stabilizers; the witnesses are **o9tde3t (n=267)** and
**5h9dau9 (n=149)**. `n2sqzrf` — the only record that matches anything today —
ranks **last** of 17, so the rule cannot be accused of fitting.

The WP-P1 three-way ambiguity was **entirely an artifact of discarding colour**.
The census showed the three candidate games share ONE uncoloured early canonical
form and THREE distinct coloured ones. Re-running the witness leg with colour
(roles, not labels — F-2) is decisive:

    n2sqzrf  n=11  UNAMBIGUOUS
        uncoloured rot0         [('917f33efd73933bb', 29)]  <- move count corroborates  COLOURED TOO
        uncoloured rot240+refl  [('149c4d508cdbef0b', 95)]
        uncoloured rot300       [('eed4639fe21a2af7', 239)]

Coloured, only `rot0` survives, on one game. This is a stronger test, not a
looser one: it could have selected `rot300` and killed identity. It did not.

**The gate stays shut, and D-447 says why more precisely than "two required".**
The one unambiguous witness sits at 11 stones (turn 6), below the measured
turn-14 sharing horizon, where a lone match is worth nothing because 212 classes
of games share an 11-stone opening. At or above the horizon one witness would
close it. Residual: identity, on one below-horizon witness.

## Steps 2 and 3 — BLOCKED, and why

Step 2 (snapshot refresh) cannot be done, on two independent grounds:

1. **hexo-bridge has no match-history surface.** Its endpoints are
   `/api/stream/event`, `/api/bot/games` (ACTIVE games), `/api/bot/status`,
   `/api/challenge/*`, `/api/challenges`, `/api/account`, `/api/token`
   (`src/hexo_bridge/adapters/platforms/hexo.py`). There is no export, archive
   or history route. Downloading historical games is a hexo-bridge FEATURE that
   does not exist, not a call this package can make.
2. **No credential.** `HEXO_BRIDGE_TOKEN` is unset and no bridge config exists
   on this machine.

Writing a history client into pistol is refused: D-441 puts the fetcher in
hexo-bridge the moment it needs a credential, and this needs one. Step 3
therefore cannot reach its second witness. The pre-registration above is
recorded so that when a snapshot does land, the witnesses are already chosen.

## ADR NUMBERING COLLISION — needs the architect

The dispatch's D-437 … D-441 **collide with existing append-only ADR lines**:

    D-437: WP-1.8a LANDS: THE POLICY-GAME df-pn SOLVER WITH FOUR ORACLE GATES…
    D-438: THE SEALBOT ANCHOR IS TAKEN, AND IT IS 0 FOR 40…
    D-439: The instrument node budget stays at 50 000 per side…
    D-441 / D-442 / D-443: also present

The log already runs to D-443, so the next free number is **D-444**. I have not
renumbered them and have not appended them — that is the architect's act on an
append-only log. Code comments therefore cite findings by NAME
("WP-P1b REVIEW-impl F1", "WP-P1 F-2") rather than by a number that currently
means something else.

## What is still NOT established

The coordinate mapping is still UNVERIFIED under the dispatch's own rule: one
unambiguous witness against a required two, so the corpus stays POSITION-GRADE
and is not fixture-grade. No mate is proven, no claim is verified, no difficulty
is measured, and the 4 colour claims are now attributable but still unscored.
The corpus census excludes duplicate GAMES but says nothing about near-duplicate
openings, which remain correlated across a split. "Distinct players" is not
checkable from this corpus at all. Nothing here is a strength claim.
