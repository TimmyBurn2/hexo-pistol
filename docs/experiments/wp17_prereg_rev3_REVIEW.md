# RE-REVIEW (SCOPED) — WP-1.7 SPRT pre-registration, revision 3 (`docs/experiments/wp17_sprt_prereg.md`)

**Revision reviewed: `17b2ff203dd0e17cc22dff8c164134cca9d50649`. HEAD matches: YES**
(`git rev-parse HEAD` → `17b2ff2…`; the document was read at HEAD, not `git show`).

Scope: revision 2 (`64f6a05`) failed its scoped re-review with 1 new MAJOR, 3 new
MINOR and 1 not-closed finding (report at `/tmp/opencode/wp17_prereg_rev2_REVIEW.md`,
read first). This re-review verifies each of the five closures against the
repository's own bytes — never against a review's paraphrase — checks the
amendment introduced nothing new in the sections it touched (header, §5, §7, §8.1,
§11 — per `git diff 64f6a05..17b2ff0`, which touches only this document), and stops
there. Reviewer: fresh context, authored nothing reviewed here. Every receipt below
was executed, not read. Scratch under `/tmp/opencode/wp17rev3/`; the repository was
not modified.

---

## 1. Receipts executed

| Claim (site) | Test | Result |
|---|---|---|
| HEAD = the named revision | `git rev-parse HEAD` | `17b2ff2…` ✓ |
| Amendment scope | `git show --stat 17b2ff2` | only `docs/experiments/wp17_sprt_prereg.md` (38+/31−) ✓ |
| Degenerate ⟺ one occupied bucket | read `is_degenerate`/`var` in `crates/pistol-arena/src/sprt.rs` and `pair_buckets` in `score.rs` | `is_degenerate = n == 0 \|\| var <= 0.0`; bucket k = pair score k/4, all scores distinct ⇒ var = 0 iff exactly one bucket occupied ✓ |
| Verdict vocabulary is closed | read `enum Verdict` in `sprt.rs` | {H0, H1, InconclusiveAtGameCap, InconclusiveDegenerate, InvalidForfeit} — every token has a §5 row ✓ |
| Inert-pair theorem | read clause (b)'s premise in `tools/wp16_warm_attribution_check.py` (docstring + `clause_b`) | identical transcripts within a pair, neither forfeited ⟹ same player index wins both ⟹ labels swap ⟹ forced 1-1 split (bucket p2); the checker ASSERTS bucket p2 for inert pairs ✓ |
| The forcing claims in the row's parenthetical | derived + MEASURED the one loose premise | k ∈ {0,1,3,4} ⟹ every pair outside bucket 2 ⟹ no pair internally identical (or symmetry-equivalent — a symmetry-image pair also splits, same argument) ⟹ distinct_n == n ✓; k = 2 admits any mix (each bucket-2 pair contributes 1 or 2 distinct games) ✓ |
| distinct_n ∈ [n/2, n] for THIS document's runs (the enumeration's completeness) | canonicalised every opening of both books under the 12 lattice symmetries (`/tmp/opencode/wp17rev3/symcheck*.py`) | `random_openings_v1.txt`: 2000 openings, **2000 distinct canonical forms** (slice 1000..1499: 500/500 distinct, none shared with the rest); `openings_v1.txt`: **1591/1591 distinct** — no two openings are symmetry-equivalent on either book, so cross-pair duplicates are impossible and every duplicate is within-pair ✓ |
| §8.1 mechanism story | re-ran `python3 tools/wp16_warm_attribution_check.py artifacts/wp17_dryrun_swapped.txt artifacts/wp17_dryrun_swapped_replay.txt target/release/pistol` | **exit 3**, message reproduced character-exact: "DETERMINISM VIOLATION: game 5 turn 14: the report records `-2,-1/5,-1`; the credited seat `staged` … answers `4,-1/5,-1`, and the other seat, asked cold … answers `4,-1/5,-1`" ✓ |
| Binary the re-run used | `sha256sum target/release/pistol target/release/arena` | `665d2815…` / `3e5c114f…` — both match §7A.1's pins ✓ |
| The eight divergence records | read `artifacts/wp17_dryrun_swapped_replay.txt` in full | records 0-7 at games 0-7 in document order, turns 5, 5, 11, 11, 14, 14, 11, 11; record 5 = game 5 turn 14, mover `staged`, recorded `-2,-1/5,-1`, answered `4,-1/5,-1` — exactly the record the exit-3 message quotes ✓ |
| Probing order and the exit-at-first-failure | read `classify()`/`violation()`/`leave()` in the checker | `classify` iterates `replay["divergences"]` in document order; `violation()` calls `leave(3)` immediately — reaching record 5 proves records 0-4 were probed and confirmed; records 6-7 are never probed ✓ |
| §7 openings facts | counted moves per opening in both fixtures (`openings_v1.txt`, `random_openings_v1.txt`); read `uniform_turn_count` in `crates/pistol-arena/src/openings.rs` | openings_v1: **all 1591 openings are 4 turns (7 stones, corpus-sourced — `# src … elo …` metadata)**; random_openings_v1: **all 2000 are 3 turns (5 stones)**; arena derives and validates a uniform `opening_turns`; the dry-run replay artifact itself prints `opening_turns 4` ✓ |
| §7 arithmetic | recomputed | 707 s ≈ 11.8 min ≈ "~12 min"; 707 × 1000/450 = 1571 s ≈ 26.2 min; 17.4 × 1000/8 = 2175 s ≈ 36.25 min; anchors differ (36.25−26.2)/26.2 = **38.4% ≈ "~40%"** ✓ |
| WP-1.6's cap-row sentence absent | `rg "distance from both bounds\|the sample is reported"` over the WP-1.7 document | no match ✓ (the sentence lives only in WP-1.6 §5, line 417, where it belongs) |
| The "every other case" row's routing | read WP-1.6 §5's first degenerate row in full | k = 3 or 4 → H1 row subject to the 100-pair floor; k = 0, 1 or 2 → h0; `capped_fraction` beside k; `llr_pair last none` — WP-1.7's row states the routing and the caveats correctly, and now names the `capped_fraction`-beside-`k` caveat rev 2's version dropped ✓ |
| §11's revision-3 row and the header's account | ran `git diff 64f6a05..17b2ff0` and enumerated touched sections | header, §5, §7, §8.1, §11 — all five named in the header's own account ✓ |

