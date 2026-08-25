# RE-REVIEW (SCOPED) — WP-1.7 SPRT pre-registration, revision 2 (`docs/experiments/wp17_sprt_prereg.md`)

**Revision reviewed: `64f6a05221cfee2aec7efc0db8452f033010d316`. HEAD matches: YES**
(`git rev-parse HEAD` → `64f6a05…`; the document was read at HEAD, not `git show`).

Scope: revision 1 (`be5cbdb`) failed with 0 BLOCKING / 2 MAJOR / 4 MINOR (report at
`/tmp/opencode/wp17_prereg_REVIEW.md`, read first). This re-review verifies each of the
six closures, checks the amendment broke nothing else in the sections it touched
(header, §5, §7, §7A.1, §8.1, §11 — per `git diff be5cbdb..64f6a05`, which touches only
this document and decisions.md), and flags only NEW defects in revision 2's own text.
Sections revision 2 did not touch are not re-litigated. Reviewer: fresh context,
authored nothing reviewed here. Every receipt below was executed, not read. Scratch
under `/tmp/opencode/wp17rev2/`; the repository was not modified.

---

## 1. Receipts executed

| Claim (site) | Test | Result |
|---|---|---|
| HEAD = the named revision | `git rev-parse HEAD` | `64f6a05…` ✓ |
| Amendment scope | `git diff --stat be5cbdb..64f6a05` | only `docs/experiments/wp17_sprt_prereg.md` + `docs/decisions.md` (D-432) ✓ |
| §5 n/2 discrimination is executable off the report's own bytes | the dry-run report's `engine a`/`engine b` lines carry `config <path> config_sha256 <digest>` per seat; `sha256sum configs/instrument_staged_h_v0.toml configs/instrument_staged_v0.toml` | `62fdd05e…` and `43effa06…` — **both exact matches** to the report's per-seat `config_sha256` ✓ |
| §8.1 divergence story | re-ran `python3 tools/wp16_warm_attribution_check.py artifacts/wp17_dryrun_swapped.txt artifacts/wp17_dryrun_swapped_replay.txt target/release/pistol` | **exit 3**, message reproduced character-exact: "DETERMINISM VIOLATION: game 5 turn 14: the report records `-2,-1/5,-1`; the credited seat `staged` … answers `4,-1/5,-1`, and the other seat, asked cold … answers `4,-1/5,-1`" ✓ |
| Binaries still the pinned ones | `sha256sum target/release/pistol target/release/arena` | `665d2815…` / `3e5c114f…` — both exact ✓ |
| Checker's game numbering and scan order | read `classify()`/`violation()`/`read_replay()` in `tools/wp16_warm_attribution_check.py` | `divergence N` is bound to game N (the report's 0-indexed `game N`); the scan probes divergences in document order and `violation()` calls `leave(3)` **immediately** — records after the first failure are never probed ✓ |
| Which record the exit-3 names | matched the message's data against the replay artifact's records | the message's recorded move `-2,-1/5,-1` / credited seat `staged` / warm answer `4,-1/5,-1` matches **divergence record 5** (game 5, 0-indexed) exactly; record 4 (game 4, 0-indexed) carries the anti-symmetric data and is NOT what the message quotes ✓ |
| Record 4 is a CONFIRMED inversion | independent cold probes at the shared 13-turn prefix of games 4/5: `staged` and `staged_h`, `go nodes 50000` | **both answer `4,-1/5,-1`** — cold `staged` reproduces game 4's recorded move `4,-1/5,-1` (confirmed inversion); game 5's recorded `-2,-1/5,-1` matches neither probe (the unconfirmable one) ✓ |
| §7 cost arithmetic | recomputed | `707 × 1000/450 = 1571 s ≈ 26.2 min`; `17.4 × 1000/8 = 2175 s ≈ 36.25 min`; `707 s ≈ 11.8 min ≈ "~12 min"` ✓ |
| §7 openings-length claim | counted moves per opening in both fixtures | `openings_v1.txt`: **all 1591 openings are 4 turns (7 stones)**; `random_openings_v1.txt`: **all 2000 openings are 3 turns (5 stones)** — see NEW MINOR C ✗ |
| D-432 quotations | diffed against the prereg's §2 quote and the header/provenance quote | delegation sentence recorded **verbatim**; fixity sentence recorded verbatim with §2's quote as its exact tail ✓; D-382/D-378/D-374 all exist and D-432's characterization of the D-382 pattern matches D-382's own record ✓ |
| Arena verdict vocabulary (totality check) | read `Verdict` in `crates/pistol-arena/src/sprt.rs` and `verdict()`/`is_degenerate()` in `score.rs` | closed set {h0, h1, inconclusive_at_game_cap, inconclusive_degenerate, invalid_forfeit}; degenerate ⟺ `var ≤ 0` (one occupied pentanomial bucket) ✓ |
| WP-1.6 §5 rows (imports) | read WP-1.6 §5 in full | cap row, aborted row, no-report row, exit 1/2/3 rows, termination row, disagreement row, both degenerate rows — all as the imports describe ✓ |
| WP-1.6 §7A.1 registrations | read WP-1.6 §7A.1 (WARM DRIVE, report WRITER, disagreement consequence) | WP-1.7's new closing sentence names the three identifiers and restates none of the registered content ✓ |