---

## 2. Per-finding closure verdicts

### NEW MAJOR A (degenerate keying hole) — **CLOSED**

- **The keying is total by construction and the reachable space is exactly as
  described.** The two rows are keyed `distinct_n == n/2` and "in every other case" —
  complementary keys, so every `inconclusive_degenerate` report matches exactly one
  row whatever `distinct_n` is. And for this document's runs the dash-enumeration is
  not merely harmless but exact: MEASURED, no two openings on either book (the
  governed `random_openings_v1` and the disjoint `openings_v1` of the
  below-100-pairs branch) are symmetry-equivalent under the 12 lattice symmetries,
  so a game can only duplicate its own pair's twin and `distinct_n ∈ [n/2, n]` —
  "distinct_n == n, and the interior mixes n/2 < distinct_n < n" is precisely the
  complement of n/2.
- **The parenthetical is true.** k ∈ {0,1,3,4} forces distinct_n == n: a pair outside
  bucket 2 cannot be internally identical — the inert-pair theorem (verified in the
  checker's own clause-(b) premise: identical transcripts ⟹ same player index wins
  both ⟹ labels swap ⟹ forced 1-1 split), and the same argument covers
  symmetry-image pairs, which also split. k = 2 admits any mix: each bucket-2 pair
  (drawn or decided-split) contributes 1 distinct game if identical, 2 if not.
  `is_degenerate` = `var <= 0` ⟺ one occupied pentanomial bucket, so the degenerate
  verdict's whole reachable space is: single occupied bucket k ∈ {0,1,3,4} ⟹
  distinct_n == n (every-other row); single occupied bucket k = 2 ⟹ distinct_n
  anywhere in [n/2, n] (n/2 row at the endpoint, every-other row otherwise). No
  shape falls between keys.
- **The header's totality claim is now true.** "Total for real": the five-token
  `Verdict` enum is fully tabulated (h1 ×2, h0, cap, forfeit, both degenerate rows),
  and the instrument-side outcomes route through the imported WP-1.6 rows. The
  revision-1 MAJOR-1 hole (cap/aborted/no-report), the revision-2 MAJOR-A hole
  (interior mixes) and this round's check all come up empty.
- The row also folds in the rev-2 observation: the `capped_fraction`-beside-`k`
  caveat is now named in the row itself, not only carried by the pointer.

### MINOR 2 (mechanism sentence, was NOT CLOSED) — **CLOSED**

The sentence was checked clause-by-clause against the artifact, the checker's source,
and the re-executed exit-3 message — not against any review's gloss:

- "eight `divergence` records, in document order (games 0-7 of the checker's
  0-indexed vocabulary), sit at turns 5, 5, 11, 11, 14, 14, 11, 11" — verified
  against the replay artifact's records 0-7 ✓.
- "The checker probes them in document order" — `classify()` iterates the records in
  document order ✓.
- "the first five (games 0-4) resolve as CONFIRMED inversions" — the re-run reached
  record 5 (the message names game 5), and `violation()` exits at the first
  unconfirmable record, so reaching record 5 proves records 0-4 were each probed and
  each cold answer matched the recorded move ✓.
- "the SIXTH — its `game 5`, the message's own 0-indexed spelling — is the first the
  dual probe cannot confirm" — the re-run's message names game 5 turn 14; record 5 is
  game 5 turn 14 ✓. One indexing convention (0-indexed) is used throughout: games
  0-7, games 0-4, game 5, games 6-7 — no mixing ✓.
- "both probes answering `4,-1/5,-1` where the report records `-2,-1/5,-1`" — the
  re-run message: credited seat `staged` warm answers `4,-1/5,-1`, the other seat
  cold answers `4,-1/5,-1`, report records `-2,-1/5,-1` ✓.
- "The last two records (games 6-7) are NEVER probed — the instrument exits at the
  first unexplained divergence — so nothing is claimed about their confirmability
  either way" — true of the checker, and the document now claims nothing about them ✓.
- "The corrupted report is still refused and nothing downstream of it is read" ✓.

The sentence survives a reader who checks every clause against the artifact. The
defect that sank revision 2 — deriving the story from the review's gloss, with mixed
indexing and unprobed records asserted confirmable — is gone, and the document's own
account of the fix ("rewritten off `artifacts/wp17_dryrun_swapped_replay.txt` with
the checker's 0-indexed vocabulary stated and the unprobed records left unclaimed")
is accurate.

### NEW MINOR B (cap row restating under a not-restated claim) — **CLOSED**

- The §5 intro no longer claims the cap row is imported: the import list now reads
  "`arena_report_aborted` and a pre-game refusal with NO REPORT AT ALL", with
  `inconclusive_at_game_cap` removed ✓ — consistent with the table's last row, which
  never listed the cap.
- WP-1.6's own sentence ("the sample is reported with its LLR and its distance from
  both bounds") no longer appears anywhere in this document (grep: no match) ✓.
- The cap row now opens "**A WP-1.7 row that ADDS to WP-1.6 §5's cap row, not an
  import of it**", and its remaining content is WP-1.7-specific vocabulary (gates,
  committed configs, the not-a-measured-null distinction, the new-pre-registration
  routing) — none of it restates WP-1.6's "Reported as inconclusive. No action."
  Under a row that declares itself an addition there is no false not-restated claim
  left to make. The D-423 second-site exposure is gone: an amendment to WP-1.6's cap
  row now has exactly one site to fix.

### NEW MINOR C (backwards openings-length fact) — **CLOSED**

- MEASURED: `openings_v1.txt` (the dry run's book) — all 1591 openings are **4 turns
  (7 stones)**, corpus-sourced; `random_openings_v1.txt` (the governed book) — all
  2000 openings are **3 turns (5 stones)**. The new sentence's claims are true: "the
  dry run's book holds 4-turn openings against the governed book's 3-turn" ✓; "its
  openings are not five-stone random samples" ✓ (they are 7-stone corpus openings);
  the header's "one turn LONGER, not shorter" ✓ (the replay artifact itself prints
  `opening_turns 4`).
- The false bracket-direction claim is replaced by the honest form the rev-2 review
  asked for: "The two anchors differ ~40% for reasons this document does not model …
  the cap estimate is read as the RANGE they span, not as either anchor" — the ~40%
  recomputes (38.4%), the anchors recompute, and no direction is asserted. The three
  anchors stay correctly marked ESTIMATED.

### NEW MINOR D (header omitting §11) — **CLOSED**

- The header names §11 — twice: the closure list's "the header names §11" and the
  revision-2 paragraph's "see §11 for the ledger" ✓.
- Every section revision 3's diff actually touches is named in the header's account
  of itself: enumerated from the diff hunks, revision 3 touches the header, §5
  (degenerate rows + cap row), §7 (cost row), §8.1 (mechanism paragraph) and §11
  (ledger rows) — all five are named ✓.
- The flat false assertion of revision 2 ("Sections not named here are revision 1's,
  unchanged") is replaced by "unchanged except as revisions 2 and 3 state", which no
  longer asserts that unnamed sections are untouched. Residual looseness is recorded
  as an observation below, not a finding.

### §11's revision-3 row (asked explicitly) — **ACCURATE, with one word inherited from the defect filed below**

All five clauses of the revision-3 row check out against the diff and the receipts
above: re-keyed rows with the interior explicitly named ✓; mechanism sentence
rewritten off the replay artifact with the 0-indexed vocabulary stated and the
unprobed records left unclaimed ✓; cap row an ADDITION that drops WP-1.6's
sentence ✓; §7's bracket justification corrected ✓; header names §11 ✓. The
revision-2 row's characterization of the rev-2 re-review (0 BLOCKING / 1 new MAJOR /
3 new MINOR / 1 of the six not closed; each finding described) matches the rev-2
report's own verdict and content ✓ — except its opening two words, which are
NEW MINOR E.

---

## 3. NEW defect found in revision 3's own text