---

## 2. Per-finding closure verdicts

### MAJOR 1 (verdict space not total: cap-inconclusive, aborted, no-report) — **CLOSED**

- All three outcomes now reach a registered consequence: the cap row is new; aborted and
  no-report are explicitly routed by the rewritten last row ("WP-1.6 §5's rows govern,
  imported by reference"), and WP-1.6 §5 does have a row for each (verified: "No verdict
  exists. The games are a diagnostic…"; "exit 2 and NO REPORT AT ALL. Re-record the
  digest and re-launch; nothing has been measured"). Exit 1/2/3, termination and
  disagreement all resolve through the same pointer. The arena's verdict vocabulary is
  the closed five-token enum, and every token has a row.
- The cap row's WP-1.7-specific consequence is consistent with the `h0` row and §6: `h0`
  closes the WP as a measured finding (§6); cap-inconclusive explicitly does NOT close
  as a measured null, and what happens next is pinned — before the run — to "the
  architect's call under a NEW pre-registration, never a re-reading of this one". That
  is exactly the consequence-with-registered-consequence shape the dead zone demanded;
  no after-the-numbers choice survives in it.
- No row contradicts the imports: the cap row's additions elaborate WP-1.6's "No
  action" (no action under THIS document; anything further is a new pre-registration);
  the n/2 row's MISMATCH branch preserves WP-1.6's n/2 consequence verbatim in substance
  and its MATCH branch is a declared replacement under the intro's "rows THIS WP
  replaces or adds" framing — legitimate, since the two matchups' seats differ in kind.
- **Caveat that does not reopen this finding**: the header's and §11's claim that the
  verdict space is thereby "made TOTAL" is false — the amendment's own degenerate keying
  opens a different hole. That is NEW MAJOR A below, not a failure of the three named
  closures.

### MAJOR 2 (the degenerate rows) — **CLOSED**, with a new hole filed separately

- The `distinct_n == n/2` row's discrimination **works as stated**. The report carries,
  per seat, both the config path and `config_sha256`; the digests of the two named
  configs match the dry-run report's fields exactly (receipt above), so the check is
  executable from the report's own bytes plus the pinned repo files, is purely
  mechanical (no strength number is read), and is registered before the run. It
  separates exactly what the row claims: a seat spawned with the wrong document records
  the wrong document's digest → MISMATCH → instrument investigation, never the engine;
  a MATCH on both seats, given §1's three-key diff and §10's flip conditions, means the
  gates never changed a chosen move → the h0 row. The routing to h0 is consistent with
  the h0 row and §6.
- The `distinct_n == n` row's pentanomial reading is **consistent with WP-1.6 §5's
  first degenerate row**, read in full: k = 3 or 4 → the H1 row subject to the same
  floor; k = 0, 1 or 2 → the h0 row; `llr_pair last none`, so no crossing is quoted —
  same routing, same floor, same LLR caveat. (Observation, not a finding: the
  parenthetical compresses WP-1.6's k = 2 branch and drops its "report `capped_fraction`
  beside `k`" requirement; the row's operative instruction is "exactly as WP-1.6 §5's
  first degenerate row prescribes", which carries the full prescription.)
- **But the keyed split itself is defective** — see NEW MAJOR A. The closure of the two
  named findings (misrouting; live case needing its own registered consequence) is real;
  the implementation of the split re-opens the verdict space one row over.

### MINOR 1 (cost) — **CLOSED**, with a new false claim filed separately

- The arithmetic is sound on the document's own measurements: 26 min, 36 min and ~12 min
  all recompute exactly (receipt above). The fixed-depth-node-ratio vs
  fixed-nodes-time-tax correction is stated in terms ("WP-1.6's 2.48x figure was a
  fixed-DEPTH node ratio, not a fixed-nodes time tax"), and the seats-within-~6% pricing
  matches D-431's measured 1.059/1.064 nps ratios.
- The replacement text adds one new false checkable claim (the openings-length
  justification) — NEW MINOR C below. The anchors stand; the sentence explaining why
  they bracket does not.

### MINOR 2 (the mechanism sentence) — **NOT CLOSED**

The artifact's ground truth, pinned three independent ways (the replay document's
records, the checker's own exit-3 message re-executed, and my cold probes):

| record | game (0-indexed) | game (1-indexed) | turn | status |
|---|---|---|---|---|
| 0 | 0 | 1 | 5 | confirmed inversion |
| 1 | 1 | 2 | 5 | confirmed inversion |
| 2 | 2 | 3 | 11 | confirmed inversion |
| 3 | 3 | 4 | 11 | confirmed inversion |
| 4 | 4 | 5 | 14 | **confirmed inversion** (cold `staged` answers `4,-1/5,-1` = its recorded move — verified by probe) |
| 5 | 5 | 6 | 14 | **first unconfirmable** — the checker's exit-3 finding |
| 6 | 6 | 7 | 11 | never probed (`violation()` exits the scan) |
| 7 | 7 | 8 | 11 | never probed |

So yes: the divergence turns are 5, 5, 11, 11, 14, 14, 11, 11 in record order, game 1
(1-indexed) turn 5 is the first divergence, and "game 5 turn 14" is the first
unconfirmable one **only in the checker's 0-indexing — i.e. the sixth game**. The
corrected sentence does not survive this table under any single convention:

- Read 1-indexed (its natural prose reading, and the one the header's "the first
  divergence was game 1 turn 5" commits it to): "turn 11 in games 3, 4, 6-8" is **false
  for game 6** (turn 14 — it is THE unconfirmable one, listed among the confirmable
  inversions); "the scan stops at … game 5, turn 14" names a game whose divergence **is
  confirmed** (receipt above); the probe detail it then quotes ("both probes answer
  `4,-1/5,-1` where the report records `-2,-1/5,-1`") belongs to game 6, not game 5;
  game 2 (turn 5, confirmed) is omitted entirely; and games 7-8 are asserted
  "dual-probe-confirmable" when the scan never probed them — the checker exits at the
  first failure, so their confirmability is unknown, and a document may not register an
  untested probe result as fact.
- Read 0-indexed: "game 5 turn 14" is right, but then "game 1 turn 5" is not the first
  divergence (game 0 is), game 4 is at turn 14 not 11, and "games 6-8" names a game that
  does not exist (games are 0-7).

The sentence mixes 1-indexed enumeration with the checker's 0-indexed finding, and is
false under either convention alone — the same defect class the finding named (a
mechanism sentence the artifact falsifies), now with more false specifics than the
sentence it replaced. The load-bearing core (the scan stops at the first unconfirmable
divergence; cold ≠ warm by turn 14, D-383; the report is still refused) remains true and
W-2′'s registered criterion is untouched, so this stays MINOR — but the closure claim is
false. Recorded for the fix: the errors trace to revision 1's REVIEW's own MINOR 2 gloss,
which used 1-indexed prose ("games 3, 4, 6, 7, 8 diverge at turn 11" — wrong for game 6)
alongside the checker's 0-indexed "game 5", and counted the confirmed records as "0-3"
when the scan's reaching record 5 proves records 0-4 all confirmed. Revision 2 codified
that mixture. The fix must be written off the artifact, not off the review's gloss.

### MINOR 3 (dispatch quotations) — **CLOSED**

D-432 records both sentences: the delegation sentence character-for-character as the
header and provenance quote it, and the fixity sentence with §2's quotation as its exact
tail ("Registered numbers never move post-hoc (D-374). elo0/elo1, budget, book stay as
registered in WP-1.6; not re-read"). This is the D-382 pattern the finding demanded
(D-382's own record, verified, is the precedent D-432 cites accurately via D-378), and
D-432 registers the flip condition — the operator's disavowal reopens §2/§3 by
amendment. The dispatch itself remains operator-checkable only, which is inherent to the
pattern and now logged where the log's own discipline governs it.

### MINOR 4 (false "pointed at, not restated" claims) — **CLOSED**, with a same-shape recurrence filed separately

Every claim of the class was enumerated and tested:

| Claim (line) | True? |
|---|---|
| §2: pair-floor derivation "not restated here" | ✓ no derivation in the document (values only) |
| §4: field list "not restated here" | ✓ (rev-1 text, untouched) |
| §5 intro: exit taxonomy "imported by reference and not restated here" | ✓ the exit-code MEANINGS are absent; §7A.1 quotes the four constants, the idiom rev 1's review accepted |
| §5 intro: cap/aborted/no-report rows "imported the same way" | **✗ for the cap row** — NEW MINOR B |
| §5 last row: "imported by reference and not restated here" | ✓ none of those rows' content appears |
| §7A.1: exit meanings "imported by reference and not restated here" | ✓ |
| §7A.1 closing: three registrations "imported by reference … does not restate them" | ✓ verified against WP-1.6 §7A.1 — only the identifiers (WARM DRIVE, report WRITER) are named, as a pointer must; the registered content is absent |
| §8.1: exit 3's consequence "WP-1.6 §5's row, imported by reference" | ✓ the old restating parenthetical ("hard stop, bigger than the WP, investigate") is deleted |

The three sites revision 1's review flagged are genuinely fixed. The shape recurs once
in new text — NEW MINOR B.

### §11 review-state table (asked explicitly)

Revision 1's outcome is described **accurately**: 0 BLOCKING / 2 MAJOR / 4 MINOR, the
report path, "every receipt it tested held … mechanism story verified true", and
characterizations of both MAJORs and all four MINORs that match the report's own
wording. The revision-2 row's claims are the ones this re-review judges; its
"§5 is total" is falsified by NEW MAJOR A and its "the mechanism sentence is corrected"
by MINOR 2 above.

### Header's account of revision 2 (asked explicitly; D-419-MINOR-G)

**Incomplete**: revision 2 touched the header, §5, §7, §7A.1, §8.1 **and §11** (the
review-state table — rewritten from revision 1's single paragraph). The header names §5,
§7, §8.1, §7A.1/§8.1 and the ADR log, then asserts "Sections not named here are
revision 1's, unchanged" — false for §11. NEW MINOR D.

---

## 3. NEW defects found in revision 2's own text

### NEW MAJOR A — the degenerate rows' `distinct_n` keying does not exhaust the degenerate verdict's reachable space; the "verdict space is made TOTAL" claim (header, §11) is false

The two rows are keyed `distinct_n == n` and `distinct_n == n/2`. The degenerate verdict
fires when the pentanomial variance is zero — one occupied bucket (`score.rs`:
`is_degenerate` = `n == 0 || var <= 0.0`). By the inert-pair theorem (the checker's own
clause-b premise, `tools/wp16_warm_attribution_check.py`: identical transcripts within a
pair ⟹ the same player index wins both games ⟹ the seats split ⟹ the pair is p2), a
pair outside bucket 2 cannot be internally identical; so k ∈ {0, 1, 3, 4} forces
`distinct_n == n` — covered. But **k = 2 admits any mix**: all pairs drawn, some
internally identical and some not, gives `n/2 < distinct_n < n`, and that outcome
matches **neither key**. No other row reaches it (the intro imports only cap/aborted/
no-report; the h0 row requires `verdict h0`).

- **Reachable, and the likely degenerate shape for THIS matchup**: WP-1.6's own first
  degenerate row calls k = 2 "the modal outcome for two closely matched deterministic
  engines"; this matchup is the closest pair of seats the project has run; the dry run
  measured the two seats agreeing on 14 of 16 replayed turns; §6's redundancy
  expectation predicts many internally-identical pairs — a partial mix is at least as
  likely as the pure all-identical signature the new n/2 row prepares for.
- **A regression against revision 1**: revision 1's single unkeyed row routed every
  degenerate verdict to the pentanomial reading — correct for the interior case
  (k = 2 → h0). Revision 2's keyed split dropped it. The hole is introduced by the
  amendment's own text, in the outcome region §6 predicts — the exact class revision 1's
  MAJOR 1 failed on ("the after-the-numbers decision [the document] exists to forbid"):
  an interior-degenerate reader must choose, after the numbers, between the n-row's
  reading (key doesn't match), the n/2 row's two branches (key doesn't match), and "no
  row applies".
- **Fix (one row, before the run)**: key on the signature that discriminates — e.g.
  "`distinct_n == n/2`" (the n/2 row, unchanged) vs "`inconclusive_degenerate`
  otherwise" (the pentanomial reading exactly as WP-1.6 §5's first degenerate row
  prescribes) — or add an explicit interior row routing k = 2 to the h0 row. The
  header's and §11's "TOTAL"/"§5 is total" claims must be true when they are next
  printed, which today they are not.

### NEW MINOR B — the cap-inconclusive row restates WP-1.6-owned content while the intro claims it is "imported the same way" (not restated): the D-419-MINOR-F shape, reintroduced by the text that closed it

§5's intro: "WP-1.6 §5's rows for `inconclusive_at_game_cap`, … are imported the same
way" — the same way being "imported by reference and not restated here". The cap row
then opens "Imported from WP-1.6 §5 by reference" and restates WP-1.6's row's reporting
content verbatim — "the sample is reported with its LLR and its distance from both
bounds" is WP-1.6's own sentence. The row's WP-1.7-specific additions (does not close as
a measured null; architect's call under a new pre-registration) are legitimate and
needed; the restated clause is not, under a sentence that says there is no copy. This is
precisely the second-site-fix mechanism D-423 codified the state-it-once rule to
abolish: an amendment to WP-1.6's cap row must now be fixed here too, and the site that
gets missed is the one the document says does not exist. Fix: delete the restated clause
(the row's own additions do not need it) or drop the not-restated claim for this row.

### NEW MINOR C — §7's bracket justification is a false checkable claim about the repo's own fixtures (D-291 shape)

"the dry run's openings are shorter than the book's five-stone openings, so the two
bracket it from below and above" — measured: **all 1591** `openings_v1.txt` openings are
4 turns (7 stones); **all 2000** `random_openings_v1.txt` openings are 3 turns (5
stones). The dry-run openings are **longer**, not shorter. The arena derives
`opening_turns` from the book (`openings.rs`, uniform turn count, validated), so the
governed run plays 37 searched turns to the dry run's 36 — the dry-run anchor
marginally *understates* per-game cost, the opposite of the stated bracket direction.
The three anchors (12 / 26 / 36 min) are honestly derived and correctly marked
ESTIMATED; the sentence explaining why they bracket is false, and it was checkable in
seconds against files the document already pins — the exact shape the document's own
history (D-291) polices. Fix: delete the justification clause, or state the honest one
(the two anchors differ ~40% for reasons this document does not model; the cap figure
lies in the range they span).

### NEW MINOR D — the header's account of revision 2 omits §11 while asserting completeness (the D-419-MINOR-G lesson, cited by this very header's process)

"Sections not named here are revision 1's, unchanged" — §11 was rewritten by this
revision and is not named. One-clause fix ("…and §11 records the review state").

---

## 4. Observations (not counted as findings)

- The n-row's parenthetical drops WP-1.6's k = 2 requirement to "report
  `capped_fraction` beside `k`" (WP-1.6 flags all-capped and closely-matched as "two
  causes [that] are different findings about the same number"). The operative
  instruction is "exactly as WP-1.6 §5's first degenerate row prescribes", which carries
  it; noted because the same k = 2 region is where NEW MAJOR A's hole sits, so the fix
  round will touch this text anyway.
- The retained Revision-1 block's "this revision governs no run until a fresh-context
  review passes it" is now historically stale (revision 1's review failed; it can never
  govern), but §11's table states the actual state unambiguously.
- §8.1's new looseness registration ("any non-zero exit with a named finding satisfies
  W-2′ … an exit 2 would itself be an instrument anomaly worth investigating") closes
  the loosening the revision-1 review noted as "the document's, not the reviewer's, to
  have said so". Its premise claim (no refusal can fire under label transposition)
  checks out against the checker's premise tests.
- MINOR 2's false specifics are inherited from the revision-1 review's own MINOR 2 gloss
  (mixed indexing; "records 0-3" for what the scan proves to be records 0-4). The
  document's fix must be derived from the artifact, not from the review's paraphrase of
  it — recorded so the next round does not re-import the same error.

---

## Verdict

**FAIL — 0 BLOCKING, 1 MAJOR (new), 3 MINOR (new), and 1 of the original six findings
not closed (MINOR 2).**

The two MAJOR closures are real in their named content: the three orphaned outcomes
reached registered consequences, the n/2 discrimination is executable, pre-registered
and verified against the report's own bytes, D-432 is the D-382 pattern done properly,
and the three false pointer claims are gone. What fails: the amendment's own keying
re-opens the verdict space one row over (NEW MAJOR A — an outcome that is not merely
reachable but the most likely degenerate shape for this closest-ever seat pair now
matches no row, while the header claims totality); the mechanism sentence was
"corrected" into a new set of false claims about the artifact (MINOR 2 not closed —
mixed indexing, game 6 mislisted at turn 11, a confirmed inversion named as the
unconfirmable one, unprobed records asserted confirmable); and three smaller
same-class-shape defects (false not-restated claim at the cap row; a backwards
openings-length "fact" checkable in seconds; the header's incomplete account of its own
diff). All are one-row or one-clause fixes, and none may be made after game one — which
is why this re-review fails rather than defers.