### NEW MINOR E — "Revision 2 closed the six findings" is false, stated twice, and contradicted by the ledger it points at (D-423 shape)

Header ¶2: "**Revision 2 closed the six findings of revision 1's review (`be5cbdb`,
0 BLOCKING / 2 MAJOR / 4 MINOR) — see §11 for the ledger.**" §11's revision-2 row
opens "Closed the six above, but **FAILED its scoped re-review — 0 BLOCKING, 1 new
MAJOR, 3 new MINOR, 1 of the six not closed** … Not closed: the mechanism
sentence …". The rev-2 re-review's verdict — the judge — is explicit: "1 of the
original six findings not closed (MINOR 2)". Revision 2 closed five of the six.

- **False, and checkable in seconds** against the very documents this header cites:
  §11's own row says one of the six was not closed, and the header's ¶1 enumerates
  revision 3's five closures as "1 new MAJOR / 3 new MINOR / 1 not-closed" — that
  "1 not-closed" is one of the original six, so the header's two paragraphs are
  mutually inconsistent about whether revision 2 closed all six.
- **Stated twice, disagreeing** — the D-423 shape this project treats as a
  first-class defect class: the state of revision 2 is asserted in the header and in
  §11's row opening, and both sites disagree with the same row's own body and with
  the rev-2 report. The charitable reading ("revision 2 addressed/took on all six")
  is not the document's own usage — "closes the five findings" in ¶1 and "Closes all
  five" in §11's revision-3 row both mean verified closure, and §11's row uses "not
  closed" in exactly that sense two clauses later.
- **What it does not do**: it gates no registered reading, leaves no verdict-space
  hole, and touches no instrument claim — which is why it is MINOR. The fix is two
  words ("closed" → "addressed" or "set out to close", at both sites, with §11's row
  needing only its opening clause); it must land before the governed run, like every
  other pre-registration fix.

---

## 4. Observations (not counted as findings)

- **The header's completeness hedge and §7A.1.** "Sections not named here are
  revision 1's, unchanged except as revisions 2 and 3 state": §7A.1 was amended by
  revision 2 (the "pointed at" → "imported by reference" fixes, per
  `git diff be5cbdb..64f6a05`), is not named in the current header, and the
  document's account of revision 2 reaches that change only through "closed the six
  above" → the rev-1 row's "three false 'pointed at, not restated' claims", without
  naming the section. The hedge clause ("except as revisions 2 and 3 state") exists
  precisely to avoid re-asserting completeness, and both readings send the reader to
  the same place (§11 and the git history), so this is recorded rather than filed.
  Naming §7A.1 in the revision-2 ledger row would close it outright.
- **"The interior shapes are the modal degenerate outcome for two closely matched
  deterministic engines, this matchup above all"** — an unmeasured frequency claim,
  but not measurable in seconds (it is about a future sample), phrased in the same
  idiom WP-1.6's own registered degenerate row uses ("the modal outcome for two
  closely matched deterministic engines"), and it gates nothing: both k = 2 shapes
  route to the same rows under the new keying. Rhetorical emphasis on the operative
  "must not fall between keys", which is true and verified.
- **"By turn 14 the other seat's COLD probe no longer reproduces what the WARM seat
  recorded"** — localized to the sixth record (game 5's turn 14), which is how the
  sentence frames it; record 4 (game 4, turn 14) was confirmed, and the sentence
  does not claim otherwise. The operative data ("both probes answering `4,-1/5,-1`
  where the report records `-2,-1/5,-1`") is spelled out and exact.
- The five closures are all real, verified against the repository's own bytes, and
  the amendment introduced exactly one new defect (MINOR E) — the smallest new-defect
  count of any round so far, in the same bookkeeping genre as the ones before it.

---

## Verdict

**FAIL — 0 BLOCKING, 0 MAJOR, 1 new MINOR (MINOR E); all five findings CLOSED.**

Every closure claimed for revision 3 is real and was verified against the bytes, not
the paraphrase: the degenerate keying is total (complementary keys, the reachable
space enumerated exactly, the forcing claims proven and the one loose premise —
cross-pair symmetry duplicates — measured absent on both books), the mechanism
sentence survives a clause-by-clause check against the artifact and the re-executed
exit-3 message under a single indexing convention, the cap row neither restates
WP-1.6's sentence nor claims it doesn't, the openings facts now run the right way
round (4-turn dry-run book against 3-turn governed book, counted), and the header's
account of its own diff is complete. What fails the revision is one false historical
claim introduced by its own compression of the revision-2 account: "Revision 2
closed the six findings" — contradicted by the ledger it points at, by the rev-2
report's verdict, and by the header's own enumeration of the five closures it
inherits. Two words, two sites, before the run — the same reason every prior round
failed: none of it may be fixed after game one.
